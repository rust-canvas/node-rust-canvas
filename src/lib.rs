extern crate cssparser;
extern crate euclid;
#[macro_use] extern crate neon;
extern crate rustcanvas;

#[macro_use] mod macros;
mod traits;

use std::ops::Deref;

use cssparser::{RGBA, Color, Parser, ParserInput};
use euclid::{Rect, Size2D, Point2D, Transform2D};
use neon::mem::{Handle};
use neon::js::{JsArray, JsFunction, JsObject, JsString, JsNumber, JsBoolean, Object, Value, Variant};
use neon::js::binary::{JsBuffer};
use neon::js::class::{JsClass, Class};
use neon::vm::{Lock, JsResult, This, FunctionCall};
use rustcanvas::{CanvasElement, create_canvas, CanvasContextType, FillOrStrokeStyle, CompositionOrBlending, LineCapStyle, LineJoinStyle, ToAzureStyle};

use traits::*;
use std::str::FromStr;

trait CheckArgument<'a> {
  fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V>;
}

impl<'a, T: This> CheckArgument<'a> for FunctionCall<'a, T> {
  fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V> {
    self.arguments.require(self.scope, i)?.check::<V>()
  }
}

declare_types! {
  pub class Canvas for CanvasElement {
    init(mut call) {
      let width = call
        .check_argument::<JsNumber>(0)
        .expect("Check width error")
        .value() as i32;
      let height = call
        .check_argument::<JsNumber>(1)
        .expect("Check height error")
        .value() as i32;
      let canvas = create_canvas(width, height, CanvasContextType::CTX2D);
      Ok(canvas)
    }

    method toBuffer(mut call) {
      let actions = call
        .check_argument::<JsArray>(0)
        .expect("Check actions error")
        .to_vec(call.scope)
        .expect("Unpack actions error");

      let mut this = call.arguments.this(call.scope);
      let canvas: &mut CanvasElement = this.grab(|c| c );
      let ctx = &mut canvas.ctx;
      actions.iter()
        .map(|v| v.check::<JsObject>().expect("Unpack JsObject Error"))
        .for_each(|v| {
          let action_type = to_str!(call.scope, v, "type");
          match action_type.deref() {
            "ARC" => {
              ctx.arc(
                &Point2D::new(to_f32!(call.scope, v, "x"), to_f32!(call.scope, v, "y")),
                to_f32!(call.scope, v, "radius"),
                to_f32!(call.scope, v, "startAngle"),
                to_f32!(call.scope, v, "endAngle"),
                to_bool!(call.scope, v, "endAngle"),
              );
            },
            "ARCTO" => {
              let x1 = to_f32!(call.scope, v, "x1");
              let y1 = to_f32!(call.scope, v, "y1");
              let x2 = to_f32!(call.scope, v, "x2");
              let y2 = to_f32!(call.scope, v, "y2");
              let radius = to_f32!(call.scope, v, "radius");
              ctx.arc_to(&Point2D::new(x1, y1), &Point2D::new(x2, y2), radius);
            },
            "BEGINPATH" => {
              ctx.begin_path();
            },
            "BEZIERCURVETO" => {
              let cp1x = to_f32!(call.scope, v, "cp1x");
              let cp1y = to_f32!(call.scope, v, "cp1y");
              let cp2x = to_f32!(call.scope, v, "cp2x");
              let cp2y = to_f32!(call.scope, v, "cp2y");
              let x = to_f32!(call.scope, v, "x");
              let y = to_f32!(call.scope, v, "y");
              ctx.bezier_curve_to(&Point2D::new(cp1x, cp1y), &Point2D::new(cp2x, cp2y), &Point2D::new(x, y));
            },
            "CLEARRECT" => {
              let x = to_f32!(call.scope, v, "x");
              let y = to_f32!(call.scope, v, "y");
              let width = to_f32!(call.scope, v, "width");
              let height = to_f32!(call.scope, v, "height");
              ctx.clear_rect(&Rect::new(Point2D::new(x, y), Size2D::new(width, height)));
            },
            "CLIP" => {
              ctx.clip();
            },
            "CLOSEPATH" => {
              ctx.close_path();
            },
            "CREATEIMAGEDATA" => {
              // todo
            },
            "CREATELINEARGRADIENT" => {
              let x0 = to_f32!(call.scope, v, "x0");
              let y0 = to_f32!(call.scope, v, "y0");
              let x1 = to_f32!(call.scope, v, "x1");
              let y1 = to_f32!(call.scope, v, "y1");
              // todo
            },
            "CREATERADIALGRADIENT" => {
              let x0 = to_f32!(call.scope, v, "x0");
              let y0 = to_f32!(call.scope, v, "y0");
              let r0 = to_f32!(call.scope, v, "r0");
              let x1 = to_f32!(call.scope, v, "x1");
              let y1 = to_f32!(call.scope, v, "y1");
              let r1 = to_f32!(call.scope, v, "r1");
              // todo
            },
            "DRAWIMAGE" => {
              let data = to_array!(call.scope, v, "data");
              let height = to_f64!(call.scope, v, "height");
              let width = to_f64!(call.scope, v, "width");
              let sx = to_f64!(call.scope, v, "sx");
              let sy = to_f64!(call.scope, v, "sy");
              let s_width = to_f64!(call.scope, v, "sWidth");
              let s_height = to_f64!(call.scope, v, "sHeight");
              let dx = to_f64!(call.scope, v, "dx");
              let dy = to_f64!(call.scope, v, "dy");
              let d_width = to_f64!(call.scope, v, "dWidth");
              let d_height = to_f64!(call.scope, v, "dHeight");
              let image_data = data.iter().map(|v| v.check::<JsNumber>().expect("Unpack JsNumber Error").value() as u8).collect();
              ctx.draw_image(
                image_data,
                Size2D::new(width, height),
                Rect::new(Point2D::new(dx, dy), Size2D::new(d_width, d_height)),
                Rect::new(Point2D::new(sx, sy), Size2D::new(s_width, s_height)),
                true
              )
            },
            "FILL" => {
              ctx.fill();
            },
            "FILLRECT" => {
              let x = to_f32!(call.scope, v, "x");
              let y = to_f32!(call.scope, v, "y");
              let width = to_f32!(call.scope, v, "width");
              let height = to_f32!(call.scope, v, "height");
              ctx.fill_rect(&Rect::new(Point2D::new(x, y), Size2D::new(width, height)));
            },
            "FILLTEXT" => {
              let text = to_str!(call.scope, v, "text");
              let x = to_f32!(call.scope, v, "x");
              let y = to_f32!(call.scope, v, "y");
              let max_width = match v.get(call.scope, "maxWidth").unwrap().variant() {
                Variant::Number(n) => Some(n.value() as f32),
                _ => None,
              };
              ctx.fill_text(text, x, y, max_width);
            },
            "LINETO" => {
              let x = to_f32!(call.scope, v, "x");
              let y = to_f32!(call.scope, v, "y");
              ctx.line_to(&Point2D::new(x, y));
            },
            "MEASURETEXT" => {
              let text = to_str!(call.scope, v, "text");
              // todo
            },
            "MOVETO" => {
              let x = to_f32!(call.scope, v, "x");
              let y = to_f32!(call.scope, v, "y");
              ctx.move_to(&Point2D::new(x, y));
            },
            "PUTIMAGEDATA" => {
              let dx = to_f64!(call.scope, v, "dx");
              let dy = to_f64!(call.scope, v, "dy");
              let dirty_x = to_f64!(call.scope, v, "dirtyX");
              let dirty_y = to_f64!(call.scope, v, "dirtyY");
              let dirty_width = to_f64!(call.scope, v, "dirtyWidth");
              let dirty_height = to_f64!(call.scope, v, "dirtyHeight");
              // todo
            },
            "QUADRATICCURVETO" => {
              let cpx = to_f32!(call.scope, v, "cpx");
              let cpy = to_f32!(call.scope, v, "cpy");
              let x = to_f32!(call.scope, v, "x");
              let y = to_f32!(call.scope, v, "y");
              ctx.quadratic_curve_to(&Point2D::new(cpx, cpy), &Point2D::new(x, y));
            },
            "RECT" => {
              let x = to_f32!(call.scope, v, "x");
              let y = to_f32!(call.scope, v, "y");
              let width = to_f32!(call.scope, v, "width");
              let height = to_f32!(call.scope, v, "height");
              ctx.rect(&Rect::new(Point2D::new(x, y), Size2D::new(width, height)));
            },
            "RESTORE" => {
              ctx.restore_context_state();
            },
            "ROTATE" => {
              let angle = to_f32!(call.scope, v, "angle");
              // todo
            },
            "SAVE" => {
              ctx.save_context_state();
            },
            "SCALE" => {
              let x = to_f32!(call.scope, v, "x");
              let y = to_f32!(call.scope, v, "y");
              // todo
            },
            "SETLINEDASH" => {
              // todo
            },
            "SETTRANSFORM" => {
              let a = to_f32!(call.scope, v, "a");
              let b = to_f32!(call.scope, v, "b");
              let c = to_f32!(call.scope, v, "c");
              let d = to_f32!(call.scope, v, "d");
              let e = to_f32!(call.scope, v, "e");
              let f = to_f32!(call.scope, v, "f");
              ctx.set_transform(&Transform2D::row_major(a, b, c, d, e, f))
            },
            "STROKE" => {
              ctx.stroke();
            },
            "STROKERECT" => {
              let x = to_f32!(call.scope, v, "x");
              let y = to_f32!(call.scope, v, "y");
              let width = to_f32!(call.scope, v, "width");
              let height = to_f32!(call.scope, v, "height");
              ctx.stroke_rect(&Rect::new(Point2D::new(x, y), Size2D::new(width, height)));
            },
            "STROKETEXT" => {
              let text = to_str!(call.scope, v, "text");
              let x = to_f32!(call.scope, v, "x");
              let y = to_f32!(call.scope, v, "y");
              let max_width = match v.get(call.scope, "maxWidth").unwrap().variant() {
                Variant::Number(n) => Some(n.value() as f32),
                _ => None,
              };
              ctx.stroke_text(text, x, y, max_width);
            },
            "TRANSFORM" => {
              let a = to_f64!(call.scope, v, "a");
              let b = to_f64!(call.scope, v, "b");
              let c = to_f64!(call.scope, v, "c");
              let d = to_f64!(call.scope, v, "d");
              let e = to_f64!(call.scope, v, "e");
              let f = to_f64!(call.scope, v, "f");
              // todo
            },
            "TRANSLATE" => {
              let x = to_str!(call.scope, v, "x");
              let y = to_f64!(call.scope, v, "y");
              // todo
            },
            "SET_CURRENTTRANSFORM" => {
              let transform = to_object!(call.scope, v, "transform");
              let a = to_f32!(call.scope, transform, "a");
              let b = to_f32!(call.scope, transform, "b");
              let c = to_f32!(call.scope, transform, "c");
              let d = to_f32!(call.scope, transform, "d");
              let e = to_f32!(call.scope, transform, "e");
              let f = to_f32!(call.scope, transform, "f");
              ctx.set_transform(&Transform2D::row_major(a, b, c, d, e, f))
            },
            "SET_FILLSTYLE" => {
              let fill_style = v.get(call.scope, "fillStyle").unwrap();
              match FillOrStrokeStyle::from_handle(fill_style) {
                Some(s) => ctx.set_fill_style(s),
                None => println!("illegal fillStyle"),
              };
            },
            "SET_FONT" => {
              let font = to_str!(call.scope, v, "font");
              ctx.set_font_style(&font);
            },
            "SET_GLOBALALPHA" => {
              let global_alpha = to_f32!(call.scope, v, "globalAlpha");
              ctx.set_global_alpha(global_alpha)
            },
            "SET_GLOBALCOMPOSITEOPERATION" => {
              let global_composite_operation = to_str!(call.scope, v, "globalCompositeOperation");
              match CompositionOrBlending::from_str(&global_composite_operation) {
                Ok(s) => ctx.set_global_composition(s),
                Err(e) => println!("illegal globalCompositeOperation"),
              };
            },
            "SET_LINECAP" => {
              let line_cap = to_str!(call.scope, v, "lineCap");
              match LineCapStyle::from_str(&line_cap) {
                Ok(s) => ctx.set_line_cap(s),
                Err(e) => println!("illegal lineCap"),
              };
            },
            "SET_LINEDASHOFFSET" => {
              let line_dash_offset = to_f32!(call.scope, v, "lineDashOffset");
              // todo
            },
            "SET_LINEJOIN" => {
              let line_join = to_str!(call.scope, v, "lineJoin");
              match LineJoinStyle::from_str(&line_join) {
                Ok(s) => ctx.set_line_join(s),
                Err(e) => println!("illegal lineJoin"),
              };
            },
            "SET_LINEWIDTH" => {
              let line_width = to_f32!(call.scope, v, "lineWidth");
              ctx.set_line_width(line_width)
            },
            "SET_MITERLIMIT" => {
              let miter_limit = to_f32!(call.scope, v, "miterLimit");
              ctx.set_miter_limit(miter_limit)
            },
            "SET_SHADOWBLUR" => {
              let shadow_blur = to_f64!(call.scope, v, "shadowBlur");
              ctx.set_shadow_blur(shadow_blur)
            },
            "SET_SHADOWCOLOR" => {
              let shadow_color = to_str!(call.scope, v, "shadowColor");
              let input = &mut ParserInput::new(&shadow_color);
              let parser = &mut Parser::new(input);
              let parse_result = Color::parse(parser);
              match parse_result {
                Ok(rgba) => {
                  match rgba {
                    Color::RGBA(rgba) => ctx.set_shadow_color(rgba.to_azure_style()),
                    _ => println!("illegal shadowColor"),
                  }
                },
                Err(e) => println!("illegal shadowColor"),
              }
            },
            "SET_SHADOWOFFSETX" => {
              let shadow_offset_x = to_f64!(call.scope, v, "shadowOffsetX");
              ctx.set_shadow_offset_x(shadow_offset_x)
            },
            "SET_SHADOWOFFSETY" => {
              let shadow_offset_y = to_f64!(call.scope, v, "shadowOffsetY");
              ctx.set_shadow_offset_y(shadow_offset_y)
            },
            "SET_STROKESTYLE" => {
              let stroke_style = v.get(call.scope, "strokeStyle").unwrap();
              match FillOrStrokeStyle::from_handle(stroke_style) {
                Some(s) => ctx.set_stroke_style(s),
                None => println!("illegal strokeStyle"),
              };
            },
            "SET_TEXTALIGN" => {
              let text_align = to_str!(call.scope, v, "textAlign");
              // todo
            },
            "SET_TEXTBASELINE" => {
              let text_baseline = to_str!(call.scope, v, "textBaseline");
              // todo
            },
            _ => println!("{}", action_type),
          };
        });

      let canvas_size = Size2D::new(canvas.width as f64, canvas.height as f64);
      let size_i32 = canvas_size.to_i32();
      let buffer = ctx.image_data(Rect::new(Point2D::new(0i32, 0i32), size_i32), canvas_size);
      let mut js_buffer = JsBuffer::new(call.scope, buffer.len() as u32).expect("New JsBuffer Error");
      js_buffer.grab(|mut contents| {
        let slice = contents.as_mut_slice();
        for i in 0..slice.len() {
          slice[i] = buffer[i];
        }
      });
      Ok(js_buffer.as_value(call.scope))
    }
  }
}

register_module!(m, {
  let class: Handle<JsClass<Canvas>> = try!(Canvas::class(m.scope));
  let constructor: Handle<JsFunction<Canvas>> = try!(class.constructor(m.scope));
  let exports = m.exports
    .check::<JsObject>()
    .unwrap();
  let deref_module = exports.deref();
  try!(deref_module.set("Canvas", constructor));
  Ok(())
});

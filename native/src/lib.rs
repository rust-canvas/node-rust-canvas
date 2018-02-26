extern crate cssparser;
extern crate euclid;
#[macro_use] extern crate neon;
extern crate rustcanvas;

#[macro_use] mod macros;

use std::ops::Deref;

use cssparser::{RGBA};
use euclid::{Rect, Size2D, Point2D};
use neon::mem::{Handle};
use neon::js::{JsArray, JsFunction, JsObject, JsString, JsNumber, JsBoolean, Object, Value};
use neon::js::binary::{JsBuffer};
use neon::js::class::{JsClass, Class};
use neon::vm::{Lock, JsResult, This, FunctionCall};
use rustcanvas::{CanvasElement, create_canvas, CanvasContextType, FillOrStrokeStyle};

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
        ?.value() as i32;
      let height = call
        .check_argument::<JsNumber>(1)
        ?.value() as i32;
      let canvas = create_canvas(width, height, CanvasContextType::CTX2D);
      Ok(canvas)
    }

    method toBlob(mut call) {
      let actions = call
        .check_argument::<JsArray>(0)
        ?.to_vec(call.scope)
        .unwrap();
      let states = call
        .check_argument::<JsArray>(1)
        ?.to_vec(call.scope)
        .unwrap();

      let mut this = call.arguments.this(call.scope);
      let canvas: &mut CanvasElement = this.grab(|c| c );
      let ctx = &mut canvas.ctx;
      ctx.set_line_width(10.0);
      ctx.set_stroke_style(FillOrStrokeStyle::Color(RGBA::new(66, 165, 245, 100)));
      actions.iter()
        .map(|v| v.check::<JsObject>().unwrap())
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
              let x0 = to_f32!(call.scope, v, "x0");;
              let y0 = to_f32!(call.scope, v, "y0");
              let r0 = to_f32!(call.scope, v, "r0");
              let x1 = to_f32!(call.scope, v, "x1");
              let y1 = to_f32!(call.scope, v, "y1");
              let r1 = to_f32!(call.scope, v, "r1");
              // todo
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
              let text = to_str!(call.scope, v, "text");;
              let x = to_f32!(call.scope, v, "x");
              let y = to_f32!(call.scope, v, "y");
              let max_width = to_f32!(call.scope, v, "maxWidth");
              ctx.fill_text(text, x, y, Some(max_width));
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
              let dy = to_f64!(call.scope, v, "dy");;
              let dirty_x = to_f64!(call.scope, v, "dirtyX");;
              let dirty_y = to_f64!(call.scope, v, "dirtyY");;
              let dirty_width = to_f64!(call.scope, v, "dirtyWidth");;
              let dirty_height = to_f64!(call.scope, v, "dirtyHeight");;
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
              let x = to_f64!(call.scope, v, "x");
              let y = to_f64!(call.scope, v, "y");
              let max_width = to_f64!(call.scope, v, "maxWidth");
              // todo
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
              let x = v.get(call.scope, "x")
                .unwrap()
                .check::<JsString>()
                .unwrap()
                .value();
              let y = v.get(call.scope, "y")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f64;
              // todo
            },
            "SET_CURRENTTRANSFORM" => {
              let transform = v.get(call.scope, "transform")
                .unwrap()
                .check::<JsObject>()
                .unwrap();
              // ctx.set_transform(transform)
            },
            "SET_FILLSTYLE" => {
              let fill_style = v.get(call.scope, "fillStyle")
                .unwrap()
                .check::<JsObject>()
                .unwrap();
              // ctx.set_fill_style(fill_style);
            },
            "SET_FONT" => {
              let font = v.get(call.scope, "font")
                .unwrap()
                .check::<JsObject>()
                .unwrap();
              // ctx.set_font_style(font)
            },
            "SET_GLOBALALPHA" => {
              let global_alpha = to_f32!(call.scope, v, "globalAlpha");
              ctx.set_global_alpha(global_alpha)
            },
            "SET_GLOBALCOMPOSITEOPERATION" => {
              let global_composite_operation = v.get(call.scope, "globalCompositeOperation")
                .unwrap()
                .check::<JsObject>()
                .unwrap();
              // ctx.set_global_composition(global_composite_operation)
            },
            "SET_LINECAP" => {
              let line_cap = v.get(call.scope, "lineCap")
                .unwrap()
                .check::<JsObject>()
                .unwrap();
              // ctx.set_line_cap(line_cap)
            },
            "SET_LINEDASHOFFSET" => {
              let line_dash_offset = v.get(call.scope, "lineDashOffset")
                .unwrap()
                .check::<JsObject>()
                .unwrap();
              // todo
            },
            "SET_LINEJOIN" => {
              let line_join = v.get(call.scope, "lineJoin")
                .unwrap()
                .check::<JsObject>()
                .unwrap();
              // ctx.set_line_join(line_join)
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
              let shadow_color = v.get(call.scope, "shadowColor")
                .unwrap()
                .check::<JsObject>()
                .unwrap();
              // ctx.set_shadow_color(shadow_color)
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
              let stroke_style = v.get(call.scope, "strokeStyle")
                .unwrap()
                .check::<JsObject>()
                .unwrap();
              // ctx.set_stroke_style(stroke_style)
            },
            "SET_TEXTALIGN" => {
              let text_align = v.get(call.scope, "textAlign")
                .unwrap()
                .check::<JsObject>()
                .unwrap();
              // todo
            },
            "SET_TEXTBASELINE" => {
              let text_baseline = v.get(call.scope, "textBaseline")
                .unwrap()
                .check::<JsObject>()
                .unwrap();
              // todo
            },
            _ => println!("{}", action_type),
          };
        });
      let canvas_size = Size2D::new(canvas.width as f64, canvas.height as f64);
      let size_i32 = canvas_size.to_i32();
      let buffer = ctx.image_data(Rect::new(Point2D::new(0i32, 0i32), size_i32), canvas_size);
      let mut js_buffer = JsBuffer::new(call.scope, buffer.len() as u32).unwrap();
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

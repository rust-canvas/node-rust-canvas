extern crate cssparser;
extern crate euclid;
#[macro_use] extern crate neon;
extern crate rustcanvas;

use std::ops::Deref;

use cssparser::{RGBA};
use euclid::{Rect, Size2D, Point2D, Vector2D};
use neon::mem::{Handle, Managed};
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
        .map(|v| JsObject::from_raw(v.to_raw()))
        .for_each(|v| {
          let action_type = v.get(call.scope, "type")
            .unwrap()
            .check::<JsString>()
            .unwrap()
            .value();
          match action_type.deref() {
            "ARC" => {
              let x = v.get(call.scope, "x")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let y = v.get(call.scope, "y")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let radius = v.get(call.scope, "radius")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let start_angle = v.get(call.scope, "startAngle")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let end_angle = v.get(call.scope, "endAngle")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let ccw = v.get(call.scope, "anticlockwise")
                .unwrap()
                .check::<JsBoolean>()
                .unwrap()
                .value() as bool;
              ctx.arc(&Point2D::new(x, y), radius, start_angle, end_angle, ccw);
            },
            "ARCTO" => {
              let x1 = v.get(call.scope, "x1")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let y1 = v.get(call.scope, "y1")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let x2 = v.get(call.scope, "x2")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let y2 = v.get(call.scope, "y2")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let radius = v.get(call.scope, "radius")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              ctx.arc_to(&Point2D::new(x1, y1), &Point2D::new(x2, y2), radius);
            },
            "BEGINPATH" => {
              ctx.begin_path();
            },
            "BEZIERCURVETO" => {
              let cp1x = v.get(call.scope, "cp1x")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let cp1y = v.get(call.scope, "cp1y")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let cp2x = v.get(call.scope, "cp2x")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let cp2y = v.get(call.scope, "cp2y")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let x = v.get(call.scope, "x")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let y = v.get(call.scope, "y")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              ctx.bezier_curve_to(&Point2D::new(cp1x, cp1y), &Point2D::new(cp2x, cp2y), &Point2D::new(x, y));
            },
            "CLEARRECT" => {
              let x = v.get(call.scope, "x")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let y = v.get(call.scope, "y")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let width = v.get(call.scope, "width")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let height = v.get(call.scope, "height")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
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
              let x0 = v.get(call.scope, "x0")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let y0 = v.get(call.scope, "y0")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let x1 = v.get(call.scope, "x1")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let y1 = v.get(call.scope, "y1")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              // todo
            },
            "CREATERADIALGRADIENT" => {
              let x0 = v.get(call.scope, "x0")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let y0 = v.get(call.scope, "y0")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let r0 = v.get(call.scope, "r0")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let x1 = v.get(call.scope, "x1")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let y1 = v.get(call.scope, "y1")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let r1 = v.get(call.scope, "r1")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              // todo
            },
            "FILL" => {
              ctx.fill();
            },
            "FILLRECT" => {
              let x = v.get(call.scope, "x")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let y = v.get(call.scope, "y")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let width = v.get(call.scope, "width")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let height = v.get(call.scope, "height")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              ctx.fill_rect(&Rect::new(Point2D::new(x, y), Size2D::new(width, height)));
            },
            "FILLTEXT" => {
              let text = v.get(call.scope, "text")
                .unwrap()
                .check::<JsString>()
                .unwrap()
                .value();
              let x = v.get(call.scope, "x")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let y = v.get(call.scope, "y")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let max_width = v.get(call.scope, "maxWidth")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              ctx.fill_text(text, x, y, Some(max_width));
            },
            "LINETO" => {
              let x = v.get(call.scope, "x")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let y = v.get(call.scope, "y")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              ctx.line_to(&Point2D::new(x, y));
            },
            "MEASURETEXT" => {
              let text = v.get(call.scope, "text")
                .unwrap()
                .check::<JsString>()
                .unwrap()
                .value();
              // todo
            },
            "MOVETO" => {
              let x = v.get(call.scope, "x")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let y = v.get(call.scope, "y")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              ctx.move_to(&Point2D::new(x, y));
            },
            "PUTIMAGEDATA" => {
              let dx = v.get(call.scope, "dx")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f64;
              let dy = v.get(call.scope, "dy")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f64;
              let dirty_x = v.get(call.scope, "dirtyX")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f64;
              let dirty_y = v.get(call.scope, "dirtyY")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f64;
              let dirty_width = v.get(call.scope, "dirtyWidth")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f64;
              let dirty_height = v.get(call.scope, "dirtyHeight")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f64;
              // todo
            },
            "QUADRATICCURVETO" => {
              let cpx = v.get(call.scope, "cpx")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let cpy = v.get(call.scope, "cpy")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let x = v.get(call.scope, "x")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let y = v.get(call.scope, "y")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              ctx.quadratic_curve_to(&Point2D::new(cpx, cpy), &Point2D::new(x, y));
            },
            "RECT" => {
              let x = v.get(call.scope, "x")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let y = v.get(call.scope, "y")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let width = v.get(call.scope, "width")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let height = v.get(call.scope, "height")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              ctx.rect(&Rect::new(Point2D::new(x, y), Size2D::new(width, height)));
            },
            "RESTORE" => {
              ctx.restore_context_state();
            },
            "ROTATE" => {
              let angle = v.get(call.scope, "angle")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              // todo
            },
            "SAVE" => {
              ctx.save_context_state();
            },
            "SCALE" => {
              let x = v.get(call.scope, "x")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let y = v.get(call.scope, "y")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              // todo
            },
            "STROKE" => {
              ctx.stroke();
            },
            "STROKERECT" => {
              let x = v.get(call.scope, "x")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let y = v.get(call.scope, "y")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let width = v.get(call.scope, "width")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              let height = v.get(call.scope, "height")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              ctx.stroke_rect(&Rect::new(Point2D::new(x, y), Size2D::new(width, height)));
            },
            "STROKETEXT" => {
              let text = v.get(call.scope, "text")
                .unwrap()
                .check::<JsString>()
                .unwrap()
                .value();
              let x = v.get(call.scope, "x")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f64;
              let y = v.get(call.scope, "y")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f64;
              let max_width = v.get(call.scope, "maxWidth")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value();
              // todo
            },
            "TRANSFORM" => {
              let a = v.get(call.scope, "a")
                .unwrap()
                .check::<JsString>()
                .unwrap()
                .value();
              let b = v.get(call.scope, "b")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f64;
              let c = v.get(call.scope, "c")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f64;
              let d = v.get(call.scope, "d")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f64;
              let e = v.get(call.scope, "e")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f64;
              let f = v.get(call.scope, "f")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f64;
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
              // ctx.set_fill_style(fill_style)
            },
            "SET_FONT" => {
              let font = v.get(call.scope, "font")
                .unwrap()
                .check::<JsObject>()
                .unwrap();
              // ctx.set_font_style(font)
            },
            "SET_GLOBALALPHA" => {
              let global_alpha = v.get(call.scope, "globalAlpha")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
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
              let line_width = v.get(call.scope, "lineWidth")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              ctx.set_line_width(line_width)
            },
            "SET_MITERLIMIT" => {
              let miter_limit = v.get(call.scope, "miterLimit")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f32;
              ctx.set_miter_limit(miter_limit)
            },
            "SET_SHADOWBLUR" => {
              let shadow_blur = v.get(call.scope, "shadowBlur")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f64;
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
              let shadow_offset_x = v.get(call.scope, "shadowOffsetX")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f64;
              ctx.set_shadow_offset_x(shadow_offset_x)
            },
            "SET_SHADOWOFFSETY" => {
              let shadow_offset_y = v.get(call.scope, "shadowOffsetY")
                .unwrap()
                .check::<JsNumber>()
                .unwrap()
                .value() as f64;
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

extern crate cssparser;
extern crate euclid;
#[macro_use] extern crate neon;
extern crate rustcanvas;

use std::ops::Deref;

use cssparser::{RGBA};
use euclid::{Rect, Size2D, Point2D};
use neon::mem::{Handle, Managed};
use neon::js::{JsArray, JsFunction, JsObject, JsString, JsNumber, Object, Value};
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
            "STROKE" => {
              ctx.stroke();
            },
            "CLOSEPATH" => {
              ctx.close_path();
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

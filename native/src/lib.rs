extern crate euclid;
#[macro_use] extern crate neon;
extern crate rustcanvas;

use std::ops::Deref;

use euclid::{Rect, Size2D, Point2D};
use neon::mem::{Handle};
use neon::js::{JsFunction, JsObject, JsNumber, JsString, Object, Value};
use neon::js::binary::{JsBuffer};
use neon::js::class::{JsClass, Class};
use neon::vm::{Lock, JsResult, This, FunctionCall};
use rustcanvas::{CanvasElement, create_canvas, CanvasContextType};

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

    method toBlob(call) {
      let scope = call.scope;
      let mut this = call.arguments.this(scope);
      let canvas: &mut CanvasElement = this.grab(|c| c );
      let canvas_size = Size2D::new(canvas.width as f64, canvas.height as f64);
      let size_i32 = canvas_size.to_i32();
      let buffer = canvas.image_data(Rect::new(Point2D::new(0i32, 0i32), size_i32), canvas_size);
      let mut js_buffer = JsBuffer::new(scope, buffer.len() as u32).unwrap();
      js_buffer.grab(|mut contents| {
        let slice = contents.as_mut_slice();
        for i in 0..slice.len() {
          slice[i] = buffer[i];
        }
      });
      Ok(js_buffer.as_value(scope))
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

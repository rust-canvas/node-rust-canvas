use std::os::raw::{c_void};
use std::sync::mpsc::{channel};

use euclid::{Rect, Point2D, Size2D};
use image::{ImageBuffer, ImageRgba8, ImageFormat};
use neon::mem::{Managed};
use neon::js::binary::{JsBuffer};
use neon::scope::{Scope};
use neon::task::{Task};
use neon::vm::{JsResult};
use rustcanvas::{CanvasMsg, Canvas2dMsg, create_canvas, CanvasContextType};

pub struct Render {
  actions: Vec<Result<CanvasMsg, ()>>,
  width: i32,
  height: i32,
}

impl Render {
  pub fn new(actions: Vec<Result<CanvasMsg, ()>>, width: i32, height: i32) -> Render {
    Render { actions, width, height }
  }
}

impl Task for Render {
  type Output = Vec<u8>;
  type Error = String;
  type JsEvent = JsBuffer;

  fn perform(&self) -> Result<Self::Output, Self::Error> {
    let width = self.width;
    let height = self.height;
    let canvas = create_canvas(width, height, CanvasContextType::CTX2D);
    let renderer = canvas.ctx;
    for action in &self.actions {
      match action {
        &Ok(ref a) => renderer.send(a.clone()).unwrap(),
        _ => { },
      }
    }
    let canvas_size = Size2D::new(width as f64, height as f64);
    let size_i32 = canvas_size.to_i32();
    let (sender, reciver) = channel();
    renderer.send(CanvasMsg::Canvas2d(Canvas2dMsg::GetImageData(
      Rect::new(Point2D::new(0i32, 0i32), size_i32),
      canvas_size,
      sender,
    ))).unwrap();
    Ok(reciver.recv().unwrap())
  }

  fn complete<'a, T: Scope<'a>>(
    self,
    scope: &'a mut T,
    result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
      let width = self.width as u32;
      let height = self.height as u32;
      match result {
        Ok(o) => {
          let mut dist = vec![];
          {
            let png_buffer = ImageBuffer::from_raw(width, height, o).unwrap();
            let dynamic_image = ImageRgba8(png_buffer);
            dynamic_image.save(&mut dist, ImageFormat::PNG).unwrap();
          };
          let js_buffer = JsBuffer::new(scope, dist.len() as u32).unwrap();
          let mut local = js_buffer.to_raw();
          let raw_buf = dist.as_mut_ptr();
          local.handle = raw_buf as *mut c_void;
          Ok(js_buffer)
        },
        Err(e) => panic!(format!("{:?}", e)),
      }
  }
}

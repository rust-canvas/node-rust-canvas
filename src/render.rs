use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};

use euclid::{Rect, Point2D, Size2D};
use neon::js::binary::{JsBuffer};
use neon::scope::{Scope};
use neon::task::{Task};
use neon::vm::{JsResult};
use rustcanvas::{CanvasMsg, Canvas2dMsg};

use super::image_buffer::{image_buffer};
use image::{ImageFormat};

pub struct Render {
  actions: Vec<Result<CanvasMsg, ()>>,
  width: i32,
  height: i32,
  format_type: ImageFormat,
  encoder_options: f32,
  renderer: Arc<Mutex<Sender<CanvasMsg>>>,
}

impl Render {
  pub fn new(renderer: Arc<Mutex<Sender<CanvasMsg>>>, actions: Vec<Result<CanvasMsg, ()>>, width: i32, height: i32, format_type: ImageFormat, encoder_options: f32) -> Render {
    Render { renderer, actions, width, height, format_type, encoder_options }
  }
}

impl Task for Render {
  type Output = Vec<u8>;
  type Error = String;
  type JsEvent = JsBuffer;

  fn perform(&self) -> Result<Self::Output, Self::Error> {
    let width = self.width;
    let height = self.height;
    let renderer = self.renderer.clone();
    let renderer = renderer.lock().unwrap();
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
          image_buffer(o, scope, width, height, self.format_type, self.encoder_options)
            .and_then(|b| b.check::<JsBuffer>())
        },
        Err(e) => panic!(format!("{:?}", e)),
      }
  }
}

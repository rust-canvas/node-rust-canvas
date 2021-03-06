use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};

use euclid::{Rect, Point2D, Size2D};
use neon::js::binary::{JsBuffer};
use neon::scope::{Scope};
use neon::task::{Task};
use neon::vm::{Lock, JsResult};
use rustcanvas::{CanvasMsg, Canvas2dMsg};

pub struct Render {
  actions: Vec<Result<CanvasMsg, ()>>,
  width: i32,
  height: i32,
  renderer: Arc<Mutex<Sender<CanvasMsg>>>,
}

impl Render {
  pub fn new(renderer: Arc<Mutex<Sender<CanvasMsg>>>, actions: Vec<Result<CanvasMsg, ()>>, width: i32, height: i32) -> Render {
    Render { renderer, actions, width, height }
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
    let (sender, reciver) = channel();
    let renderer = renderer.lock().unwrap();
    for action in &self.actions {
      match action {
        &Ok(ref a) => renderer.send(a.clone()).unwrap(),
        _ => { },
      }
    }
    let canvas_size = Size2D::new(width as f64, height as f64);
    let size_i32 = canvas_size.to_i32();
    renderer.send(CanvasMsg::Canvas2d(Canvas2dMsg::GetImageData(
      Rect::new(Point2D::new(0i32, 0i32), size_i32),
      canvas_size,
      sender,
    ))).unwrap();
    drop(renderer);
    let data = reciver.recv().expect("Recive data fail");
    Ok(data)
  }

  fn complete<'a, T: Scope<'a>>(
    self,
    scope: &'a mut T,
    result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
      match result {
        Ok(dist) => {
          let mut js_buffer = JsBuffer::new(scope, dist.len() as u32).unwrap();
          js_buffer.grab(|mut contents| {
            contents.as_mut_slice().copy_from_slice(dist.as_ref());
          });
          Ok(js_buffer)
        },
        Err(e) => panic!(e),
      }
  }
}

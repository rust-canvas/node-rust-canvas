use euclid::{Rect, Point2D, Size2D};
use ipc_channel::ipc::{IpcSender, channel};
use neon::js::binary::{JsBuffer};
use neon::scope::{Scope};
use neon::task::{Task};
use neon::vm::{JsResult};
use neon::vm::Lock;
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
    let (sender, reciver) = channel().unwrap();
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
      match result {
        Ok(o) => {
          let len = o.len();
          let buf = JsBuffer::new(scope, len as u32);
          match buf {
            Ok(mut b) => {
              b.grab(|mut contents| {
                let slice = contents.as_mut_slice();
                for i in 0..slice.len() {
                  slice[i] = o[i];
                }
              });
              Ok(b)
            },
            Err(e) => panic!(format!("{:?}", e)),
          }
        },
        Err(e) => panic!(format!("{:?}", e)),
      }
  }
}

use ipc_channel::ipc::{IpcSender};
use neon::js::binary::{JsBuffer};
use neon::scope::{Scope};
use neon::task::{Task};
use neon::vm::{JsResult};
use neon::vm::Lock;
use rustcanvas::{CanvasMsg};

pub struct Render {
  renderer: IpcSender<CanvasMsg>,
}

impl Render {
  pub fn new(renderer: IpcSender<CanvasMsg>) -> Render {
    Render { renderer }
  }
}

impl Task for Render {
  type Output = Vec<u8>;
  type Error = String;
  type JsEvent = JsBuffer;

  fn perform(&self) -> Result<Self::Output, Self::Error> {
    Ok(vec![1, 2, 3])
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

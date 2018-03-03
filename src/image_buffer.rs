use image::{ImageBuffer, ImageRgba8, ImageFormat};
use neon::mem::{Managed};
use neon::js::{JsValue, Value};
use neon::js::binary::{JsBuffer};
use neon::scope::{Scope};
use neon::vm::{Lock, JsResult};

pub fn image_buffer<'a, S: Scope<'a>>(o: Vec<u8>, scope: &'a mut S, width: u32, height: u32) -> JsResult<JsValue> {
  let mut dist = vec![];
  {
    let png_buffer = ImageBuffer::from_raw(width, height, o).unwrap();
    let dynamic_image = ImageRgba8(png_buffer);
    dynamic_image.save(&mut dist, ImageFormat::PNG).unwrap();
  };
  let mut js_buffer = JsBuffer::new(scope, dist.len() as u32).unwrap();
  js_buffer.grab(|mut contents| {
    contents.as_mut_slice().copy_from_slice(dist.as_ref());
  });
  Ok(js_buffer.as_value(scope))
}
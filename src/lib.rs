extern crate cssparser;
extern crate euclid;
#[macro_use] extern crate neon;
extern crate neon_runtime;
extern crate rustcanvas;

#[macro_use] mod macros;
mod traits;
mod render;

use std::ops::Deref;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};

use cssparser::{Color, Parser, ParserInput};
use euclid::{Rect, Size2D, Point2D, Transform2D};
use neon::mem::{Handle};
use neon::js::{JsArray, JsFunction, JsObject, JsString, JsNumber, JsBoolean, JsUndefined, Object, Value, Variant};
use neon::js::class::{JsClass, Class};
use neon::js::binary::{JsBuffer};
use neon::task::Task;
use neon::vm::{Lock, JsResult, This, FunctionCall};
use rustcanvas::{FillOrStrokeStyle, CompositionOrBlending, LineCapStyle, LineJoinStyle};
use rustcanvas::{CanvasMsg, Canvas2dMsg, Context2d};

use traits::*;
use render::Render;

trait CheckArgument<'a> {
  fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V>;
  fn variant_argument(&mut self, i: i32) -> Option<Variant>;
}

impl<'a, T: This> CheckArgument<'a> for FunctionCall<'a, T> {
  fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V> {
    self.arguments.require(self.scope, i)?.check::<V>()
  }

  fn variant_argument(&mut self, i: i32) -> Option<Variant> {
    match self.arguments.get(self.scope, i) {
      None => None,
      Some(value) => Some(value.variant())
    }
  }
}

pub struct CanvasRenderer {
  width: i32,
  height: i32,
  renderer: Arc<Mutex<Sender<CanvasMsg>>>,
}

macro_rules! collect_actions {
  ($c:expr, $a:expr) => (
    $a.iter()
      .map(|v| v.check::<JsObject>().expect("Canvas Action must be JsObject"))
      .map(|v| {
        let action_type = to_str!($c.scope, v, "type");
        match action_type.deref() {
          "ARC" => {
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::Arc(
              Point2D::new(to_f32!($c.scope, v, "x"), to_f32!($c.scope, v, "y")),
              to_f32!($c.scope, v, "radius"),
              to_f32!($c.scope, v, "startAngle"),
              to_f32!($c.scope, v, "endAngle"),
              to_bool!($c.scope, v, "endAngle")
            )))
          },
          "ARCTO" => {
            let x1 = to_f32!($c.scope, v, "x1");
            let y1 = to_f32!($c.scope, v, "y1");
            let x2 = to_f32!($c.scope, v, "x2");
            let y2 = to_f32!($c.scope, v, "y2");
            let radius = to_f32!($c.scope, v, "radius");
            Ok(CanvasMsg::Canvas2d(
              Canvas2dMsg::ArcTo(Point2D::new(x1, y1),
              Point2D::new(x2, y2),
              radius)))
          },
          "BEGINPATH" => {
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::BeginPath))
          },
          "BEZIERCURVETO" => {
            let cp1x = to_f32!($c.scope, v, "cp1x");
            let cp1y = to_f32!($c.scope, v, "cp1y");
            let cp2x = to_f32!($c.scope, v, "cp2x");
            let cp2y = to_f32!($c.scope, v, "cp2y");
            let x = to_f32!($c.scope, v, "x");
            let y = to_f32!($c.scope, v, "y");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::BezierCurveTo(
              Point2D::new(cp1x, cp1y),
              Point2D::new(cp2x, cp2y),
              Point2D::new(x, y))))
          },
          "CLEARRECT" => {
            let x = to_f32!($c.scope, v, "x");
            let y = to_f32!($c.scope, v, "y");
            let width = to_f32!($c.scope, v, "width");
            let height = to_f32!($c.scope, v, "height");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::ClearRect(
              Rect::new(Point2D::new(x, y),
              Size2D::new(width, height)))))
          },
          "CLIP" => {
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::Clip))
          },
          "CLOSEPATH" => {
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::ClosePath))
          },
          "CREATEIMAGEDATA" => {
            // todo
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::NotImplement))
          },
          "CREATELINEARGRADIENT" => {
            let x0 = to_f32!($c.scope, v, "x0");
            let y0 = to_f32!($c.scope, v, "y0");
            let x1 = to_f32!($c.scope, v, "x1");
            let y1 = to_f32!($c.scope, v, "y1");
            // todo
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::NotImplement))
          },
          "CREATERADIALGRADIENT" => {
            let x0 = to_f32!($c.scope, v, "x0");
            let y0 = to_f32!($c.scope, v, "y0");
            let r0 = to_f32!($c.scope, v, "r0");
            let x1 = to_f32!($c.scope, v, "x1");
            let y1 = to_f32!($c.scope, v, "y1");
            let r1 = to_f32!($c.scope, v, "r1");
            // todo
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::NotImplement))
          },
          "DRAWIMAGE" => {
            let mut data = to_buffer!($c.scope, v, "data");
            let height = to_f64!($c.scope, v, "height");
            let width = to_f64!($c.scope, v, "width");
            let sx = to_f64!($c.scope, v, "sx");
            let sy = to_f64!($c.scope, v, "sy");
            let s_width = to_f64!($c.scope, v, "sWidth");
            let s_height = to_f64!($c.scope, v, "sHeight");
            let dx = to_f64!($c.scope, v, "dx");
            let dy = to_f64!($c.scope, v, "dy");
            let d_width = to_f64!($c.scope, v, "dWidth");
            let d_height = to_f64!($c.scope, v, "dHeight");
            let image_data = data.grab(|d| {
              d.as_slice().to_vec()
            });
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::DrawImage(
              image_data,
              Size2D::new(width, height),
              Rect::new(Point2D::new(dx, dy), Size2D::new(d_width, d_height)),
              Rect::new(Point2D::new(sx, sy), Size2D::new(s_width, s_height)),
              true
            )))
          },
          "FILL" => {
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::Fill))
          },
          "FILLRECT" => {
            let x = to_f32!($c.scope, v, "x");
            let y = to_f32!($c.scope, v, "y");
            let width = to_f32!($c.scope, v, "width");
            let height = to_f32!($c.scope, v, "height");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::FillRect(Rect::new(Point2D::new(x, y), Size2D::new(width, height)))))
          },
          "FILLTEXT" => {
            let text = to_str!($c.scope, v, "text");
            let x = to_f32!($c.scope, v, "x");
            let y = to_f32!($c.scope, v, "y");
            let max_width = match v.get($c.scope, "maxWidth").unwrap().variant() {
              Variant::Number(n) => Some(n.value() as f32),
              _ => None,
            };
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::FillText(text, x, y, max_width)))
          },
          "LINETO" => {
            let x = to_f32!($c.scope, v, "x");
            let y = to_f32!($c.scope, v, "y");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::LineTo(Point2D::new(x, y))))
          },
          "MEASURETEXT" => {
            let text = to_str!($c.scope, v, "text");
            // todo
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::NotImplement))
          },
          "MOVETO" => {
            let x = to_f32!($c.scope, v, "x");
            let y = to_f32!($c.scope, v, "y");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::MoveTo(Point2D::new(x, y))))
          },
          "PUTIMAGEDATA" => {
            let dx = to_f64!($c.scope, v, "dx");
            let dy = to_f64!($c.scope, v, "dy");
            let dirty_x = to_f64!($c.scope, v, "dirtyX");
            let dirty_y = to_f64!($c.scope, v, "dirtyY");
            let dirty_width = to_f64!($c.scope, v, "dirtyWidth");
            let dirty_height = to_f64!($c.scope, v, "dirtyHeight");
            // todo
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::NotImplement))
          },
          "QUADRATICCURVETO" => {
            let cpx = to_f32!($c.scope, v, "cpx");
            let cpy = to_f32!($c.scope, v, "cpy");
            let x = to_f32!($c.scope, v, "x");
            let y = to_f32!($c.scope, v, "y");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::QuadraticCurveTo(Point2D::new(cpx, cpy), Point2D::new(x, y))))
          },
          "RECT" => {
            let x = to_f32!($c.scope, v, "x");
            let y = to_f32!($c.scope, v, "y");
            let width = to_f32!($c.scope, v, "width");
            let height = to_f32!($c.scope, v, "height");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::Rect(Rect::new(Point2D::new(x, y), Size2D::new(width, height)))))
          },
          "RESTORE" => {
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::RestoreContext))
          },
          "SAVE" => {
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::SaveContext))
          },
          "SETLINEDASH" => {
            // todo
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::NotImplement))
          },
          "SETTRANSFORM" => {
            let a = to_f32!($c.scope, v, "a");
            let b = to_f32!($c.scope, v, "b");
            let c = to_f32!($c.scope, v, "c");
            let d = to_f32!($c.scope, v, "d");
            let e = to_f32!($c.scope, v, "e");
            let f = to_f32!($c.scope, v, "f");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::SetTransform(Transform2D::row_major(a, b, c, d, e, f))))
          },
          "STROKE" => {
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::Stroke))
          },
          "STROKERECT" => {
            let x = to_f32!($c.scope, v, "x");
            let y = to_f32!($c.scope, v, "y");
            let width = to_f32!($c.scope, v, "width");
            let height = to_f32!($c.scope, v, "height");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::StrokeRect(
              Rect::new(Point2D::new(x, y),
              Size2D::new(width, height)))))
          },
          "STROKETEXT" => {
            let text = to_str!($c.scope, v, "text");
            let x = to_f32!($c.scope, v, "x");
            let y = to_f32!($c.scope, v, "y");
            let max_width = match v.get($c.scope, "maxWidth").unwrap().variant() {
              Variant::Number(n) => Some(n.value() as f32),
              _ => None,
            };
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::StrokeText(text, x, y, max_width)))
          },
          "SET_CURRENTTRANSFORM" => {
            let transform = to_object!($c.scope, v, "transform");
            let a = to_f32!($c.scope, transform, "a");
            let b = to_f32!($c.scope, transform, "b");
            let c = to_f32!($c.scope, transform, "c");
            let d = to_f32!($c.scope, transform, "d");
            let e = to_f32!($c.scope, transform, "e");
            let f = to_f32!($c.scope, transform, "f");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::SetTransform(Transform2D::row_major(a, b, c, d, e, f))))
          },
          "SET_FILLSTYLE" => {
            let fill_style = v.get($c.scope, "fillStyle").unwrap();
            match FillOrStrokeStyle::from_handle(fill_style) {
              Some(s) => Ok(CanvasMsg::Canvas2d(Canvas2dMsg::SetFillStyle(s))),
              None => panic!("illegal fillStyle"),
            }
          },
          "SET_FONT" => {
            let font = to_str!($c.scope, v, "font");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::SetFontStyle(font)))
          },
          "SET_GLOBALALPHA" => {
            let global_alpha = to_f32!($c.scope, v, "globalAlpha");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::SetGlobalAlpha(global_alpha)))
          },
          "SET_GLOBALCOMPOSITEOPERATION" => {
            let global_composite_operation = to_str!($c.scope, v, "globalCompositeOperation");
            match CompositionOrBlending::from_str(&global_composite_operation) {
              Ok(s) => Ok(CanvasMsg::Canvas2d(Canvas2dMsg::SetGlobalComposition(s))),
              Err(e) => panic!("illegal globalCompositeOperation"),
            }
          },
          "SET_LINECAP" => {
            let line_cap = to_str!($c.scope, v, "lineCap");
            match LineCapStyle::from_str(&line_cap) {
              Ok(s) => Ok(CanvasMsg::Canvas2d(Canvas2dMsg::SetLineCap(s))),
              Err(_) => panic!("illegal lineCap"),
            }
          },
          "SET_LINEDASHOFFSET" => {
            let line_dash_offset = to_f32!($c.scope, v, "lineDashOffset");
            // todo
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::NotImplement))
          },
          "SET_LINEJOIN" => {
            let line_join = to_str!($c.scope, v, "lineJoin");
            match LineJoinStyle::from_str(&line_join) {
              Ok(s) => Ok(CanvasMsg::Canvas2d(Canvas2dMsg::SetLineJoin(s))),
              Err(_) => panic!("illegal lineJoin"),
            }
          },
          "SET_LINEWIDTH" => {
            let line_width = to_f32!($c.scope, v, "lineWidth");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::SetLineWidth(line_width)))
          },
          "SET_MITERLIMIT" => {
            let miter_limit = to_f32!($c.scope, v, "miterLimit");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::SetMiterLimit(miter_limit)))
          },
          "SET_SHADOWBLUR" => {
            let shadow_blur = to_f64!($c.scope, v, "shadowBlur");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::SetShadowBlur(shadow_blur)))
          },
          "SET_SHADOWCOLOR" => {
            let shadow_color = to_str!($c.scope, v, "shadowColor");
            let input = &mut ParserInput::new(&shadow_color);
            let parser = &mut Parser::new(input);
            let parse_result = Color::parse(parser);
            match parse_result {
              Ok(rgba) => {
                match rgba {
                  Color::RGBA(rgba) =>
                    Ok(CanvasMsg::Canvas2d(Canvas2dMsg::SetShadowColor(rgba))),
                  _ => panic!("illegal shadowColor"),
                }
              },
              Err(_) => panic!("illegal shadowColor"),
            }
          },
          "SET_SHADOWOFFSETX" => {
            let shadow_offset_x = to_f64!($c.scope, v, "shadowOffsetX");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::SetShadowOffsetX(shadow_offset_x)))
          },
          "SET_SHADOWOFFSETY" => {
            let shadow_offset_y = to_f64!($c.scope, v, "shadowOffsetY");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::SetShadowOffsetY(shadow_offset_y)))
          },
          "SET_STROKESTYLE" => {
            let stroke_style = v.get($c.scope, "strokeStyle").unwrap();
            match FillOrStrokeStyle::from_handle(stroke_style) {
              Some(s) => Ok(CanvasMsg::Canvas2d(Canvas2dMsg::SetStrokeStyle(s))),
              None => panic!("illegal strokeStyle"),
            }
          },
          "SET_TEXTALIGN" => {
            let text_align = to_str!($c.scope, v, "textAlign");
            // todo
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::NotImplement))
          },
          "SET_TEXTBASELINE" => {
            let text_baseline = to_str!($c.scope, v, "textBaseline");
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::NotImplement))
            // todo
          },
          _ => {
            println!("{}", action_type);
            Ok(CanvasMsg::Canvas2d(Canvas2dMsg::NotImplement))
          },
        }
      })
  )
}

declare_types! {
  pub class Canvas for CanvasRenderer {
    init(mut call) {
      let width = call
        .check_argument::<JsNumber>(0)
        .expect("Check width error")
        .value() as i32;
      let height = call
        .check_argument::<JsNumber>(1)
        .expect("Check height error")
        .value() as i32;
      let renderer = Context2d::start(Size2D::new(width, height));
      let renderer = Arc::new(Mutex::new(renderer));
      Ok(CanvasRenderer { width, height, renderer })
    }

    method toBuffer(mut call) {
      let actions = call
        .check_argument::<JsArray>(0)
        .expect("Check actions error")
        .to_vec(call.scope)
        .expect("Unpack actions error");
      let callback = call.check_argument::<JsFunction>(1)
        .expect("Check toBuffer callback error");
      let mut this = call.arguments.this(call.scope);
      let (width, height, renderer) = this.grab(|c| (c.width, c.height, c.renderer.clone()));
      let canvas_actions = collect_actions!(call, actions).collect();
      let ren = Render::new(renderer, canvas_actions, width, height);
      ren.schedule(callback);
      return Ok(JsUndefined::new().as_value(call.scope))
    }

    method toBufferSync(mut call) {
      let actions = call
        .check_argument::<JsArray>(0)
        .expect("Check actions error")
        .to_vec(call.scope)
        .expect("Unpack actions error");
      let mut this = call.arguments.this(call.scope);
      let (width, height, renderer) = this.grab(|c| (c.width, c.height, c.renderer.clone()));
      let renderer = renderer.lock().unwrap();
      collect_actions!(call, actions).for_each(|action: Result<CanvasMsg, ()>| match action {
        Ok(a) => renderer.send(a.clone()).unwrap(),
        _ => { },
      });
      let canvas_size = Size2D::new(width as f64, height as f64);
      let size_i32 = canvas_size.to_i32();
      let (sender, reciver) = channel();
      renderer.send(CanvasMsg::Canvas2d(Canvas2dMsg::GetImageData(
        Rect::new(Point2D::new(0i32, 0i32), size_i32),
        canvas_size,
        sender,
      ))).unwrap();
      drop(renderer);
      let dist = reciver.recv().unwrap();
      let mut js_buffer = JsBuffer::new(call.scope, dist.len() as u32).unwrap();
      js_buffer.grab(|mut contents| {
        contents.as_mut_slice().copy_from_slice(dist.as_ref());
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

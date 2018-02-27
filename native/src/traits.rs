use std::f32;
use std::str::{FromStr};

use neon::js::{JsValue, Variant};
use neon::mem::{Handle};
use cssparser::{Color, Parser, ParserInput, Token};

use rustcanvas::{FillOrStrokeStyle};

pub trait FromJS<'a> {
  type Target;

  fn from_handle(js: Handle<'a, JsValue>) -> Option<Self::Target>;
}

impl <'a> FromJS<'a> for FillOrStrokeStyle {
  type Target = FillOrStrokeStyle;

  fn from_handle(js: Handle<'a, JsValue>) -> Option<FillOrStrokeStyle> {
    match js.variant() {
      Variant::String(s) => {
        let string = s.value();
        let input = &mut ParserInput::new(&string);
        let parser = &mut Parser::new(input);
        let n = parser.next();
        match n {
          Ok(&Token::Hash(ref s)) => {
            println!("{:?}", f32::from_str(s));
            None
            // Some(FillOrStrokeStyle::Color(*rgba))
          },
          Ok(_) => None,
          Err(e) => {
            println!("{:?}", e);
            None
          },
        }
      },
      _ => None,
    }
  }
}

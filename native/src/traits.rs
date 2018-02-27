use neon::js::{JsValue, Variant};
use neon::mem::{Handle};
use cssparser::{Color, Parser, ParserInput};

use rustcanvas::{FillOrStrokeStyle};

pub trait FromJS<'a> {
  type Target;

  fn from_handle(js: Handle<'a, JsValue>) -> Option<Self::Target>;
}

impl <'a> FromJS<'a> for FillOrStrokeStyle {
  type Target = FillOrStrokeStyle;

  fn from_handle(js: Handle<'a, JsValue>) -> Option<FillOrStrokeStyle> {
    match js.variant() {
      Variant::String(_) => {
        let parse_result = Color::from_handle(js);
        match parse_result {
          Some(Color::RGBA(rgba)) => Some(FillOrStrokeStyle::Color(rgba)),
          _ => None,
        }
      },
      _ => None,
    }
  }
}

impl <'a> FromJS<'a> for Color {
  type Target = Color;

  fn from_handle(js: Handle<'a, JsValue>) -> Option<Color> {
    match js.variant() {
      Variant::String(s) => {
        let string = s.value();
        let input = &mut ParserInput::new(&string);
        let parser = &mut Parser::new(input);
        let parse_result = Color::parse(parser).unwrap_or(Color::CurrentColor);
        match parse_result {
          Color::RGBA(rgba) => Some(Color::RGBA(rgba)),
          _ => None,
        }
      },
      _ => None,
    }
  }
}

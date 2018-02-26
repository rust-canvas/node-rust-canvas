macro_rules! to_f32 {
  ($s:expr, $o:expr, $k:expr) => { $o.get($s, $k).unwrap().check::<JsNumber>().unwrap().value() as f32 }
}

macro_rules! to_f64 {
  ($s:expr, $o:expr, $k:expr) => { $o.get($s, $k).unwrap().check::<JsNumber>().unwrap().value() }
}

macro_rules! to_bool {
  ($s:expr, $o:expr, $k:expr) => { $o.get($s, $k).unwrap().check::<JsBoolean>().unwrap().value() }
}

macro_rules! to_str {
  ($s:expr, $o:expr, $k:expr) => { $o.get($s, $k).unwrap().check::<JsString>().unwrap().value() }
}

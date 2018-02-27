macro_rules! to_f32 {
  ($s:expr, $o:expr, $k:expr) => { $o.get($s, $k).expect("to_f32 get fail").check::<JsNumber>().expect("to_f32 check fail").value() as f32 }
}

macro_rules! to_f64 {
  ($s:expr, $o:expr, $k:expr) => { $o.get($s, $k).expect("to_f64 get fail").check::<JsNumber>().expect("to_f32 check fail").value() }
}

macro_rules! to_bool {
  ($s:expr, $o:expr, $k:expr) => { $o.get($s, $k).expect("to_bool get fail").check::<JsBoolean>().expect("to_bool check fail").value() }
}

macro_rules! to_str {
  ($s:expr, $o:expr, $k:expr) => { $o.get($s, $k).expect("to_str get fail").check::<JsString>().expect("to_str check fail").value() }
}

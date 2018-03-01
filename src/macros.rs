macro_rules! to_f32 {
  ($s:expr, $o:expr, $k:expr) => { $o.get($s, $k).expect("to_f32 get fail").check::<JsNumber>().expect(&format!("{} {}","to_f32 check fail".to_string(),$k.to_string())).value() as f32 }
}

macro_rules! to_f64 {
  ($s:expr, $o:expr, $k:expr) => { $o.get($s, $k).expect("to_f64 get fail").check::<JsNumber>().expect(&format!("{} {}","to_f64 check fail".to_string(),$k.to_string())).value() }
}

macro_rules! to_bool {
  ($s:expr, $o:expr, $k:expr) => { $o.get($s, $k).expect("to_bool get fail").check::<JsBoolean>().expect(&format!("{} {}","to_bool check fail".to_string(),$k.to_string())).value() }
}

macro_rules! to_str {
  ($s:expr, $o:expr, $k:expr) => { $o.get($s, $k).expect("to_str get fail").check::<JsString>().expect(&format!("{} {}","to_str check fail".to_string(),$k.to_string())).value() }
}

macro_rules! to_object {
  ($s:expr, $o:expr, $k:expr) => { $o.get($s, $k).expect("to_object get fail").check::<JsObject>().expect(&format!("{} {}","to_object check fail".to_string(),$k.to_string())) }
}

macro_rules! to_array {
  ($s:expr, $o:expr, $k:expr) => { $o.get($s, $k).expect("to_array get fail").check::<JsArray>().expect(&format!("{} {}","to_array check fail".to_string(),$k.to_string())).to_vec($s).unwrap() }
}

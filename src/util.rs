// mod util;

pub fn humanize(mut value: u64) -> String {
  let sufix_table = [" B", " KiB", " MiB", " GiB", " TiB"];
  let mut divisions = 0;
  let mut f_value: f32 = value as f32;
  let mut rest: f32 = 0_f32;

  while value > 999 && divisions + 1 < sufix_table.len() {
    divisions = divisions + 1;
    let new_f_value = f_value / 1024_f32;
    value = new_f_value.floor() as u64;
    rest = f_value - value as f32 * 1024_f32;
    f_value = new_f_value;
  }

  let mut result = "".to_string();
  if value == 0 {
    value = 1;
    result = result + &value.to_string();
  } else {
    let decimal = rest / 1024_f32;
    result = result + &value.to_string();
    if value < 10 && decimal >= 0.1_f32 && decimal < 1_f32 {
      result += ".";
      result += &(decimal * 10_f32).floor().to_string();
    }
  }
  result = result + &sufix_table[divisions].to_string();

  return result;
}

#[test]
fn humanize_returns_expected_values() {
  assert_eq!("1 B",      humanize(1));
  assert_eq!("1 KiB",    humanize(1023));
  assert_eq!("1 KiB",    humanize(1025));
  assert_eq!("1.5 KiB",  humanize(1024 + 512));
  assert_eq!("2 MiB",    humanize(1024 * 1024 * 2));
  assert_eq!("2.4 MiB",  humanize(1024 * (2048 + 410)));
  assert_eq!("4 GiB",    humanize(1024 * 1024 * 2 * 1024 * 2));
  assert_eq!("3 TiB",    humanize(1024 * 1024 * 1024 * 1024 * 3));
  assert_eq!("1024 TiB", humanize(1024 * 1024 * 1024 * 1024 * 1024));
  assert_eq!("1024 TiB", humanize(1024 * 1024 * 1024 * 1024 * 1024 + 512));
}

// mod util;

pub fn humanize(mut value: u64) -> String {
  let sufix_table = [" B", " KiB", " MiB", " GiB", " TiB"];
  let mut divisions = 0;
  let mut f_value: f32 = value as f32;

  while value > 999 && divisions + 1 < sufix_table.len() {
    divisions = divisions + 1;
    f_value = f_value / 1024_f32;
    value = f_value.round() as u64;
  }

  let mut result = "".to_string();
  result = result + &value.to_string();
  result = result + &sufix_table[divisions].to_string();

  return result;
}

#[test]
fn humanize_returns_expected_values() {
  assert_eq!("1 B",      humanize(1));
  assert_eq!("1 KiB",    humanize(1025));
  assert_eq!("2 MiB",    humanize(1024 * 1024 * 2));
  assert_eq!("4 GiB",    humanize(1024 * 1024 * 2 * 1024 * 2));
  assert_eq!("3 TiB",    humanize(1024 * 1024 * 1024 * 1024 * 3));
  assert_eq!("1024 TiB", humanize(1024 * 1024 * 1024 * 1024 * 1024));
}

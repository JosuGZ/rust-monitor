// mod terminal;

use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ffi::CString;

#[link(name="ncurses")]
extern {
  fn initscr();
  fn timeout(delay: c_int);
  fn printw(fmt: *const c_char, ...);
  fn clear();
  fn refresh();
  fn getch() -> c_int;
  fn endwin();
}

pub fn init() {
  unsafe {
    initscr();
    timeout(100);
  }
}

pub fn print_header() {
  // 16, 8, 16, 16
  let the_message = CString::new(
    "Name             PID      RSS              Swap            >Sum             \n"
  ).unwrap();

  unsafe {
    printw(the_message.as_ptr());
  }
}

pub fn print_line(line: String) {
  let the_message = CString::new(line).unwrap();

  unsafe {
    printw(the_message.as_ptr());
  }
}

pub fn _clear() {
  unsafe {
    clear();
  }
}

pub fn _refresh() {
  unsafe {
    refresh();
  }
}

pub fn wait_key() -> Option<i32> { // TODO: Change return
  let result;

  unsafe {
    result = getch();
  }

  if result == -1 {
    return None;
  } else {
    return Some(result);
  }
}

pub fn deinit() {
  unsafe {
    endwin();
  }
}

// mod terminal;

extern crate ncurses;

use ncurses::*;

pub fn init() {
  initscr();
  timeout(100);
}

pub fn print_header() {
  // 16, 8, 16, 16
  addstr("Name             PID      RSS              Swap            >Sum             \n");
}

pub fn print_line(line: String) {
  addstr(&line);
}

pub fn clear() {
  ncurses::clear();
}

pub fn refresh() {
  ncurses::refresh();
}

pub fn wait_key() -> Option<i32> { // TODO: Change return
  let result = getch();

  if result == -1 {
    return None;
  } else {
    return Some(result);
  }
}

pub fn deinit() {
  endwin();
}

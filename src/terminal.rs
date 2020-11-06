// mod terminal;

extern crate ncurses;

use ncurses::*;
use super::proc::Proc;

struct Column<'a> {
  name: &'a str,
  width: i32,
  position: i32
}

static COLUMNS: [Column; 5] = [
  Column { name: "Name", width: 16, position:  0 },
  Column { name: "PID",  width:  8, position: 17 },
  Column { name: "RSS",  width: 16, position: 26 },
  Column { name: "Swap", width: 16, position: 43 },
  Column { name: "Sum",  width: 16, position: 60 }
];

pub fn init() {
  initscr();
  timeout(100);
}

pub fn print_header() {
  for column in &COLUMNS {
    mvaddnstr(0, column.position, column.name, column.width);
  }
}

pub fn print_line(proc: &Proc, position: i32) {
  mvaddnstr(position, COLUMNS[0].position, &proc.status.name, COLUMNS[0].width);
  mvaddnstr(position, COLUMNS[1].position, &proc.pid.to_string(), COLUMNS[0].width);
  mvaddnstr(position, COLUMNS[2].position, &proc.status.vm_rss.to_string(), COLUMNS[0].width);
  mvaddnstr(position, COLUMNS[3].position, &proc.status.vm_swap.to_string(), COLUMNS[0].width);
  let sum = proc.status.vm_rss + proc.status.vm_swap;
  mvaddnstr(position, COLUMNS[4].position, &sum.to_string(), COLUMNS[0].width);
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

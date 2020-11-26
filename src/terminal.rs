// mod terminal;

extern crate ncurses;

use ncurses::*;
use super::proc::Proc;

pub enum Key {
  KeyUp,
  KeyDown,
  KeyLeft,
  KeyRight,
  KeyEnter,
  KeyEsc
}

impl Key {
  fn from_i32(key: i32) -> Option<Key> {
    return match key {
      ncurses::KEY_UP => Some(Key::KeyUp),
      ncurses::KEY_DOWN => Some(Key::KeyDown),
      ncurses::KEY_LEFT => Some(Key::KeyLeft),
      ncurses::KEY_RIGHT => Some(Key::KeyRight),
      ncurses::KEY_ENTER => Some(Key::KeyEnter),
      27 => Some(Key::KeyEsc),
      _ => None
    };
  }
}

struct Column<'a> {
  name: &'a str,
  width: i32,
  position: i32
}

static COLUMNS: [Column; 5] = [
  Column { name: "Name                        ", width: 16, position:  0 },
  Column { name: "PID                         ", width:  6, position: 17 },
  Column { name: "RSS                         ", width:  16, position: 24 },
  Column { name: "Swap                        ", width:  16, position: 41 },
  Column { name: "Sum                         ", width:  16, position: 58 }
];

pub fn init() {
  initscr();
  raw();
  keypad(stdscr(), true);
  noecho();
  timeout(100);
  start_color();
}

pub fn print_header() {
  init_pair(1, COLOR_BLACK, COLOR_WHITE);
  attron(COLOR_PAIR(1));

  for column in &COLUMNS {
    mvaddnstr(0, column.position, column.name, column.width + 1);
  }
  attroff(COLOR_PAIR(1));
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

pub fn wait_key() -> Option<Key> { // TODO: Change return
  let result = getch();

  return Key::from_i32(result);
}

pub fn deinit() {
  endwin();
}

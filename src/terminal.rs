// mod terminal;

extern crate ncurses;
extern crate num_cpus;

use ncurses::*;
use super::util::humanize;
use super::proc::Proc;
use super::proc::Uptime;
use super::proc::MemInfo;

pub enum Key {
  KeyUp,
  KeyDown,
  KeyLeft,
  KeyRight,
  KeyEnter,
  KeyEsc,
  KeyGroup
}

impl Key {
  fn from_i32(key: i32) -> Option<Key> {
    match key {
      ncurses::KEY_UP => Some(Key::KeyUp),
      ncurses::KEY_DOWN => Some(Key::KeyDown),
      ncurses::KEY_LEFT => Some(Key::KeyLeft),
      ncurses::KEY_RIGHT => Some(Key::KeyRight),
      ncurses::KEY_ENTER => Some(Key::KeyEnter),
      27 => Some(Key::KeyEsc),
      103 => Some(Key::KeyGroup), // 'g'
      _ => None
    }
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
  Column { name: "RSS                         ", width:  8, position: 24 },
  Column { name: "Swap                        ", width:  8, position: 33 },
  Column { name: "Sum                         ", width:  8, position: 42 }
];

pub fn init() {
  initscr();
  raw();
  keypad(stdscr(), true);
  noecho();
  timeout(2000);
  start_color();
}

pub fn print_uptime(uptime: &Uptime, last_uptime: &Uptime) {
  let seconds_up = uptime.up as i32;
  let mut minutes_up = seconds_up / 60;
  let mut hours_up = minutes_up / 60;
  minutes_up -= hours_up * 60;
  let days_up = hours_up / 24;
  hours_up -= days_up * 24;

  let partial_uptime = uptime - last_uptime;
  let cpus = num_cpus::get() as f64;
  let idle_time = (partial_uptime.idle / cpus) / partial_uptime.up * 100_f64;

  let formated = format!(
    "{} days {:02}:{:02} | Idle: {:.1}%",
    days_up, hours_up, minutes_up, idle_time
  );
  mvaddnstr(0, 0, "Uptime: ", 20);
  mvaddnstr(0, 8, &formated, 72);
}

pub fn print_mem_info(mem_info: &MemInfo) {
  let formatted = format!(
    "Memory: [{} / {}] Swap: [{} / {}]",
    humanize(mem_info.mem_total - mem_info.mem_available),
    humanize(mem_info.mem_total),
    humanize(mem_info.swap_total - mem_info.swap_free),
    humanize(mem_info.swap_total)
  );
  mvaddnstr(1, 0, &formatted, 80);
}

pub fn print_header(group: bool, selected_col: usize) {
  init_pair(1, COLOR_BLACK, COLOR_WHITE);
  attron(COLOR_PAIR(1));

  for i in 0..COLUMNS.len() {
    let column = &COLUMNS[i];
    let mut column_name = column.name;

    if group && i == 1 {
      column_name = "Count  ";
    }

    mvaddnstr(2, column.position, column_name, column.width + 1);
    if i == selected_col + 1 {
      mvaddnstr(2, column.position -1, ">", 1);
    }
  }
  attroff(COLOR_PAIR(1));
}

pub fn print_line(proc: &Proc, position: i32, is_group: bool) {

  let value = &proc.status.name;
  mvaddnstr(position + 2, COLUMNS[0].position, value, COLUMNS[0].width);

  if !is_group {
    let value = &proc.pid.to_string();
    mvaddnstr(position + 2, COLUMNS[1].position, value, COLUMNS[1].width);
  } else {
    let value = &proc.count.to_string();
    mvaddnstr(position + 2, COLUMNS[1].position, value, COLUMNS[1].width);
  }

  let value = &humanize(proc.status.vm_rss);
  mvaddnstr(position + 2, COLUMNS[2].position, value, COLUMNS[0].width);

  let value = &humanize(proc.status.vm_swap);
  mvaddnstr(position + 2, COLUMNS[3].position, value, COLUMNS[0].width);

  let value = &humanize(proc.status.vm_rss + proc.status.vm_swap);
  mvaddnstr(position + 2, COLUMNS[4].position, value, COLUMNS[0].width);
}

pub fn clear() {
  ncurses::clear();
}

pub fn refresh() {
  ncurses::refresh();
}

pub fn wait_key() -> Option<Key> { // TODO: Change return
  let result = getch();

  Key::from_i32(result)
}

pub fn deinit() {
  endwin();
}

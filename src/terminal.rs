// mod terminal;

extern crate ncurses;
extern crate num_cpus;

use std::time::Instant;

use libc::{sysconf, _SC_CLK_TCK}; // TODO: Move out here
use ncurses::*;
use super::util::humanize;
use super::proc::Proc;
use super::proc::Uptime;
use super::proc::MemInfo;

pub enum Key {
  Up,
  Down,
  Left,
  Right,
  Enter,
  Esc,
  Group
}

impl Key {
  fn from_i32(key: i32) -> Option<Key> {
    match key {
      ncurses::KEY_UP => Some(Key::Up),
      ncurses::KEY_DOWN => Some(Key::Down),
      ncurses::KEY_LEFT => Some(Key::Left),
      ncurses::KEY_RIGHT => Some(Key::Right),
      ncurses::KEY_ENTER => Some(Key::Enter),
      27 => Some(Key::Esc),
      103 => Some(Key::Group), // 'g'
      _ => None
    }
  }
}

struct Column<'a> {
  name: &'a str,
  width: i32,
  position: i32
}

static COLUMNS: [Column; 6] = [
  Column { name: "Name                        ", width: 16, position:  0 },
  Column { name: "PID                         ", width:  6, position: 17 },
  Column { name: "[% CPU]                       ", width:  8, position: 24 },
  Column { name: "RSS                         ", width:  8, position: 32 },
  Column { name: "Swap                        ", width:  8, position: 41 },
  Column { name: "Sum                         ", width:  8, position: 50 }
];

pub struct Terminal {
  line: i32,
  sc_clk_tck: u64,
  last_update: Instant,
  elapsed_time: f32
}

impl Terminal {

  pub fn init() -> Terminal {
    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();
    timeout(2000);
    start_color();

    init_pair(1, COLOR_BLACK, COLOR_WHITE);
    init_pair(2, COLOR_BLACK, COLOR_GREEN);
    init_pair(3, COLOR_BLACK, COLOR_RED);
    init_pair(4, COLOR_BLACK, COLOR_YELLOW);

    let sc_clk_tck = unsafe {
      let sc_clk_tck = sysconf(_SC_CLK_TCK);
      if sc_clk_tck > 0 {
        sc_clk_tck as u64
      } else {
        100 // Silent fallback to 100
      }
    };

    Terminal {
      line: 0,
      sc_clk_tck,
      last_update: Instant::now(),
      elapsed_time: 0f32
    }
  }

  pub fn print_uptime(&mut self, uptime: &Uptime, last_uptime: &Uptime) {
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
      "{days_up} days {hours_up:02}:{minutes_up:02} | Idle: {idle_time:.1}%",
    );
    mvaddnstr(self.line, 0, "Uptime: ", 20);
    mvaddnstr(self.line, 8, &formated, 72);
    self.line += 1;
  }

  pub fn print_mem_info(&mut self, mem_info: &MemInfo) {
    let formatted = format!(
      "Memory: [{} / {}] Swap: [{} / {}]",
      humanize(mem_info.mem_total - mem_info.mem_available),
      humanize(mem_info.mem_total),
      humanize(mem_info.swap_total - mem_info.swap_free),
      humanize(mem_info.swap_total)
    );
    mvaddnstr(self.line, 0, &formatted, 80);
    self.line += 1;
  }

  pub fn print_swap_stats(
    &mut self, pages_in: u64, pages_out: u64
  ) {
    if pages_in != 0 || pages_out != 0 {
      let pages_in =  humanize(pages_in * 1024 * 4);
      let pages_out = humanize(pages_out * 1024 * 4);
      let formatted = format!("Swap: in: {pages_in} out: {pages_out}");
      mvaddnstr(self.line, 0, &formatted, 80);
      self.line += 1;
    }
  }

  pub fn print_battery(
    &mut self, percent: i32, rate: f32, hours: i32, minutes: i32
  ) {
    let formatted = format!(
      "Battery: [{percent}% | {rate:.3} w | {hours}:{minutes:02} remaining]",
    );
    mvaddnstr(self.line, 0, &formatted, 80);
    self.line += 1;
  }

  fn update_time(&mut self) {
    self.elapsed_time = self.last_update.elapsed().as_millis() as f32 / 1000f32;
    self.last_update = Instant::now();
  }

  pub fn print_header(&mut self, group: bool, selected_col: usize) {
    self.update_time();
    attron(COLOR_PAIR(1));

    for (i, column) in COLUMNS.iter().enumerate() {
      let mut column_name = column.name;

      if group && i == 1 {
        column_name = "Count  ";
      }

      mvaddnstr(self.line, column.position, column_name, column.width + 1);
      if i == selected_col + 1 {
        mvaddnstr(self.line, column.position -1, ">", 1);
      }
    }
    attroff(COLOR_PAIR(1));
    self.line += 1;
  }

  pub fn print_line(&mut self, proc: &Proc, is_group: bool) {
    let line = self.line;

    let color = if proc.new { Some(2) }
      else if proc.deleted { Some(3) }
      else if proc.new && proc.deleted { Some(4)}
      else { None }
    ;

    if let Some(color) = color {
      if proc.new { attron(COLOR_PAIR(color)); }
      if proc.deleted { attron(COLOR_PAIR(color)); }
      if proc.new && proc.deleted { attron(COLOR_PAIR(color)); }
      mvaddnstr(line, 0, &" ".repeat(59), 8000);
    }

    let value = &proc.status.name;
    mvaddnstr(line, COLUMNS[0].position, value, COLUMNS[0].width);

    if !is_group {
      let value = &proc.pid.to_string();
      mvaddnstr(line, COLUMNS[1].position, value, COLUMNS[1].width);
    } else {
      let value = &proc.count.to_string();
      mvaddnstr(line, COLUMNS[1].position, value, COLUMNS[1].width);
    }

    let value = proc.stat.utime + proc.stat.stime;
    let value = value * 100 / self.sc_clk_tck;
    let value = if self.elapsed_time != 0f32 {
      value as f32 / self.elapsed_time
    } else {
      0f32
    };
    let value = format!("{value:7.1} %");
    mvaddnstr(line, COLUMNS[2].position, &value, COLUMNS[2].width);

    let value = &humanize(proc.status.vm_rss);
    mvaddnstr(line, COLUMNS[3].position, value, COLUMNS[3].width);

    let value = &humanize(proc.status.vm_swap);
    mvaddnstr(line, COLUMNS[4].position, value, COLUMNS[4].width);

    let value = &humanize(proc.status.vm_rss + proc.status.vm_swap);
    mvaddnstr(line, COLUMNS[5].position, value, COLUMNS[5].width);

    if let Some(color) = color {
      attroff(COLOR_PAIR(color));
    }
    self.line += 1;
  }

  pub fn clear(&mut self) {
    self.line = 0;
    ncurses::clear();
  }

  pub fn refresh(&mut self) {
    ncurses::refresh();
  }

  pub fn wait_key(&mut self) -> Option<Key> { // TODO: Change return
    let result = getch();

    Key::from_i32(result)
  }

  pub fn deinit(&mut self) {
    endwin();
  }

}

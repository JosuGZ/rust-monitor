#![allow(clippy::erasing_op)]
#![allow(clippy::manual_range_contains)] // To allow or not to allow...
#![allow(clippy::derive_partial_eq_without_eq)] // To allow or not to allow...
#![allow(clippy::comparison_chain)] // To allow or not to allow...

mod util;
mod proc;
mod parsers;
mod terminal;
mod battery;

use std::fs::read_dir;
use std::collections::HashMap;

use proc::*;
use terminal::{Terminal, Key};
use crate::battery::Battery;

use parsers::get_proc;
use parsers::get_uptime;
use parsers::get_mem_info;

/// Returns an Ordering between 2 elements
fn comp<T: std::cmp::Ord>(a: &T, b: &T) -> std::cmp::Ordering {
  if a > b {
    std::cmp::Ordering::Less
  } else if a < b {
    std::cmp::Ordering::Greater
  } else {
    std::cmp::Ordering::Equal
  }
}

fn pid_sort_function(a: &Proc, b: &Proc) -> std::cmp::Ordering {
  let a_value = a.pid;
  let b_value = b.pid;
  comp(&a_value, &b_value)
}

fn count_sort_function(a: &Proc, b: &Proc) -> std::cmp::Ordering {
  let a_value = a.count;
  let b_value = b.count;
  comp(&a_value, &b_value)
}

fn rss_sort_function(a: &Proc, b: &Proc) -> std::cmp::Ordering {
  let a_value = a.status.vm_rss;
  let b_value = b.status.vm_rss;
  comp(&a_value, &b_value)
}

fn swap_sort_function(a: &Proc, b: &Proc) -> std::cmp::Ordering {
  let a_value = a.status.vm_swap;
  let b_value = b.status.vm_swap;
  comp(&a_value, &b_value)
}

fn sum_sort_function(a: &Proc, b: &Proc) -> std::cmp::Ordering {
  let a_value = a.status.vm_rss + a.status.vm_swap;
  let b_value = b.status.vm_rss + b.status.vm_swap;
  comp(&a_value, &b_value)
}

type SortFunction = fn (a: &proc::Proc, b: &proc::Proc) -> std::cmp::Ordering;

static SORT_FUNCTIONS: [SortFunction; 4] = [
  pid_sort_function,
  rss_sort_function,
  swap_sort_function,
  sum_sort_function
];

static GROUP_SORT_FUNCTIONS: [SortFunction; 4] = [
  count_sort_function,
  rss_sort_function,
  swap_sort_function,
  sum_sort_function
];

fn do_reading(
  terminal: &mut Terminal,
  sort_function: SortFunction, group: bool
) -> Result<(), std::io::Error> {
  let readed = read_dir("/proc")?;

  let procs = readed.enumerate().filter(|val| {
    matches!(val.1, Ok(_))
  }).map(|b| get_proc(&b.1.unwrap())).filter(|val| {
    matches!(val, Some(_))
  }).map(|val| val.unwrap());

  let mut procs_vec: Vec<Proc> = procs.collect();

  if group {
    let mut group = HashMap::new();
    for proc in procs_vec {
      let key = proc.status.name.clone();
      group.entry(key).and_modify(|p: &mut Proc| {
        p.count += 1;
        p.status.vm_rss += proc.status.vm_rss;
        p.status.vm_swap += proc.status.vm_swap;
      }).or_insert(proc);
    }

    procs_vec = group.into_iter().map(|e| e.1).collect();
  }

  procs_vec.sort_by(sort_function);

  for proc in procs_vec.iter() {
    terminal.print_line(proc, group);
  }

  Result::Ok(())
}

fn main() {
  let mut terminal = Terminal::init();
  let mut battery = Battery::init();

  let mut sort_function_index: usize = 3;
  let mut group: bool = false;
  let mut last_uptime: Uptime = Default::default();
  let mut uptime: Uptime;

  loop {
    let sort_functions = {
      if group {
        GROUP_SORT_FUNCTIONS
      } else {
        SORT_FUNCTIONS
      }
    };

    terminal.clear();
    uptime = get_uptime();
    terminal.print_uptime(&uptime, &last_uptime);
    last_uptime = uptime;
    terminal.print_mem_info(&get_mem_info());

    if let Some(battery) = &mut battery {
      battery.refresh();
      if battery.discharging() {
        let data = battery.get_data();
        terminal.print_battery(
          data.percent,
          data.rate,
          data.hours,
          data.minutes
        );
      }
    }

    terminal.print_header(group, sort_function_index);
    let result = do_reading(
      &mut terminal, sort_functions[sort_function_index], group
    );
    if let Err(err) = result { println!("{}", err); }
    terminal.refresh();

    let key_option = terminal.wait_key();
    match key_option {
      Some(Key::Right) => {
        sort_function_index += 1;
        if sort_function_index >= sort_functions.len() {
          sort_function_index = 0;
        }
      },
      Some(Key::Left) => {
        if sort_function_index > 0 {
          sort_function_index -= 1;
        } else {
          sort_function_index = sort_functions.len() - 1;
        }
      },
      Some(Key::Group) => group = !group,
      Some(Key::Esc) => break,
      _ => ()
    }
  }

  terminal.deinit(); // TODO: Make sure this gets called
}

mod util;
mod proc;
mod parsers;
mod terminal;

use std::fs::read_dir;
use std::collections::HashMap;

use proc::*;
use terminal::Key;

use parsers::get_proc;
use parsers::get_uptime;
use parsers::get_mem_info;

fn pid_sort_function(a: &Proc, b: &Proc) -> std::cmp::Ordering {
  let a_value = a.pid;
  let b_value = b.pid;
  if a_value > b_value {
    return std::cmp::Ordering::Less;
  } else if a_value < b_value {
    return std::cmp::Ordering::Greater;
  } else {
    return std::cmp::Ordering::Equal;
  }
}

fn rss_sort_function(a: &Proc, b: &Proc) -> std::cmp::Ordering {
  let a_value = a.status.vm_rss;
  let b_value = b.status.vm_rss;
  if a_value > b_value {
    return std::cmp::Ordering::Less;
  } else if a_value < b_value {
    return std::cmp::Ordering::Greater;
  } else {
    return std::cmp::Ordering::Equal;
  }
}

fn swap_sort_function(a: &Proc, b: &Proc) -> std::cmp::Ordering {
  let a_value = a.status.vm_swap;
  let b_value = b.status.vm_swap;
  if a_value > b_value {
    return std::cmp::Ordering::Less;
  } else if a_value < b_value {
    return std::cmp::Ordering::Greater;
  } else {
    return std::cmp::Ordering::Equal;
  }
}

fn sum_sort_function(a: &Proc, b: &Proc) -> std::cmp::Ordering {
  let a_value = a.status.vm_rss + a.status.vm_swap;
  let b_value = b.status.vm_rss + b.status.vm_swap;
  if a_value > b_value {
    return std::cmp::Ordering::Less;
  } else if a_value < b_value {
    return std::cmp::Ordering::Greater;
  } else {
    return std::cmp::Ordering::Equal;
  }
}

type SortFunction = fn (a: &proc::Proc, b: &proc::Proc) -> std::cmp::Ordering;

static SORT_FUNCTIONS: [SortFunction; 4] = [
  pid_sort_function,
  rss_sort_function,
  swap_sort_function,
  sum_sort_function
];

fn do_reading(
  sort_function_index: usize, group: bool
) -> Result<(), std::io::Error> {
  let readed = read_dir("/proc")?;

  let procs = readed.enumerate().filter(|val| {
    match val.1 {
      Ok(_) => true,
      _ => false
    }
  }).map(|b| get_proc(b.1.unwrap())).filter(|val| {
    match val {
      Some(_) => true,
      _ => false
    }
  }).map(|val| val.unwrap());

  let mut procs_vec: Vec<Proc> = procs.collect();

  if group {
    let mut group = HashMap::new();
    for proc in procs_vec {
      let key = proc.status.name.clone();
      group.entry(key).and_modify(|p: &mut Proc| {
        (*p).status.vm_rss = (*p).status.vm_rss + proc.status.vm_rss;
        (*p).status.vm_swap = (*p).status.vm_swap + proc.status.vm_swap;
      }).or_insert(proc);
    }

    procs_vec = group.into_iter().map(|e| e.1).collect();
  }

  procs_vec.sort_by(SORT_FUNCTIONS[sort_function_index]);

  for (i, proc) in procs_vec.iter().enumerate() {
    terminal::print_line(&proc, i as i32 + 1, group);
  }

  Result::Ok(())
}

fn main() {
  terminal::init();

  let mut sort_function_index: usize = 3;
  let mut group: bool = false;
  let mut last_uptime: Uptime = Default::default();
  let mut uptime: Uptime;

  loop {
    terminal::clear();
    uptime = get_uptime();
    terminal::print_uptime(&uptime, &last_uptime);
    last_uptime = uptime;
    terminal::print_mem_info(&get_mem_info());
    terminal::print_header(sort_function_index);
    match do_reading(sort_function_index, group) {
      Err(err) => println!("{}", err),
      _ => ()
    }
    terminal::refresh();

    let key_option = terminal::wait_key();
    match key_option {
      Some(key) => match key {
        Key::KeyRight => {
          sort_function_index = sort_function_index + 1;
          if sort_function_index >= SORT_FUNCTIONS.len() {
            sort_function_index = 0;
          }
        },
        Key::KeyLeft => {
          if sort_function_index > 0 {
            sort_function_index = sort_function_index - 1;
          } else {
            sort_function_index = SORT_FUNCTIONS.len() - 1;
          }
        },
        Key::KeyGroup => group = !group,
        Key::KeyEsc => break,
        _ => ()
      },
      _ => ()
    }
  }

  terminal::deinit(); // TODO: Make sure this gets called
}

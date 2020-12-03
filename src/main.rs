mod util;
mod proc;
mod parsers;
mod terminal;

use std::fs::read_dir;

use proc::Proc;
use terminal::Key;

use parsers::get_proc;

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

fn do_reading(sort_function_index: usize) -> Result<(), std::io::Error> {
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
  procs_vec.sort_by(SORT_FUNCTIONS[sort_function_index]);

  for (i, proc) in procs_vec.iter().enumerate() {
    terminal::print_line(&proc, i as i32 + 1);
  }

  Result::Ok(())
}

fn main() {
  terminal::init();

  let mut sort_function_index: usize = 3;

  loop {
    terminal::clear();
    terminal::print_header(sort_function_index);
    match do_reading(sort_function_index) {
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
        Key::KeyEsc => break,
        _ => ()
      },
      _ => ()
    }
  }

  terminal::deinit(); // TODO: Make sure this gets called
}

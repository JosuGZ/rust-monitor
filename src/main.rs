mod proc;
mod parsers;
mod terminal;

use std::fs::read_dir;

use proc::Proc;

use parsers::get_proc;

fn do_reading() -> Result<(), std::io::Error> {
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
  procs_vec.sort_by(|a, b| {
    let a_value = a.status.vm_rss + a.status.vm_swap;
    let b_value = b.status.vm_rss + b.status.vm_swap;
    if a_value > b_value {
      return std::cmp::Ordering::Less;
    } else if a_value < b_value {
      return std::cmp::Ordering::Greater;
    } else {
      return std::cmp::Ordering::Equal;
    }
  });

  for (i, proc) in procs_vec.iter().enumerate() {
    terminal::print_line(&proc, i as i32 + 1);
  }

  Result::Ok(())
}

fn main() {
  terminal::init();

  loop {
    terminal::clear();
    terminal::print_header();
    match do_reading() {
      Err(err) => println!("{}", err),
      _ => ()
    }
    terminal::refresh();

    let key_option = terminal::wait_key();
    if let Some(_) = key_option {
      break;
    }
  }

  terminal::deinit(); // TODO: Make sure this gets called
}

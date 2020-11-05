mod proc;
mod parsers;
mod terminal;

use std::fs::read_dir;

use std::string::String;

use proc::Proc;

use parsers::get_proc;

fn print_line(proc: &Proc) {
  let mut line: String = "".to_string();

  let name_extended: String = proc.status.name.clone() + "                ";
  let name: String = name_extended.chars().take(16).collect();
  line = line + &name;

  let pid: String = (proc.pid.to_string() + "        ").chars().take(8).collect();
  line = line + " " + &pid;

  let vm_rss: String = (proc.status.vm_rss.to_string() + "                ").chars().take(16).collect();
  line = line + " " + &vm_rss;

  let vm_swap: String = (proc.status.vm_swap.to_string() + "                ").chars().take(16).collect();
  line = line + " " + &vm_swap;

  let vm_sum: String = ((proc.status.vm_rss + proc.status.vm_swap).to_string() + "                ").chars().take(16).collect();
  line = line + " " + &vm_sum;

  line = line + "\n";

  terminal::print_line(line);
}

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

  for proc in procs_vec {
    print_line(&proc);
  }

  Result::Ok(())
}

fn main() {
  terminal::init();

  loop {
    terminal::_clear();
    terminal::print_header();
    match do_reading() {
      Err(err) => println!("{}", err),
      _ => ()
    }
    terminal::_refresh();

    let key_option = terminal::wait_key();
    if let Some(_) = key_option {
      break;
    }
  }

  terminal::deinit(); // TODO: Make sure this gets called
}

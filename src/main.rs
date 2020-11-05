mod proc;
mod parsers;

use std::fs::read_dir;

use std::string::String;

use proc::Proc;

use parsers::get_proc;

fn print_header() {
  // 16, 8, 16, 16
  println!("Name             PID      RSS              Swap            >Sum             ");
}

fn print_line(proc: &Proc) {
  let name_extended: String = proc.status.name.clone() + "                ";
  let name: String = name_extended.chars().take(16).collect();
  print!("{} ", name);
  //let remaining = 17 - name.len();
  // let pid_extended;
  let pid: String = (proc.pid.to_string() + "        ").chars().take(8).collect();
  print!("{} ", pid);
  // let rss_extended;
  let vm_rss: String = (proc.status.vm_rss.to_string() + "                ").chars().take(16).collect();
  print!("{} ", vm_rss);
  let vm_swap: String = (proc.status.vm_swap.to_string() + "                ").chars().take(16).collect();
  print!("{} ", vm_swap);
  let vm_sum: String = ((proc.status.vm_rss + proc.status.vm_swap).to_string() + "                ").chars().take(16).collect();
  println!("{} ", vm_sum);
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

  print_header();
  for proc in procs_vec {
    print_line(&proc);
  }

  Result::Ok(())
}

fn main() {
  // for _ in 0..100 {
    match do_reading() {
      Err(err) => println!("{}", err),
      _ => println!("Finish!")
    }
  // }
}

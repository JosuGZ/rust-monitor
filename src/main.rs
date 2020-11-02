// use std::fs::File;
use std::path::Path;
// use std::fs::DirBuilder;
use std::fs::DirEntry;
use std::fs::read_dir;
use std::fs::read_to_string;

use std::string::String;

#[cfg(test)]
static STATUS_EXAMPLE_1: &str = "Name:	kworker/0:0-events
Umask:	0000
State:	I (idle)
Tgid:	27161
Ngid:	0
Pid:	27161
PPid:	2
TracerPid:	0
Uid:	0	0	0	0
Gid:	0	0	0	0
FDSize:	64
Groups:
NStgid:	27161
NSpid:	27161
NSpgid:	0
NSsid:	0
Threads:	1
SigQ:	0/46445
SigPnd:	0000000000000000
ShdPnd:	0000000000000000
SigBlk:	0000000000000000
SigIgn:	ffffffffffffffff
SigCgt:	0000000000000000
CapInh:	0000000000000000
CapPrm:	0000003fffffffff
CapEff:	0000003fffffffff
CapBnd:	0000003fffffffff
CapAmb:	0000000000000000
NoNewPrivs:	0
Seccomp:	0
Speculation_Store_Bypass:	thread vulnerable
Cpus_allowed:	01
Cpus_allowed_list:	0
Mems_allowed:	00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000001
Mems_allowed_list:	0
voluntary_ctxt_switches:	127195
nonvoluntary_ctxt_switches:	5";

#[cfg(test)]
static STATUS_EXAMPLE_2: &str = "Name:	dropbox
Umask:	0002
State:	S (sleeping)
Tgid:	24104
Ngid:	0
Pid:	24104
PPid:	1
TracerPid:	0
Uid:	1000	1000	1000	1000
Gid:	1000	1000	1000	1000
FDSize:	256
Groups:	4 24 27 30 46 112 127 999 1000
NStgid:	24104
NSpid:	24104
NSpgid:	1641
NSsid:	1641
VmPeak:	 3393164 kB
VmSize:	 3326428 kB
VmLck:	       0 kB
VmPin:	       0 kB
VmHWM:	  577716 kB
VmRSS:	  537500 kB
RssAnon:	  471628 kB
RssFile:	   65868 kB
RssShmem:	       4 kB
VmData:	 1124308 kB
VmStk:	     140 kB
VmExe:	    9056 kB
VmLib:	   91264 kB
VmPTE:	    2112 kB
VmSwap:	       0 kB
HugetlbPages:	       0 kB
CoreDumping:	0
THP_enabled:	1
Threads:	86
SigQ:	0/46445
SigPnd:	0000000000000000
ShdPnd:	0000000000000000
SigBlk:	0000000000000000
SigIgn:	0000000001001000
SigCgt:	00000001800004e8
CapInh:	0000000000000000
CapPrm:	0000000000000000
CapEff:	0000000000000000
CapBnd:	0000003fffffffff
CapAmb:	0000000000000000
NoNewPrivs:	0
Seccomp:	0
Speculation_Store_Bypass:	thread vulnerable
Cpus_allowed:	ff
Cpus_allowed_list:	0-7
Mems_allowed:	00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000001
Mems_allowed_list:	0
voluntary_ctxt_switches:	105568
nonvoluntary_ctxt_switches:	1919";

// TODO: Eliminado "Copy"
#[derive(Clone, Debug, PartialEq)]
struct Status {
  name: String,
  vm_peack: u64,
  vm_size: u64,
  vm_lck: u64,
  vm_pin: u64,
  vm_hwm: u64,
  vm_rss: u64,
  rss_anon: u64,
  rss_file: u64,
  rss_shmem: u64,
  vm_data: u64,
  vm_stk: u64,
  vm_exe: u64,
  vm_lib: u64,
  vm_pte: u64,
  vm_swap: u64
}



fn parse_status(file_content: &str) -> Option<Status> {
  fn get_value(name: &str, line: &str) -> Option<u64> {
    if line.contains(name) {
      let mut parts = line.split_whitespace();
      let value = match parts.nth(1) {
        Some(value) => value,
        _ => return None
      };
      match u64::from_str_radix(value, 10) {
        Ok(value) => return Some(value * 1024), // Assuming kB
        _ => return None
      };
    } else {
      return None
    };
  }
  fn get_value_str(name: &str, line: &str) -> Option<String> {
    if line.contains(name) {
      let mut parts = line.split_whitespace();
      let value_str = match parts.nth(1) {
        Some(value) =>  value,
        _ => return None
      };
      return Some(String::from(value_str));
    } else {
      return None
    };
  }

  let mut lines = file_content.split("\n");

  let name;
  let mut vm_peack = 0;
  let mut vm_size = 0;
  let mut vm_lck = 0;
  let mut vm_pin = 0;
  let mut vm_hwm = 0;
  let mut vm_rss = 0;
  let mut rss_anon = 0;
  let mut rss_file = 0;
  let mut rss_shmem = 0;
  let mut vm_data = 0;
  let mut vm_stk = 0;
  let mut vm_exe = 0;
  let mut vm_lib = 0;
  let mut vm_pte = 0;
  let mut vm_swap = 0;


  let first_line = match lines.next() {
    Some(line) => line,
    None => ""
  };
  if let Some(value_str) = get_value_str("Name:", first_line) {
    name = value_str;
  } else {
    return None;
  }

  for line in lines {
    if let Some(value) = get_value("VmPeak:", line) {
      vm_peack = value;
    }
    if let Some(value) = get_value("VmSize:", line) {
      vm_size = value;
    }
    if let Some(value) = get_value("VmLck:", line) {
      vm_lck = value;
    }
    if let Some(value) = get_value("VmPin:", line) {
      vm_pin = value;
    }
    if let Some(value) = get_value("VmHWM:", line) {
      vm_hwm = value;
    }
    if let Some(value) = get_value("VmRSS:", line) {
      vm_rss = value;
    }
    if let Some(value) = get_value("RssAnon:", line) {
      rss_anon = value;
    }
    if let Some(value) = get_value("RssFile:", line) {
      rss_file = value;
    }
    if let Some(value) = get_value("RssShmem:", line) {
      rss_shmem = value;
    }
    if let Some(value) = get_value("VmData:", line) {
      vm_data = value;
    }
    if let Some(value) = get_value("VmStk:", line) {
      vm_stk = value;
    }
    if let Some(value) = get_value("VmExe:", line) {
      vm_exe = value;
    }
    if let Some(value) = get_value("VmLib:", line) {
      vm_lib = value;
    }
    if let Some(value) = get_value("VmPTE:", line) {
      vm_pte = value;
    }
    if let Some(value) = get_value("VmSwap:", line) {
      vm_swap = value;
    }
  }

  let result = Status {
    name,
    vm_peack,
    vm_size,
    vm_lck,
    vm_pin,
    vm_hwm,
    vm_rss,
    rss_anon,
    rss_file,
    rss_shmem,
    vm_data,
    vm_stk,
    vm_exe,
    vm_lib,
    vm_pte,
    vm_swap
  };

  return Some(result);
}

#[test]
fn parse_status_1() {
  let expected = Some(Status {
    name: "kworker/0:0-events".to_string(),
    vm_peack: 0,
    vm_size: 0,
    vm_lck: 0,
    vm_pin: 0,
    vm_hwm: 0,
    vm_rss: 0,
    rss_anon: 0,
    rss_file: 0,
    rss_shmem: 0,
    vm_data: 0,
    vm_stk: 0,
    vm_exe: 0,
    vm_lib: 0,
    vm_pte: 0,
    vm_swap: 0
  });

  let status = parse_status(STATUS_EXAMPLE_1);

  assert_eq!(expected, status);
}

#[test]
fn parse_status_2() {
  let expected = Some(Status {
    name: "dropbox".to_string(),
    vm_peack: 3393164 * 1024,
    vm_size: 3326428 * 1024,
    vm_lck: 0 * 1024,
    vm_pin: 0 * 1024,
    vm_hwm: 577716 * 1024,
    vm_rss: 537500 * 1024,
    rss_anon: 471628 * 1024,
    rss_file: 65868 * 1024,
    rss_shmem: 4 * 1024,
    vm_data: 1124308 * 1024,
    vm_stk: 140 * 1024,
    vm_exe: 9056 * 1024,
    vm_lib: 91264 * 1024,
    vm_pte: 2112 * 1024,
    vm_swap: 0 * 1024
  });

  let status = parse_status(STATUS_EXAMPLE_2);

  assert_eq!(expected, status);
}
#[test]
fn parse_status_3() {
  let expected = None;

  let status = parse_status("");

  assert_eq!(expected, status);
}

struct Proc {
  pid: i32,
  cmdline: String,
  status: Status
}

fn get_proc(ref entry: DirEntry) -> Option<Proc> {

  let name = match entry.file_name().into_string() {
    Ok(s) => s,
    _ => return None
  };

  let pid = match name.parse::<i32>() {
    Ok(pid) => pid,
    _ => return None
  };

  let cmd_line_path = "/proc/".to_string() + &name + "/cmdline";
  let path = Path::new(&cmd_line_path);
  let cmdline_result = read_to_string(path);

  let cmdline;
  //println!("Opening {}...", cmd_line_path);
  if let Ok(cmdline_ok) = cmdline_result {
    cmdline = cmdline_ok;
  } else if let Err(e) = cmdline_result {
    //println!("{:?}", e);
    return None;
  } else {
    panic!();
  }

  let status_path = "/proc/".to_string() + &name + "/status";
  let status_result = read_to_string(status_path);

  let status_string;
  if let Ok(value) = status_result {
    status_string = value;
  } else if let Err(e) = status_result {
    println!("{:?}", e);
    return None;
  } else {
    panic!();
  }

  let status = match parse_status(&status_string) {
    Some(status) => status,
    None => return None
  };

  let proc = Proc {
    pid,
    cmdline,
    status
  };

  // Here we are sure we have a number, now we check if it is a process
  return Some(proc);
}

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

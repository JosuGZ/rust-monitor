// mod parsers;

#[cfg(test)]
mod tests;

use std::path::Path;
use std::fs::DirEntry;
use std::fs::read_to_string;

use std::str::FromStr;

use super::proc::Status;
use super::proc::Proc;
use super::proc::Uptime;
use super::proc::MemInfo;

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
      let mut value_str = match parts.nth(1) {
        Some(value) =>  value.to_string(),
        _ => return None
      };
      for part in parts {
        value_str += " ";
        value_str += part;
      }
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

pub fn get_proc(ref entry: DirEntry) -> Option<Proc> {

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
  } else if let Err(_) = cmdline_result {
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

fn parse_uptime(uptime: &str) -> Uptime {
  let mut bits = uptime.split_whitespace();

  let up_str = bits.next().unwrap();
  let up = f64::from_str(up_str).unwrap();

  let idle_str = bits.next().unwrap();
  let idle = f64::from_str(idle_str).unwrap();

  return Uptime {
    up: up,
    idle: idle
  };
}

pub fn get_uptime() -> Uptime {
  let uptime_path = Path::new("/proc/uptime");
  let uptime = read_to_string(uptime_path).unwrap();
  return parse_uptime(&uptime);
}

fn parse_mem_info(mem_info_str: &str) -> MemInfo {
  let mut mem_info = MemInfo {
    mem_total: 0,
    mem_free: 0,
    mem_available: 0,
    swap_total: 0,
    swap_free: 0
  };

  let lines = mem_info_str.split("\n");

  for line in lines {
    let mut parts = line.split_whitespace();

    let key = parts.next();
    let value_str = parts.next();

    match (key, value_str) {
      (Some(key), Some(value_str)) => {
        let value = u64::from_str(value_str);

        match key {
          "MemTotal:" => mem_info.mem_total = value.unwrap() * 1024,
          "MemFree:" => mem_info.mem_free = value.unwrap() * 1024,
          "MemAvailable:" => mem_info.mem_available = value.unwrap() * 1024,
          "SwapTotal:" => mem_info.swap_total = value.unwrap() * 1024,
          "SwapFree:" => mem_info.swap_free = value.unwrap() * 1024,
          _ => {}
        }
      },
      _ => {}
    }
  }

  return mem_info;
}

pub fn get_mem_info() -> MemInfo {
  let uptime_path = Path::new("/proc/meminfo");
  let mem_info = read_to_string(uptime_path).unwrap();
  return parse_mem_info(&mem_info);
}

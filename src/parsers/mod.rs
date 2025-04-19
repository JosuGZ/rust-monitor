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
use super::proc::VmStat;
use super::proc::Stat;
use super::proc::IoStats;
use crate::proc::CpuInfo;

fn get_value(name: &str, line: &str) -> Option<u64> {
  if !line.starts_with(name) { return None; }

  let mut parts = line.split_whitespace();
  let value = match parts.nth(1) {
    Some(value) => value,
    _ => return None
  };

  match u64::from_str(value) {
    Ok(value) => Some(value),
    _ => None
  }
}

fn get_value_str(name: &str, line: &str) -> Option<String> {
  if !line.starts_with(name) { return None; }

  let mut parts = line.split_whitespace();
  let mut value_str = match parts.nth(1) {
    Some(value) =>  value.to_string(),
    _ => return None
  };
  for part in parts {
    value_str += " ";
    value_str += part;
  }

  Some(value_str)
}

fn parse_status(file_content: &str) -> Option<Status> {
  let mut lines = file_content.split('\n');

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


  let first_line = lines.next().unwrap_or("");
  if let Some(value_str) = get_value_str("Name:", first_line) {
    name = value_str;
  } else {
    return None;
  }

  for line in lines {
    if let Some(value) = get_value("VmPeak:", line) {
      vm_peack = value * 1024;
    }
    else if let Some(value) = get_value("VmSize:", line) {
      vm_size = value * 1024;
    }
    else if let Some(value) = get_value("VmLck:", line) {
      vm_lck = value * 1024;
    }
    else if let Some(value) = get_value("VmPin:", line) {
      vm_pin = value * 1024;
    }
    else if let Some(value) = get_value("VmHWM:", line) {
      vm_hwm = value * 1024;
    }
    else if let Some(value) = get_value("VmRSS:", line) {
      vm_rss = value * 1024;
    }
    else if let Some(value) = get_value("RssAnon:", line) {
      rss_anon = value * 1024;
    }
    else if let Some(value) = get_value("RssFile:", line) {
      rss_file = value * 1024;
    }
    else if let Some(value) = get_value("RssShmem:", line) {
      rss_shmem = value * 1024;
    }
    else if let Some(value) = get_value("VmData:", line) {
      vm_data = value * 1024;
    }
    else if let Some(value) = get_value("VmStk:", line) {
      vm_stk = value * 1024;
    }
    else if let Some(value) = get_value("VmExe:", line) {
      vm_exe = value * 1024;
    }
    else if let Some(value) = get_value("VmLib:", line) {
      vm_lib = value * 1024;
    }
    else if let Some(value) = get_value("VmPTE:", line) {
      vm_pte = value * 1024;
    }
    else if let Some(value) = get_value("VmSwap:", line) {
      vm_swap = value * 1024;
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

  Some(result)
}

fn parse_stat(file_content: &str) -> Option<Stat> {
  // Name is the second element
  // TODO: in case of error, display something
  let contents = file_content.split(')').nth(1)?.split_whitespace();

  let mut contents = contents.skip(11);
  Some(Stat {
    utime: contents.next()?.parse().ok()?,
    stime: contents.next()?.parse().ok()?
  })
}

fn parse_io(file_content: &str) -> Option<IoStats> {
  let mut io_stats = IoStats::default();
  let lines = file_content.split('\n');

  for line in lines {
    if let Some(value) = get_value("read_bytes:", line) {
      io_stats.read_bytes = value;
    }
    else if let Some(value) = get_value("write_bytes:", line) {
      io_stats.write_bytes = value;
    }
  }

  Some(io_stats)
}

pub fn get_proc(entry: &DirEntry) -> Option<Proc> {

  let name = entry.file_name().into_string().ok()?;
  let pid = name.parse::<i32>().ok()?;

  let cmd_line_path = format!("/proc/{name}/cmdline");
  let cmdline = read_to_string(cmd_line_path).ok()?;

  let status_path = format!("/proc/{name}/status");
  let status_string = read_to_string(status_path).ok()?;

  let stat_path = format!("/proc/{name}/stat");
  let stat_string = read_to_string(stat_path).ok()?;

  let io_path = format!("/proc/{name}/io");
  let io_string = read_to_string(io_path).ok()?;

  let status = parse_status(&status_string)?;
  let stat = parse_stat(&stat_string)?;
  let io = parse_io(&io_string)?;

  let proc = Proc {
    pid,
    count: 1,
    cmdline,
    status,
    stat,
    io,
    new: false,
    deleted: false
  };

  // Here we are sure we have a number, now we check if it is a process
  Some(proc)
}

fn parse_uptime(uptime: &str) -> Uptime {
  let mut bits = uptime.split_whitespace();

  let up_str = bits.next().unwrap();
  let up = f64::from_str(up_str).unwrap();

  let idle_str = bits.next().unwrap();
  let idle = f64::from_str(idle_str).unwrap();

  Uptime { up, idle }
}

pub fn get_uptime() -> Uptime {
  let uptime_path = Path::new("/proc/uptime");
  let uptime = read_to_string(uptime_path).unwrap();
  parse_uptime(&uptime)
}

fn parse_mem_info(mem_info_str: &str) -> MemInfo {
  let mut mem_info = MemInfo {
    mem_total: 0,
    mem_free: 0,
    mem_available: 0,
    swap_total: 0,
    swap_free: 0
  };

  let lines = mem_info_str.split('\n');

  for line in lines {
    let mut parts = line.split_whitespace();

    let key = parts.next();
    let value = parts.next().map(|value_str| {
      u64::from_str(value_str)
    });

    match (key, value) {
      (Some("MemTotal:"), Some(value)) => mem_info.mem_total = value.unwrap() * 1024,
      (Some("MemFree:"), Some(value)) => mem_info.mem_free = value.unwrap() * 1024,
      (Some("MemAvailable:"), Some(value)) => mem_info.mem_available = value.unwrap() * 1024,
      (Some("SwapTotal:"), Some(value)) => mem_info.swap_total = value.unwrap() * 1024,
      (Some("SwapFree:"), Some(value)) => mem_info.swap_free = value.unwrap() * 1024,
      _ => {}
    }
  }

  mem_info
}

pub fn get_mem_info() -> MemInfo {
  let uptime_path = Path::new("/proc/meminfo");
  let mem_info = read_to_string(uptime_path).unwrap();
  parse_mem_info(&mem_info)
}

fn parse_vm_stat(file_content: &str) -> VmStat {
  let lines = file_content.split('\n');

  let mut vmstat = VmStat::default();

  // let first_line = lines.next().unwrap_or("");
  // if let Some(value_str) = get_value_str("Name:", first_line) {
  //   name = value_str;
  // } else {
  //   return None;
  // }

  for line in lines {
    if let Some(value) = get_value("pswpin", line) {
      vmstat.pswpin = value;
    }
    else if let Some(value) = get_value("pswpout", line) {
      vmstat.pswpout = value;
    }
  }

  vmstat
}

pub fn get_vm_stat() -> VmStat {
  let vmstat_path = Path::new("/proc/vmstat");
  let vmstat = read_to_string(vmstat_path).unwrap();
  parse_vm_stat(&vmstat)
}

pub fn parse_cpu_info(file_content: &str) -> Option<Vec<CpuInfo>> {
  file_content.split_terminator("\n\n").map(|cpu_info| {
    let mut processor = None;
    let mut mhz = None;

    for line in cpu_info.lines() {
      let split = line.split_once(':').map(|v| (v.0.trim(), v.1.trim()));
      if let Some((key, value)) = split {
        match key {
          "processor" => processor = value.parse().ok(),
          "cpu MHz"   => mhz = value.parse().ok(),
          _ => {}
        }
      }
    }

    if let (Some(processor), Some(mhz)) = (processor, mhz) {
      Some(CpuInfo {
        processor, mhz
      })
    } else {
      None
    }
  }).collect()
}

pub fn get_cpu_info() -> Option<Vec<CpuInfo>> {
  let cpu_info_path = Path::new("/proc/cpuinfo");
  let cpu_info = read_to_string(cpu_info_path).unwrap();
  parse_cpu_info(&cpu_info)
}

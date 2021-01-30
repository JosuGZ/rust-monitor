// mod proc;

use std::ops;

#[derive(Clone, Debug, PartialEq)]
pub struct Proc {
  pub pid: i32,
  pub cmdline: String,
  pub status: Status
}

// TODO: Eliminado "Copy"
#[derive(Clone, Debug, PartialEq)]
pub struct Status {
  pub name: String,
  pub vm_peack: u64,
  pub vm_size: u64,
  pub vm_lck: u64,
  pub vm_pin: u64,
  pub vm_hwm: u64,
  pub vm_rss: u64,
  pub rss_anon: u64,
  pub rss_file: u64,
  pub rss_shmem: u64,
  pub vm_data: u64,
  pub vm_stk: u64,
  pub vm_exe: u64,
  pub vm_lib: u64,
  pub vm_pte: u64,
  pub vm_swap: u64
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Uptime {
  pub up: f64,
  pub idle: f64
}

impl ops::Sub<&Uptime> for &Uptime {
  type Output = Uptime;

  fn sub(self, b: &Uptime) -> Self::Output {
    return Uptime {
      up: self.up - b.up,
      idle: self.idle - b.idle
    };
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MemInfo {
  pub mem_total: u64,
  pub mem_free: u64,
  pub mem_available: u64,
  pub swap_total: u64,
  pub swap_free: u64
}

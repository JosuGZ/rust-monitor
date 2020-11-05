// mod proc;

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

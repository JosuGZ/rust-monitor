// mod parsers::tests

use super::*;

static STATUS_EXAMPLE_1: &str = include_str!("./examples/status_example_1.txt");
static STATUS_EXAMPLE_2: &str = include_str!("./examples/status_example_2.txt");

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
    name: "dropbox 2 3 4".to_string(),
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

#[test]
fn parse_stat_1() {
  const STAT_EXAMPLE_1: &str = include_str!("./examples/stat_1.txt");
  let expected = Some(Stat {
    utime: 14,
    stime: 15
  });

  let result = parse_stat(STAT_EXAMPLE_1);

  assert_eq!(expected, result);
}

#[test]
fn parse_uptime_1() {
  let expected = Uptime {
    up: 2978723.18_f64,
    idle: 18677515.22_f64
  };

  let uptime = parse_uptime("2978723.18 18677515.22");

  assert_eq!(expected, uptime);
}

static MEM_INFO_EXAMPLE_1: &str = include_str!("./examples/mem_info_example_1.txt");

#[test]
fn parse_mem_info_1() {
  let expected = MemInfo {
    mem_total: 6 * 1024,
    mem_free: 2* 1024,
    mem_available: 3 * 1024,
    swap_total: 1024 * 1024,
    swap_free: 512 * 1024
  };

  let uptime = parse_mem_info(MEM_INFO_EXAMPLE_1);

  assert_eq!(expected, uptime);
}


static VMSTAT_EXAMPLE: &str = include_str!("./examples/vmstat.txt");

#[test]
fn test_parse_vm_stat() {
  let expected = VmStat {
    pswpin: 174385139,
    pswpout: 223337038
  };

  let vmstat = parse_vm_stat(VMSTAT_EXAMPLE);

  assert_eq!(expected, vmstat);
}

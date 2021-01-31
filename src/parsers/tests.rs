// mod parsers::tests

use super::*;

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

static STATUS_EXAMPLE_2: &str = "Name:	dropbox 2 3 4
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
fn parse_uptime_1() {
  let expected = Uptime {
    up: 2978723.18_f64,
    idle: 18677515.22_f64
  };

  let uptime = parse_uptime("2978723.18 18677515.22");

  assert_eq!(expected, uptime);
}

static MEM_INFO_EXAMPLE_1: &str = "MemTotal:              6 kB
MemFree:               2 kB
MemAvailable:          3 kB
SwapTotal:          1024 kB
SwapFree:            512 kB";

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

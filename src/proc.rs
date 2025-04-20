use std::ops::{AddAssign, SubAssign, Sub};

/// Stores either a process, or an aggregation of a group of processes
///
/// Adding two of this will convert them in a group, adding the internal values.
/// In this case, values like pid or cmdline become useles.
///
/// Subtracting two `Proc` will yied the difference, useful to know how the data
/// has changed over time (TODO)
///
/// TODO: Both sume and subtract are only implemented for the fields I'm
/// interested in, in order to use more fields, this has to be reviewed
#[derive(Clone, Debug, PartialEq)]
pub struct Proc {
  /// Number of processes in a group
  pub count: i32,
  pub pid: i32,
  pub cmdline: String,
  pub status: Status,
  pub stat: Stat,
  pub io: IoStats,
  pub new: bool,
  pub deleted: bool
}

impl AddAssign for Proc {

  fn add_assign(&mut self, rhs: Self) {
    self.new = self.new || rhs.new;
    self.deleted = self.deleted || rhs.deleted;

    if !rhs.deleted {
      self.count += rhs.count;

      self.status.vm_rss += rhs.status.vm_rss;
      self.status.vm_swap += rhs.status.vm_swap;

      self.stat += rhs.stat;
      self.io += rhs.io;
    }
  }
}

impl SubAssign for Proc {

  /// When subtracting we don't subtrackt most metrics, only CPU etc
  fn sub_assign(&mut self, rhs: Self) {
    self.stat -= rhs.stat;
    self.io -= rhs.io;
  }
}

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

impl Sub<&Uptime> for &Uptime {
  type Output = Uptime;

  fn sub(self, b: &Uptime) -> Self::Output {
    Uptime {
      up: self.up - b.up,
      idle: self.idle - b.idle
    }
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

#[derive(Clone, Debug, PartialEq, Default)]
pub struct VmStat {
  pub pswpin: u64,
  pub pswpout: u64
}

impl Sub<&VmStat> for &VmStat {
  type Output = VmStat;

  fn sub(self, rhs: &VmStat) -> VmStat {
    Self::Output {
      pswpin: self.pswpin - rhs.pswpin,
      pswpout: self.pswpout - rhs.pswpout
    }
  }
}

/// Represents the information extracted from the `/proc/<PID>/stat` file.
///
/// https://stackoverflow.com/a/60441542/1971526
///
/// man 5 proc
#[derive(Clone, Debug, PartialEq)]
pub struct Stat {
  /// (14) utime  %lu
  ///
  /// Amount of time that this process has been scheduled in user mode, measured
  /// in clock ticks (divide by sysconf(_SC_CLK_TCK)).  This includes guest
  /// time, guest_time (time spent running a virtual CPU, see below), so that
  /// applications that are not aware of the guest time field do not lose that
  /// time from their calculations.
  pub utime: u64,

  /// (15) stime  %lu
  ///
  /// Amount of time that this process has been scheduled in kernel mode,
  /// measured in clock ticks (divide by sysconf(_SC_CLK_TCK)).
  pub stime: u64,
}

impl AddAssign for Stat {
  fn add_assign(&mut self, rhs: Self) {
    self.utime += rhs.utime;
    self.stime += rhs.stime;
  }
}

impl SubAssign for Stat {

  /// Subtracting `Stat` values is not a typical subtraction. It is meant to
  /// compute CPU derivatives //TODO:
  fn sub_assign(&mut self, rhs: Self) {
    self.utime -= rhs.utime;
    self.stime -= rhs.stime;
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CpuInfo {
  pub processor: usize,
  pub mhz: f32
}

/// IO statistics for a process and its waited-for children, as reported by
/// `/proc/<pid>/io`.
///
/// See man proc(5) for more details.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct IoStats {
  /// The number of bytes returned by successful read(2) and similar system
  /// calls
  pub rchar: u64,
  /// The number of bytes returned by successful write(2) and similar system
  /// calls
  pub wchar: u64,
  /// The number of "file read" system calls - those from the read(2) family,
  /// sendfile(2), copy_file_range(2), and ioctl(2) BTRFS_IOC_ENCODED_READ
  pub syscr: u64,
  /// The number of "file write" system calls - those from the write(2) family,
  /// sendfile(2), copy_file_range(2), and ioctl(2) BTRFS_IOC_ENCODED_WRITE
  pub syscw: u64,
  /// The number of bytes really fetched from the storage layer.
  /// This is accurate for block-backed filesystems.
  pub read_bytes: u64,
  /// The number of bytes really sent to the storage layer
  pub write_bytes: u64,
  /// The number of bytes "saved" from I/O writeback due to truncation.
  /// This can yield to having done negative I/O if caches dirtied by another
  /// process are truncated. This applies to I/O already accounted-for in
  /// write_bytes.
  pub cancelled_write_bytes: u64
}

impl AddAssign for IoStats {
  fn add_assign(&mut self, rhs: Self) {
    self.rchar += rhs.rchar;
    self.wchar += rhs.wchar;
    self.syscr += rhs.syscr;
    self.syscw += rhs.syscw;
    self.read_bytes += rhs.read_bytes;
    self.write_bytes += rhs.write_bytes;
    self.cancelled_write_bytes += rhs.cancelled_write_bytes;
  }
}

impl SubAssign for IoStats {
  fn sub_assign(&mut self, rhs: Self) {
    self.rchar -= rhs.rchar;
    self.wchar -= rhs.wchar;
    self.syscr -= rhs.syscr;
    self.syscw -= rhs.syscw;
    self.read_bytes -= rhs.read_bytes;
    self.write_bytes -= rhs.write_bytes;
    self.cancelled_write_bytes -= rhs.cancelled_write_bytes;
  }
}

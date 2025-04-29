use core::ffi::*;

// ansi/include/bits/ansi/time_t.h

pub type time_t = c_long;

// abis/popcorn/suseconds_t.h

pub type suseconds_t = i64;

// abis/popcorn/rlim_t.h

pub type rlim_t = c_ulonglong;

// ansi/include/time.h

pub type clock_t = c_long;

pub struct tm {
	pub tm_sec: c_int,
	pub tm_min: c_int,
	pub tm_hour: c_int,
	pub tm_mday: c_int,
	pub tm_mon: c_int,
	pub tm_year: c_int,
	pub tm_wday: c_int,
	pub tm_yday: c_int,
	pub tm_isdst: c_int,
	pub tm_gmtoff: c_long,
	pub tm_zone: *const c_char,
}

// popcorn/include/abi-bits/mode_t.h

pub type mode_t = c_int;

// internal/include/bits/wchar_t.h

pub type wchar_t = c_int;

// posix/include/pwd.h

#[repr(C)]
pub struct passwd {
	pub pw_name: *mut c_char,
	pub pw_passwd: *mut c_char,
	pub pw_uid: crate::uid_t,
	pub pw_gid: crate::gid_t,
	pub pw_gecos: *mut c_char,
	pub pw_dir: *mut c_char,
	pub pw_shell: *mut c_char,
}

// posix/include/sys/socket.h

#[repr(C)]
pub struct sockaddr {
	pub sa_family: crate::sa_family_t,
	pub sa_data: [c_char; 14],
}

// abis/popcorn/socklen_t.h

pub type socklen_t = c_uint;

// abis/popcrorn/stat.h

#[repr(C)]
pub struct stat {
	pub st_dev: crate::dev_t,
	pub st_ino: crate::ino_t,
	pub st_mode: crate::mode_t,
	pub st_nlink: crate::nlink_t,
	pub st_uid: crate::uid_t,
	pub st_gid: crate::gid_t,
	pub st_rdev: crate::dev_t,
	pub st_size: crate::off_t,
	pub st_atim: crate::timespec,
	pub st_mtim: crate::timespec,
	pub st_ctim: crate::timespec,
	pub st_btim: crate::timespec,
	pub st_blksize: crate::blksize_t,
	pub st_blocks: crate::blkcnt_t,
}

// posix/include/dirent.h

#[repr(C)]
pub struct dirent {
	pub d_ino: crate::ino_t,
	pub d_off: crate::off_t,
	pub d_reclen: c_ushort,
	pub d_type: c_char,
	pub d_name: [c_char; 1024],
}

// abis/popcorn/termios.h

pub const NCCS: usize = 11;

#[repr(C)]
pub struct termios {
	pub c_iflag: crate::tcflag_t,
	pub c_oflag: crate::tcflag_t,
	pub c_cflag: crate::tcflag_t,
	pub c_lflag: crate::tcflag_t,
	pub c_cc: [crate::cc_t; NCCS],
	pub ibaud: crate::speed_t,
	pub obaud: crate::speed_t,
};

pub type speed_t = c_uint;
pub type cc_t = c_uint;
pub type tcflag_t = c_uint;

// abis/popcorn/signal.h

pub type sigset_t = c_long;

#[repr(C)]
pub struct sigaction {
	pub sa_sigaction: crate::sighandler_t,
	pub sa_restorer: Option<extern "C" fn()>,
	pub sa_mask: sigset_t,
	pub sa_flags: c_int,
};

// posix/include/bits/posix/fd_set.h

#[repr(C)]
pub fd_set {
	pub fds_bits: [u8; 128],
}

// internal/include/bits/off_t.h

pub type off_t = c_long;
pub type off64_t = c_long;

// posix/include/dlfcn.h

#[repr(C)]
pub struct Dl_info {
	pub dli_fname: *const c_char,
	pub dli_fbase: *mut c_void,
	dli_sname: *const c_char,
	dli_saddr: *mut c_void,
}

// abis/popcorn/statvfs.h

#[repr(C)]
pub struct statvfs {
	pub f_bsize: c_ulong,
	pub f_frsize: c_ulong,
	pub f_blocks: crate::fsblkcnt_t,
	pub f_bfree: crate::fsblkcnt_t,
	pub f_bavail: crate::fsblkcnt_t,

	pub f_files: crate::fsfilcnt_t,
	pub f_ffree: crate::fsfilcnt_t,
	pub f_favail: crate::fsfilcnt_t,

	pub f_fsid: c_ulong,
	pub f_flag: c_ulong,
	pub f_namemax: c_ulong,
	pub f_basetype: [c_char; 80],
}

// posix/include/semaphore.h

#[repr(C)]
pub struct sem_t {
	__mlibc_count: c_uint_
}

// ansi/include/locale.h

#[repr(C)]
pub struct lconv {
	pub decimal_point: *mut c_char,
	pub thousands_sep: *mut c_char,
	pub grouping: *mut c_char,
	pub mon_decimal_point: *mut c_char,
	pub mon_thousands_sep: *mut c_char,
	pub mon_grouping: *mut c_char,
	pub positive_sign: *mut c_char,
	pub negative_sign: *mut c_char,
	pub currency_symbol: *mut c_char,
	pub frac_digits: c_char,
	pub p_cs_precedes: c_char,
	pub n_cs_precedes: c_char,
	pub p_sep_by_space: c_char,
	pub n_sep_by_space: c_char,
	pub p_sign_posn: c_char,
	pub n_sign_posn: c_char,
	pub int_curr_symbol: *mut c_char,
	pub int_frac_digits: c_char,
	pub int_p_cs_precedes: c_char,
	pub int_n_cs_precedes: c_char,
	pub int_p_sep_by_space: c_char,
	pub int_n_sep_by_space: c_char,
	pub int_p_sign_posn: c_char,
	pub int_n_sign_posn: c_char,
};

// posix/include/sys/poll.h

pub type nfds_t = usize;


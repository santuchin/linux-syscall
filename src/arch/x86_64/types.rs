
pub type c_void = core::ffi::c_void;

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_char = c_uchar; // since linux 6.2, it is compiled with -funsigned-char flag

pub type c_short = i16;
pub type c_ushort = u16;

pub type c_int = i32;
pub type c_uint = u32;

pub type c_long = i64;
pub type c_ulong = u64;

pub type c_longlong = i64;
pub type c_ulonglong = u64;

pub type size_t = usize;
pub type ssize_t = isize;

pub type off_t = c_long;

pub type mode_t = c_ushort;

pub type pid_t = c_int;

pub type socklen_t = c_uint;

pub type sa_family_t = c_ushort;

pub type __u64 = u64;
pub type __u32 = u32;

#[repr(C)]
pub struct open_how {
	pub flags: __u64,
	pub mode: __u64,
	pub resolve: __u64,
}

#[repr(C)]
pub struct sockaddr {
	pub sa_family: sa_family_t,
	pub sa_data: [c_char; 14],
}






#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct io_uring_params {
    pub sq_entries: __u32,
    pub cq_entries: __u32,
    pub flags: __u32,
    pub sq_thread_cpu: __u32,
    pub sq_thread_idle: __u32,
    pub features: __u32,
    pub wq_fd: __u32,
    pub resv: [__u32; 3usize],
    pub sq_off: io_sqring_offsets,
    pub cq_off: io_cqring_offsets,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct io_sqring_offsets {
    pub head: __u32,
    pub tail: __u32,
    pub ring_mask: __u32,
    pub ring_entries: __u32,
    pub flags: __u32,
    pub dropped: __u32,
    pub array: __u32,
    pub resv1: __u32,
    pub resv2: __u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct io_cqring_offsets {
    pub head: __u32,
    pub tail: __u32,
    pub ring_mask: __u32,
    pub ring_entries: __u32,
    pub overflow: __u32,
    pub cqes: __u32,
    pub resv1: __u64,
    pub resv2: __u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct io_uring_cqe {
	pub user_data: u64,
	pub res: i32,
	pub flags: u32,
	pub big_cqe: *mut u64,
}



#[derive(Debug, Clone)]
pub struct stat {
    // TODO
}

#[derive(Debug, Clone)]
pub struct rusage {
    // TODO
}



pub type __u8  = u8;
pub type __u16 = u16;
pub type __s32 = i32;
pub type __kernel_rwf_t = u32;

#[repr(C)]
pub union io_uring_sqe_flags_union {
    pub rw_flags: __kernel_rwf_t,
    pub fsync_flags: __u32,
    pub poll_events: __u16,
    pub sync_range_flags: __u32,
    pub msg_flags: __u32,
    pub timeout_flags: __u32,
    pub accept_flags: __u32,
    pub cancel_flags: __u32,
    pub open_flags: __u32,
    pub statx_flags: __u32,
    pub fadvise_advice: __u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct io_uring_sqe_buf_union {
    pub buf_index: __u16,
    pub personality: __u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union io_uring_sqe_union {
    pub buf: io_uring_sqe_buf_union,
    pub __pad2: [__u64; 3],
}

#[repr(C)]
pub struct io_uring_sqe {
    pub opcode: __u8,
    pub flags: __u8,
    pub ioprio: __u16,
    pub fd: __s32,
    pub off: __u64,
    pub addr: __u64,
    pub len: __u32,
    pub union1: io_uring_sqe_flags_union,
    pub user_data: __u64,
    pub u: io_uring_sqe_union,
}

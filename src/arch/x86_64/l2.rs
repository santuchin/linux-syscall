
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::l1::*;

use libc::{
	c_char as char,
	c_int as int,
	c_long as long,
	c_uint as uint,
	mode_t,
	size_t,
	ssize_t
};

pub use std::os::fd::RawFd;

macro_rules! catch {
	($error:expr, $valid:expr) => {
		
	};
}






pub unsafe fn read<T>(
	file_desc: RawFd,
	buffer: *mut T,
	size: size_t,
) -> ssize_t {
	unsafe {
		syscall!(
			sys::READ,
			file,
			buffer,
			size
		)
	}
}

pub unsafe fn write<T>(
	file: RawFd,
	data: *const T,
	length: size_t,
) -> ssize_t {
	unsafe {
		syscall!(
			sys::WRITE,
			file,
			data,
			length
		)
	}
}

pub unsafe fn close(file: RawFd) -> int {
	unsafe { syscall!( sys::CLOSE, file) }
}

pub fn getpid() -> int {
	unsafe { syscall!(sys::GETPID) }
}

pub fn exit(status: int) -> ! {
	unsafe { syscall!(sys::EXIT, status) };
	unsafe { core::hint::unreachable_unchecked() }
}


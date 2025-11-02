
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


macro_rules! catch {
	($error:expr, $valid:expr) => {
		
	};
}



pub unsafe fn read<T>(
	file: int,
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
	file: int,
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

/*
#[deprecated(note = "discouraged, use openat instead")]
pub fn open(
	path: *const char,
	options: OpenOptions,
) -> Result<int, Error> {

	let value = unsafe {
		syscall!(
			sys::OPEN,
			options.flags,
			options.mode
		)
	};

	if let ERROR_RESERVED..0 = value {
		Err(-value as _)
	} else {
		Ok(value as _)
	}
}
*/

pub unsafe fn close(file: int) -> int {
	unsafe { syscall!( sys::CLOSE, file) }
}

pub fn getpid() -> int {
	unsafe { syscall!(sys::GETPID) }
}

pub unsafe fn socket(
	domain: int,
	r#type: int,
	protocol: int,
) -> int {
	unsafe { syscall!(sys::SOCKET, domain, r#type, protocol) }
}

pub fn exit(status: int) -> ! {
	unsafe { syscall!(sys::EXIT, status) };
	unsafe { core::hint::unreachable_unchecked() }
}


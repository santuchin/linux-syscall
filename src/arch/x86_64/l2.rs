
#![allow(dead_code)]
#![allow(unused_imports)]

pub use std::os::fd::RawFd;

use crate::l1::{
	syscall,
};
use crate::Sys;
use crate::types;
use crate::result::Result;


use libc::{

	c_void,

	
	c_char,
	c_schar,
	c_uchar,

	c_short,
	c_ushort,

	c_int,
	c_uint,

	c_long,
	c_ulong,

	c_longlong,
	c_ulonglong,


	c_float,

	c_double,

	
	size_t,
	ssize_t,


	mode_t,
};





pub fn pause() -> Result {
	unsafe { syscall!(Sys::Pause) }
}

pub fn scheduler_yield() -> Result {
	unsafe { syscall!(Sys::SchedulerYield) }
}

pub fn get_process_id() -> Result {
	unsafe { syscall!(Sys::GetProcessId) }
}

pub fn get_parent_process_id() -> Result {
	unsafe { syscall!(Sys::GetParentProcessId) }
}

pub fn get_group_id() -> Result {
	unsafe { syscall!(Sys::GetGroupId) }
}

pub fn get_user_id() -> Result {
	unsafe { syscall!(Sys::GetUserId) }
}

pub fn get_thread_id() -> Result {
	unsafe { syscall!(Sys::GetThreadId) }
}


pub unsafe fn read<T>(
	file_desc: RawFd,
	buffer: *mut T,
	size: usize,
) -> Result {
	unsafe {
		syscall!(
			Sys::Read,
			file_desc,
			buffer,
			size,
		)
	}
}


pub fn accept(
	file_desc: RawFd,
	address: *mut libc::sockaddr,
	address_length: *mut size_t,
) -> Result {
	unsafe {
		syscall!(
			Sys::Accept,
			file_desc,
			address,
			address_length,
		)
	}
}


pub unsafe fn socket(
	domain: c_int,
	semantics: c_int,
	protocol: c_int,
) -> Result {
	unsafe {
		syscall!(
			Sys::Socket,
			domain,
			semantics,
			protocol
		)
	}
}

pub unsafe fn write<T>(
	file_desc: RawFd,
	data: *const T,
	length: size_t,
) -> Result {
	unsafe {
		syscall!(
			Sys::Write,
			file_desc,
			data,
			length,
		)
	}
}

pub unsafe fn close(file_desc: c_int) -> Result {
	unsafe { syscall!(Sys::Close, file_desc) }
}



pub unsafe fn exit(status: c_int) -> Result {
	unsafe { syscall!(Sys::Exit, status) }
}

pub unsafe fn long_seek(
	file_desc: RawFd,
	offset: libc::off_t,
	whence: c_uint,
) -> Result {
	unsafe {
		syscall!(
			Sys::LongSeek,
			offset,
			whence,
		)
	}
}

pub unsafe fn open(
	filename: *const c_char,
	flags: c_int,
	mode: mode_t,
) -> Result {
	unsafe {
		syscall!(
			Sys::Open,
			filename,
			flags,
			mode,
		)
	}
}

pub unsafe fn openat(
	dir_file_desc: RawFd,
	filename: *const c_char,
	flags: c_int,
	mode: mode_t,
) -> Result {
	syscall!(
		Sys::OpenAt,
		dir_file_desc,
		filename,
		flags,
		mode,
	)
}

pub unsafe fn openat2(
	dir_file_desc: RawFd,
	filename: *const char,
	open_how: *const types::OpenHow,
	size: size_t,
) -> Result {
	syscall!(
		Sys::OpenAt,
		dir_file_desc,
		filename,
		open_how,
		size,
	)
}


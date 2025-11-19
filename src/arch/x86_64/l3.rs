
pub use std::ffi::CStr;
use std::ffi::c_int;

use crate::l2::{self, open};
use crate::result::Error;
pub use crate::types::*;


pub fn pause() -> () {
	l2::pause();
}

pub fn scheduler_yield() -> () {
	l2::scheduler_yield();
}

pub fn get_process_id() -> ProcessId {
	ProcessId { value: l2::get_process_id().value as _ }
}

pub fn exit(status: u8) -> ! {
	unsafe {
		l2::exit(status as _);
		core::hint::unreachable_unchecked()
	}
}


#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FileDesc {
	pub raw: l2::RawFd,
}

impl FileDesc {

	pub fn openat2(
		dir_file_desc: FileDesc,
		filename: &CStr,
		open_how: OpenHow,
	) -> Result<Self, Error> {
		unsafe {
			l2::openat2(
				dir_file_desc.raw,
				filename.as_ptr() as _,
				&open_how,
				core::mem::size_of_val(&open_how),
			).catch()
		}.map(|value| Self { raw: value as _ })
	}

	pub fn long_seek(
		&self,
		offset: isize,
		whence: Seek,
	) -> Result<isize, Error> {
		unsafe {
			l2::long_seek(
				self.raw,
				offset as _,
				whence as _,
			).catch()
		}.map(|value| value as isize)
	}

	pub fn open(
		filename: &CStr,
		flags: OpenFlags,
		mode: u32,
	) -> Result<FileDesc, Error> {
		unsafe {
			l2::open(
				filename.as_ptr(),
				flags.raw,
				mode,
			).catch()
		}.map(|value| Self { raw: value as _ })
	}

	pub fn read(&self, buffer: &mut [u8]) -> Result<usize, Error> {
		unsafe {
			l2::read(
				self.raw,
				buffer.as_mut_ptr(),
				buffer.len(),
			).catch()
		}.map(|value| value as usize)
	}

	pub fn write(&self, data: &[u8]) -> Result<usize, Error> {
		unsafe {
			l2::write(
				self.raw,
				data.as_ptr(),
				data.len(),
			).catch()
		}.map(|value| value as usize)
	}

	pub fn socket(
		domain: AddressFamily,
		semantics: ProtocolSemantic,
		protocol: c_int,
	) -> Result<Self, Error> {
		unsafe {
			l2::socket(
				domain as _,
				semantics as _,
				protocol,
			).catch()
		}.map(|value| Self { raw: value as _ })
	}

	
	pub fn accept(
		&self,
	) -> Result<(Self, libc::sockaddr), Error> {

		let endpoint = libc::sockaddr {
			sa_family: AddressFamily::IPV6,
			sa_data: [0; 14],
		};
		let length = core::mem::size_of_val(&endpoint);

		unsafe {
			l2::accept(
				self.raw,
				endpoint as *mut libc::sockaddr,
				&mut length,
			).catch()
		}.map(|value| (Self { raw: value as _}, endpoint))
	}
}

pub static STD_INPUT: FileDesc = FileDesc { raw: 0 };
pub static STD_OUTPUT: FileDesc = FileDesc { raw: 1 };
pub static STD_ERROR: FileDesc = FileDesc { raw: 2 };
pub static CURRENT_WORKING_DIRECTORY: FileDesc = FileDesc { raw: -100 };

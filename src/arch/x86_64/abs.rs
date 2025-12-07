use std::ffi::CStr;
use std::path;

use libc::close;

use crate::funcs;
use crate::types;
use crate::result::Error;
use crate::consts;



pub type Result<T> = core::result::Result<T, Error>;

pub fn exit(status: u8) -> ! {
	unsafe {
		funcs::exit(status as _);
		core::hint::unreachable_unchecked()
	}
}

pub fn pause() {
	unsafe { funcs::pause(); }
}

pub fn fork() {
	unsafe { funcs::fork(); }
}

pub fn get_process_id() -> types::pid_t {
	unsafe {
		funcs::getpid()
	}.catch_unchecked() as _
}

pub struct OpenFlags {}
pub struct FileStatus {}
pub struct Resolve {}
pub enum AddressFamily {}
pub enum ProtocolSemantic {}

#[derive(Debug, Clone)]
pub enum ShutdownHow {
	Read,
	Write,
	ReadWrite,
}

pub struct OpenHow {
	flags: OpenFlags,
	mode: FileStatus,
	resolve: Resolve,
}


#[derive(Debug, Clone, PartialEq)]
pub enum IOResult {
	Ok,
	Closed(usize),
	Error(Error, usize),
}


use core::task::{
	Poll,
	Context,
};

use core::pin::Pin;


#[macro_export] macro_rules! dont_interrupt {
	($expr:expr) => {
		loop {
			let value = ($expr);

			match value {
				Err($crate::result::Error::Interrupted) => {},
				other => break other,
			}
		}
	};
}

pub struct WouldBlock<F>(pub F);

impl<F, T> Future for WouldBlock<F>
where
	F: FnMut() -> Result<T>,
{
	type Output = Result<T>;

	fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
		let this = unsafe { self.get_unchecked_mut() };
		match (this.0)() {
			Err(Error::Again) => Poll::Pending,
			other => Poll::Ready(other),
		}
	}
}

#[macro_export] macro_rules! would_block {
    ($expr:expr) => {
		{
			$crate::WouldBlock(|| { $expr })
		}
    };
}

#[macro_export] macro_rules! retry {
	($expr:expr) => {
		would_block!(dont_interrupt!($expr))
	};
}

#[derive(Debug)]
pub struct File {
	pub value: types::c_int,
}

impl File {

	pub fn long_seek(&self, offset: types::off_t, whence: types::c_int) -> Result<types::off_t> {
		unsafe {
			funcs::lseek(self.value, offset, whence)
		}.catch().map(|result| result as _)
	}

	pub fn create(pathname: &CStr, mode: types::mode_t) -> Result<Self> {
		unsafe {
			funcs::creat(pathname.as_ptr() as _, mode)
		}.catch().map(|result| Self { value: result as _ })
	}

	pub fn open_at_v2(directory: &Self, pathname: &CStr, how: &mut types::open_how) -> Result<Self> {
		unsafe {
			funcs::openat2(
				directory.value,
				pathname.as_ptr() as _,
				how,
				core::mem::size_of_val(&how),
			)
		}.catch().map(|result| File { value: result as _ })
	}

	pub fn read(&self, buffer: &mut [u8]) -> Result<usize> {
		unsafe {
			funcs::read(self.value, buffer.as_mut_ptr() as _, buffer.len())
		}.catch().map(|result| result as _)
	}

	pub fn write(&self, data: &[u8]) -> Result<usize> {
		unsafe {
			funcs::write(self.value, data.as_ptr() as _, data.len())
		}.catch().map(|result| result as _)
	}

	pub async fn write_all(&self, data: &[u8]) -> IOResult {
		
		let mut index: usize = 0;

		while index < data.len() {

			match would_block!(self.write(&data[index..])).await {
				Err(error) => return IOResult::Error(error, index),
				Ok(0) => return IOResult::Closed(index),
				Ok(written) => index += written,
			}
		}

		IOResult::Ok
	}

	pub async fn read_all(&self, buffer: &mut [u8]) -> IOResult {

		let mut index: usize = 0;

		while index < buffer.len() {

			match would_block!(self.read(&mut buffer[index..])).await {
				Err(error) => return IOResult::Error(error, index),
				Ok(0) => return IOResult::Closed(index),
				Ok(read) => index += read,
			}
		}

		IOResult::Ok
	}

	pub fn bind<T>(&self, address: &T) -> Result<()> {
		unsafe {
			funcs::bind(self.value, address as *const _ as _, core::mem::size_of_val(address) as _)
		}.catch().map(|_| ())
	}

	pub fn listen(&self, backlog: types::c_uint) -> Result<()> {
		unsafe {
			funcs::listen(self.value, backlog as _)
		}.catch().map(|_| ())
	}

	pub unsafe fn accept_simple(&self) -> Result<File> {
		unsafe {
			funcs::accept(self.value, core::ptr::null_mut(), core::ptr::null_mut())
		}.catch().map(|result| File { value: result as _ })
	}

	pub unsafe fn accept_extra<T>(&self, non_block: bool, close_exec: bool) -> Result<(File, T, types::socklen_t)> {

		let mut address = core::mem::MaybeUninit::<T>::uninit();
		let mut length = core::mem::size_of::<T>() as types::socklen_t;

		let mut flags: types::c_int = 0;
		if non_block { flags |= consts::SOCK_NONBLOCK; }
		if close_exec { flags |= consts::SOCK_CLOEXEC; }

		unsafe {
			funcs::accept4(self.value, &mut address as *mut _ as _, &mut length, flags)
		}.catch().map(
			|result| (
				File { value: result as _ },
				unsafe { address.assume_init() },
				length,
			)
		)
	}

	pub fn socket(address_family: types::c_int, semantic: types::c_int, protocol: types::c_int) -> Result<Self> {
		unsafe {
			funcs::socket(address_family, semantic, protocol)
		}.catch().map(|result| Self { value: result as _ } )
	}

	pub fn get_socket_option<T>(&self, level: types::c_int, option: types::c_int) -> Result<(types::c_int, T)> {

		let mut value = core::mem::MaybeUninit::<T>::uninit();

		unsafe {
			funcs::setsockopt(
				self.value,
				level,
				option,
				&mut value as *mut _ as _,
				core::mem::size_of::<T>() as _
			)
		}.catch().map(
			|result| (
				result as _,
				unsafe { value.assume_init() },
			)
		)
	}

	pub fn set_socket_option<T>(&self, level: types::c_int, option: types::c_int, value: &T) -> Result<types::c_int> {
		unsafe {
			funcs::setsockopt(
				self.value,
				level,
				option,
				value as *const _ as _,
				core::mem::size_of_val(value) as _
			)
		}.catch().map(|result| result as _)
	}

	pub fn shutdown(&self, how: ShutdownHow) -> Result<()> {

		let how = match how {
			ShutdownHow::Read => 0,
			ShutdownHow::Write => 1,
			ShutdownHow::ReadWrite => 2,
		};

		unsafe {
			funcs::shutdown(self.value, how)
		}.catch().map(|_| ())
	}

	pub fn connect<T>(&self, address: &T) -> Result<()> {
		unsafe {
			funcs::connect(self.value, address as *const _ as _, core::mem::size_of_val(address) as _)
		}.catch().map(|_| ())
	}

}

impl Drop for File {
	fn drop(&mut self) {
		unsafe {
			funcs::close(self.value);
		}
	}
}


pub static STD_INPUT: File = File { value: 0 };
pub static STD_OUTPUT: File = File { value: 1 };
pub static STD_ERROR: File = File { value: 2 };
pub static CWD: File = File { value: -100 };




pub struct Memory {
	pointer: *mut types::c_void,
	size: usize,
}

impl Memory {

	pub fn len(&self) -> usize {
		self.size
	}

	pub fn as_ptr(&self) -> *const types::c_void {
		self.pointer
	}

	pub fn as_mut_ptr(&self) -> *mut types::c_void {
		self.pointer
	}

	pub unsafe fn from_raw(pointer: *mut types::c_void, size: usize) -> Memory {
		Self { pointer,	size }
	}

	pub fn new(
		suggested: *mut types::c_void,
		size: usize,
		protection: types::c_int,
		flags: types::c_int,
		file: &File,
		offset: types::off_t,
	) -> Result<Self> {

		unsafe {
			funcs::mmap(
				suggested,
				size,
				protection,
				flags,
				file.value,
				offset,
			)
		}.catch().map(|value| Self { pointer: value as _, size })
	}

}

impl Drop for Memory {
	fn drop(&mut self) {
		unsafe {
			funcs::munmap(self.pointer, self.size)
		};
	}
}








pub struct IORing {
	pub file: File,
	pub params: types::io_uring_params,
}

impl IORing {

	pub fn setup(entries: u32, params: &mut types::io_uring_params) -> Result<File> {
		unsafe {
			funcs::io_uring_setup(entries, params)
		}.catch().map(|result| File { value: result as _})
	}

	pub fn enter(&self, to_submit: u32, min_complete: u32, flags: u32) -> Result<()> {
		unsafe {
			funcs::io_uring_enter(
				self.file.value as _,
				to_submit,
				min_complete,
				flags,
				core::ptr::null(),
				0
			)
		}.catch().map(|_| ())
	}

	pub fn new(entries: u32, mut params: types::io_uring_params) -> Result<Self> {
		Ok(
			Self {
				file: Self::setup(entries, &mut params)?,
				params,
			}
		)
	}
	
	pub fn map_submission_queue_ring(&self) -> Result<Memory> {

		let size =
			self.params.sq_off.array as usize +
			self.params.sq_entries as usize * core::mem::size_of::<u32>();

		Memory::new(
			core::ptr::null_mut(),
			size,
			consts::PROT_READ | consts::PROT_WRITE,
			consts::MAP_SHARED | consts::MAP_POPULATE,
			&self.file,
			consts::IORING_OFF_SQ_RING as _,
		)
	}

	pub fn map_completion_queue_ring(&self) -> Result<Memory> {

		let size =
		self.params.cq_off.cqes as usize +
		self.params.cq_entries as usize * core::mem::size_of::<types::io_uring_cqe>();

		Memory::new(
			core::ptr::null_mut(),
			size,
			consts::PROT_READ | consts::PROT_WRITE,
			consts::MAP_SHARED | consts::MAP_POPULATE,
			&self.file,
			consts::IORING_OFF_CQ_RING as _,
		)
	}

	pub fn map_submission_queue_entries(&self) -> Result<Memory> {

		let size = self.params.sq_entries as usize * core::mem::size_of::<types::io_uring_sqe>();

		Memory::new(
			core::ptr::null_mut(),
			size,
			consts::PROT_READ | consts::PROT_WRITE,
			consts::MAP_SHARED | consts::MAP_POPULATE,
			&self.file,
			consts::IORING_OFF_SQES as _,
		)
	}

}

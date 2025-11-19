
use std::os::raw::c_void;

use core::task::{
	Poll,
	Context,
};

use core::pin::Pin;

use crate::l1;
use crate::l1::{
	Sys
};

use libc::{
	c_int as int, c_long as long, c_uint as uint, c_void as void, in6_addr, in_port_t, off_t, sa_family_t, socklen_t
};






macro_rules! syscall {
	(
		$number:expr
		
		$(,$a:expr
		$(,$b:expr
		$(,$c:expr
		$(,$d:expr
		$(,$e:expr
		$(,$f:expr
		)?)?)?)?)?)?
		$(,)?
	) => {
		{
			Error::catch(
				l1::syscall!(
					$number,
					$($a,
					$($b,
					$($c,
					$($d,
					$($e,
					$($f,
					)?)?)?)?)?)?
				)
			)
		}
	}
}

pub struct WouldBlock<F>(pub F);

impl<F, T> Future for WouldBlock<F>
where
	F: FnMut() -> Result<T, Error>,
{
	type Output = Result<T, Error>;

	fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
		let this = unsafe { self.get_unchecked_mut() };
		match (this.0)() {
			Err(Error::AGAIN) => Poll::Pending,
			other => Poll::Ready(other),
		}
	}
}

#[macro_export] macro_rules! would_block {
    ($expr:expr) => {
		{
			$crate::l3::WouldBlock(|| { $expr })
		}
    };
}

pub use would_block;

/*
#[repr(u16)]
pub enum Error {
	Perm = 1,
	NoEnt = 2,
	Search = 3,
	Interrupt = 4,
	Again = 11,
}
*/

#[derive(Debug, PartialEq)]
pub struct Error {
	value: u16,
}

impl Error {

	pub const PERM: Self = Self { value: 1 };
	pub const NO_ENT: Self = Self { value: 2 };
	pub const SEARCH: Self = Self { value: 3 };
	pub const INTR: Self = Self { value: 4 };
	pub const AGAIN: Self = Self { value: 11 };


	fn catch(value: long) -> Result<usize, Self> {

		match value {
			-0xfff..0 => Err(Self { value: -value as u16 }),
			_ => Ok(value as _),
		}
	}
}

#[repr(C)]
pub struct OpenHow {
	flags: u64,
	mode: u64,
	resolve: u64,
}

impl OpenHow {
	
	fn new() -> Self {
		Self {
			flags: 0,
			mode: 0,
			resolve: 0,
		}
	}

	fn read_only(mut self) -> Self {
		self.flags |= libc::O_RDONLY as u64;
		self
	}
}

pub enum AddressFamily {
	IPV4 = 2,
	IPV6 = 10,
}

pub enum Semantic {
	Stream = 1,
	Datagram = 2,
}

pub enum Level {
	Socket = 1,
	ProtocolIP6 = 41,
}

#[repr(C)]
pub struct SocketIPV6 {
	family: sa_family_t,
	port: in_port_t,
	flowinfo: u32,
	address: in6_addr,
	scope_id: u32,
}

pub enum ShutdownHow {
	Read = 0,
	Write = 1,
	ReadWrite = 2,
}

pub struct File {
	pub desc: int,
}

pub struct Connection {
	pub socket: File,
	pub endpoint: SocketIPV6,
}

impl File {

	pub fn socket(
		family: AddressFamily,
		semantic: int,
	) -> Result<Self, Error> {

		let value = unsafe {
			syscall!(
				Sys::Socket,
				family as int,
				semantic,
				0 as int,
			)
		};

		match value {
			Err(value) => Err(value),
			Ok(value) => Ok(Self { desc: value as _}),
		}
	}

	pub fn set_socket_option<T>(
		&self,
		level: Level,
		name: int,
		value: &T,
	) -> Result<int, Error> {

		let value = unsafe {
			syscall!(
				Sys::SetSocketOption,
				self.desc,
				level as int,
				name as int,
				value as *const _,
				core::mem::size_of::<T>(),
			)
		};

		match value {
			Err(value) => Err(value),
			Ok(value) => Ok(value as _),
		}
	}

	pub fn bind<T>(
		&self,
		endpoint: &T,
	) -> Result<(), Error> {

		let value = unsafe {
			syscall!(
				Sys::Bind,
				self.desc,
				endpoint,
				core::mem::size_of_val(endpoint),
			)
		};

		match value {
			Err(value) => Err(value),
			Ok(_) => Ok(()),
		}
	}

	pub fn accept<T>(
		&self,
		endpoint: Option<&mut T>,
		flags: int,
	) -> Result<Self, Error> {

		let (endpoint, mut length) = if let Some(endpoint) = endpoint {
			(endpoint as *mut T, core::mem::size_of_val(&endpoint))
		} else {
			(core::ptr::null_mut(), 0)
		};

		let value = unsafe {
			syscall!(
				Sys::Accept,
				self.desc,
				endpoint,
				&mut length,
				flags,
			)
		};

		match value {
			Err(value) => Err(value),
			Ok(value) => Ok(Self { desc: value as _}),
		}
	}


	pub fn accept_all(
		&self,
		flags: int,
	) -> Result<Connection, Error> {
		
		let mut endpoint = std::mem::MaybeUninit::<SocketIPV6>::uninit();
		let mut length = std::mem::size_of_val(&endpoint);

		let value = unsafe {
			syscall!(
				Sys::Accept,
				self.desc,
				&mut endpoint,
				&mut length,
				flags,
			)
		};

		match value {
			Err(value) => Err(value),
			Ok(value) => Ok(
				Connection {
					socket: Self { desc: value as _ },
					endpoint: unsafe { endpoint.assume_init() },
				}
			),
		}
	}

	pub fn listen(
		&self,
		backlog: uint,
	) -> Result<(), Error> {

		let value = unsafe {
			syscall!(
				Sys::Listen,
				self.desc,
				backlog,
			)
		};

		match value {
			Err(value) => Err(value),
			Ok(_) => Ok(()),
		}
	}

	pub fn shutdown(&self, how: ShutdownHow) -> Result<(), Error> {

		let value = unsafe {
				syscall!(
				Sys::Shutdown,
				how as int,
			)
		};

		match value {
			Err(value) => Err(value),
			Ok(_) => Ok(()),
		}
	}

	pub fn openat2(
		directory: File,
		path: *const char,
		how: &OpenHow,
	) -> Result<Self, Error> {
		
		let value = unsafe {
			syscall!(
				Sys::OpenAt2,
				directory.desc,
				path,
				how,
				core::mem::size_of_val(&how),
			)
		};

		match value {
			Err(value) => Err(value),
			Ok(value) => Ok(Self { desc: value as _}),
		}
	}

	pub fn read(&self, buffer: &mut [u8]) -> Result<usize, Error> {
	
		unsafe {
			syscall!(
				Sys::Read,
				self.desc,
				buffer.as_mut_ptr(),
				buffer.len(),
			)
		}
	}

	pub fn write(&self, data: &[u8]) -> Result<usize, Error> {

		unsafe {
			syscall!(
				Sys::Write,
				self.desc,
				data.as_ptr(),
				data.len(),
			)
		}
	}


	pub fn ipv6_only(&self, value: bool) -> Result<(), Error> {

		let value = self.set_socket_option(
			Level::ProtocolIP6,
			libc::IPV6_V6ONLY,
			&(value as int),
		);

		match value {
			Err(value) => Err(value),
			Ok(_) => Ok(()),
		}
	}

	pub fn reuse_address(&self, value: bool) -> Result<(), Error> {

		let value = self.set_socket_option(
			Level::Socket,
			libc::SO_REUSEADDR,
			&(value as int),
		);

		match value {
			Err(value) => Err(value),
			Ok(_) => Ok(()),
		}
	}
}



impl Drop for File {
	fn drop(&mut self) {
		let _ = unsafe {
			syscall!(
				Sys::Close,
				self.desc,
			)
		};
	}
}

pub static STDIN: File = File { desc: 0 };
pub static STDOUT: File = File { desc: 1 };
pub static STDERR: File = File { desc: 2 };

static CWD: File = File { desc: -100 };


pub fn get_pid() -> long {
	unsafe { l1::syscall!(Sys::GetProcessId) }
}

pub fn exit(status: u8) -> ! {
	unsafe {
		l1::syscall!(Sys::Exit, status);
		core::hint::unreachable_unchecked()
	}
}


struct Memory {
	pointer: *mut (),
}

impl Memory {

	fn new(
		suggestion: Option<&[char]>,
		protocol: int,
		flags: int,
		fd: int,
		offset: off_t,
	) -> Self {
		todo!()
	}
}

impl Drop for Memory {
	fn drop(&mut self) {
		todo!()
	}
}


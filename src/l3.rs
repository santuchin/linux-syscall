
use std::os::raw::c_void;

use crate::l1;

use crate::l1::{
	sys
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

#[derive(Debug, PartialEq)]
pub struct Error {
	value: u16
}

impl Error {

	pub const PERM: Self = Self { value: 1 };
	pub const NO_ENT: Self = Self { value: 2 };
	pub const SEARCH: Self = Self { value: 3 };
	pub const INTR: Self = Self { value: 4 };
	pub const AGAIN: Self = Self { value: 11 };
	pub const WOULD_BLOCK: Self = Self { value: 11 };

	fn catch(value: long) -> Result<usize, Self> {

		match value {
			-0xfff..0 => Err(Self { value: -value as u16 }),
			_ => Ok(value as _),
		}
	}
}

#[repr(C)]
struct OpenHow {
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
	desc: int,
}

impl File {

	pub fn socket(
		family: AddressFamily,
		semantic: int,
	) -> Result<Self, Error> {

		let value = unsafe {
			syscall!(
				sys::SOCKET,
				family as int,
				semantic,
				0 as int,
			)
		};

		match value {
			Ok(desc) => Ok(Self { desc: desc as int }),
			Err(value) => Err(value),
		}
	}

	pub fn set_socket_options<T>(
		&self,
		level: Level,
		name: int,
		value: &T,
	) -> Result<int, Error> {

		let value = unsafe {
			syscall!(
				sys::SETSOCKOPT,
				self.desc,
				level as int,
				name as int,
				value as *const _,
				std::mem::size_of::<T>(),
			)
		};

		match value {
			Ok(desc) => Ok(desc as int),
			Err(value) => Err(value),
		}
	}

	pub fn bind<T>(
		&self,
		endpoint: &T,
	) -> Result<(), Error> {

		let value = unsafe {
			syscall!(
				sys::BIND,
				self.desc,
				endpoint,
				std::mem::size_of_val(endpoint),
			)
		};

		match value {
			Err(value) => Err(value),
			_ => Ok(()),
		}
	}

	// int accept(int sockfd, struct sockaddr *_Nullable restrict addr, socklen_t *_Nullable restrict addrlen);
	pub fn accept<T>(
		&self,
		endpoint: Option<&mut T>,
	) -> Result<Self, Error> {

		let mut length: socklen_t = if endpoint.is_some() {
			0
		} else {
			std::mem::size_of::<T>()
		} as _;

		let value = unsafe {
			syscall!(
				sys::ACCEPT,
				self.desc,
				endpoint.unwrap_unchecked(),
				&mut length,
			)
		};

		match value {
			Err(value) => Err(value),
			Ok(value) => Ok(Self { desc: value as _ }),
		}
	}

	pub fn accept4<T>(
		&self,
		endpoint: Option<&mut T>,
		flags: int,
	) -> Result<Self, Error> {

		let mut length: socklen_t = if endpoint.is_some() {
			0
		} else {
			std::mem::size_of::<T>()
		} as _;

		let value = unsafe {
			syscall!(
				sys::ACCEPT,
				self.desc,
				endpoint.unwrap_unchecked(),
				&mut length,
				flags,
			)
		};

		match value {
			Err(value) => Err(value),
			Ok(value) => Ok(Self { desc: value as _ }),
		}
	}

	pub fn listen(
		&self,
		backlog: uint,
	) -> Result<(), Error> {

		let value = unsafe {
			syscall!(
				sys::LISTEN,
				self.desc,
				backlog,
			)
		};

		match value {
			Err(value) => Err(value),
			_ => Ok(()),
		}
	}

	pub fn shutdown(&self, how: ShutdownHow) -> Result<(), Error> {

		let value = unsafe {
				syscall!(
				sys::SHUTDOWN,
				how as int,
			)
		};

		match value {
			Err(value) => Err(value),
			_ => Ok(()),
		}		
	}

	pub fn openat2(
		directory: File,
		path: *const char,
		how: &OpenHow,
	) -> Result<Self, Error> {
		
		let value = unsafe {
			syscall!(
				sys::OPENAT2,
				directory.desc,
				path,
				how,
				std::mem::size_of_val(&how),
			)
		};


		match value {
			Ok(desc) => Ok(Self { desc: desc as int }),
			Err(value) => Err(value),
		}
	}

	pub fn read<T>(&self, buffer: &mut [T]) -> Result<usize, Error> {
	
		unsafe {
			syscall!(
				sys::READ,
				self.desc,
				buffer.as_mut_ptr(),
				std::mem::size_of::<T>() * buffer.len(),
			)
		}
	}

	pub fn write<T>(&self, data: &[T]) -> Result<usize, Error> {

		unsafe {
			syscall!(
				sys::WRITE,
				self.desc,
				data.as_ptr(),
				std::mem::size_of::<T>() * data.len(),
			)
		}
	}
}

impl Drop for File {
	fn drop(&mut self) {
		unsafe {
			syscall!(
				sys::CLOSE,
				self.desc,
			)
		};
	}
}

static stdin: File = File { desc: 0 };
static stdout: File = File { desc: 1 };
static stderr: File = File { desc: 2 };

static cwd: File = File { desc: -100 };


pub fn getpid() -> long {
	unsafe { l1::syscall!(sys::GETPID) }
}

pub fn exit(status: u8) -> ! {
	unsafe { l1::syscall!(sys::EXIT, status as int) };
	unsafe { core::hint::unreachable_unchecked() }
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
	}
}

impl Drop for Memory {
	fn drop(&mut self) {
		
	}
}



use core::ffi::{
	c_int,
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct OpenHow {
	pub flags: u64,
	pub mode: u64,
	pub resolve: u64,
}

/*

#define RESOLVE_NO_XDEV		0x01 /* Block mount-point crossings
					(includes bind-mounts). */
#define RESOLVE_NO_MAGICLINKS	0x02 /* Block traversal through procfs-style
					"magic-links". */
#define RESOLVE_NO_SYMLINKS	0x04 /* Block traversal through all symlinks
					(implies OEXT_NO_MAGICLINKS) */
#define RESOLVE_BENEATH		0x08 /* Block "lexical" trickery like
					"..", symlinks, and absolute
					paths which escape the dirfd. */
#define RESOLVE_IN_ROOT		0x10 /* Make all jumps to "/" and ".."
					be scoped inside the dirfd
					(similar to chroot(2)). */
#define RESOLVE_CACHED		0x20

*/

impl OpenHow {

	pub fn new() -> Self {
		Self {
			flags: 0,
			mode: 0,
			resolve: 0,
		}
	}
	
	pub fn resolve_no_xdev(mut self) -> Self {
		self.resolve |= 0b1; self
	}

	pub fn resolve_no_magic_links(mut self) -> Self {
		self.resolve |= 0b10; self
	}

	pub fn resolve_no_sym_links(mut self) -> Self {
		self.resolve |= 0b100; self
	}

	pub fn resolve_beneath(mut self) -> Self {
		self.resolve |= 0b1000; self
	}

	pub fn resolve_in_root(mut self) -> Self {
		self.resolve |= 0b1_0000; self
	}

	pub fn resolve_cached(mut self) -> Self {
		self.resolve |= 0b10_0000; self
	}

}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ProcessId {
	pub value: c_int,
}


#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct OpenFlags {
	pub raw: c_int,
}

impl OpenFlags {

	pub fn new() -> Self {
		Self {
			raw: 0
		}
	}

	pub fn read_only(mut self) -> Self {
		self.raw |= libc::O_RDONLY; self
	}

	pub fn write_only(mut self) -> Self {
		self.raw |= libc::O_WRONLY; self
	}

	pub fn read_write(mut self) -> Self {
		self.raw |= libc::O_RDWR; self
	}
}

pub enum Seek {
	Start = 0,
	Current = 1,
	End = 2,
	NextData = 3,
	NextHole = 4
}

pub enum AddressFamily {
	IPV4 = 2,
	IPV6 = 10,
}

pub enum ProtocolSemantic {
	TCP = 1,
	UDP = 2,
}

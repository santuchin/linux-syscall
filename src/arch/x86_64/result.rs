
/*

https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/uapi/asm-generic/errno-base.h

https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/uapi/asm-generic/errno.h

*/

use core::ffi::c_long;

pub const MAX_ERROR: core::ffi::c_long = 0xfff;

#[derive(Debug)]
#[repr(transparent)]
pub struct Result {
	pub value: c_long,
}

impl Result {

	pub unsafe fn catch(self) -> core::result::Result<c_long, Error> {

		match self.value {
			-0xfff..0 => Err(std::mem::transmute((-(self.value as i16)) as u16)),
			_ => Ok(self.value),
		}
	}

}

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum Error {
	OperationNotPermitted = 1,
	NoSuchFileOrDirectory = 2,
	NoSuchProcess = 3,
	Interrupted = 4,
	InputOutput = 5,
	NoSuchDeviceOrAddress = 6,
	TooMuchArgs = 7,
	NoExec = 8,
	BadFileDesc = 9,
	NoChild = 10,
	Again = 11,
	OutOfMemory = 12,
	PermissionDenied = 13,
	BadAddress = 14,
	BlockDeviceRequired = 15,
	ResourceBusy = 16,
	FileExists = 17,
	CrossDeviceLink = 18,
	NoSuchDevice = 19,
	NotADirectory = 20,
	IsADirectory = 21,
	InvalidArgument = 22,
	FileTableOverflow = 23,
	TooManyOpenFiles = 24,
	NotATypewriter = 25,
	TextFileBusy = 26,
	FileTooLarge = 27,
	NoSpaceLeft = 28,
	IlegalSeek = 29,
	ReadOnlyFileSystem = 30,
	TooManyLinks = 31,
	BrokenPipe = 32,
	OutOfDomain = 33,
	OutOfRange = 34,
}


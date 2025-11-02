
#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_export] macro_rules! syscall {

	/*
	
	rax
	
	rdi
	rsi
	rdx
	r10
	r8
	r9

	*/

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
			let value: libc::c_long;

			std::arch::asm!(
				"syscall",
				
				in("rax") $number,
				
				$(in("rdi") $a,
				$(in("rsi") $b,
				$(in("rdx") $c,
				$(in("r10") $d,
				$(in("r8")  $e,
				$(in("r9")  $f,
				)?)?)?)?)?)?

				lateout("rax") value,
				lateout("rcx") _,
				lateout("r11") _,
			);

			value
		}
	}
}

pub use syscall;

pub const ERROR_RESERVED: libc::c_long = -4095;

pub mod sys {
	use core::ffi::c_long as long;

	pub const READ: long = libc::SYS_read;
	pub const WRITE: long = 1;
	pub const OPEN: long = 2;
	pub const CLOSE: long = 3;

	pub const GETPID: long = 39;

	pub const SOCKET: long = 41;

	pub const SETSOCKOPT: long = libc::SYS_setsockopt;
	pub const SHUTDOWN: long = libc::SYS_shutdown;

	pub const ACCEPT: long = 43;

	pub const BIND: long = 49;
	pub const LISTEN: long = 50;

	pub const EXIT: long = 60;

	pub const IO_URING_SETUP: long = 425;
	pub const IO_URING_ENTER: long = 426;
	pub const IO_URING_REGISTER: long = 427;

	pub const OPENAT2: long = 437;

	// pub const : long = ;
}

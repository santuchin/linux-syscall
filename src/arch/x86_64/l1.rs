
use core::ffi::c_long as CLong;

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
			use core::ffi::c_long;

			let value: c_long;

			core::arch::asm!(
				"syscall",
				
				in("rax") ($number) as c_long,
				
				$(in("rdi") ($a),
				$(in("rsi") ($b),
				$(in("rdx") ($c),
				$(in("r10") ($d),
				$(in("r8")  ($e),
				$(in("r9")  ($f),
				)?)?)?)?)?)?

				lateout("rax") value,
				lateout("rcx") _,
				lateout("r11") _,

				clobber_abi("sysv64"),
				options(nostack),
			);

			value
		}
	}
}

pub use syscall;







pub const MAX_ERROR: libc::c_long = -0xfff;






#[repr(i64)]
pub enum Sys {
	Read = 0,
	Write = 1,
	Open = 2,
	Close = 3,
	GetProcessId = 39,
	Socket = 41,
	SetSocketOption = libc::SYS_setsockopt,
	Shutdown = libc::SYS_shutdown,
	Accept = 43,
	Bind = 49,
	Listen = 50,
	Exit = 60,
	IoUringSetup = 425,
	IoUringEnter = 426,
	IoUringRegister = 427,
	OpenAt2 = 437,
}

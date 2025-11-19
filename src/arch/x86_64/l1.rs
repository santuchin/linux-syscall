

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

			$crate::result::Result { value }
		}
	}
}

pub use syscall;




#[macro_export] macro_rules! syscall {

	(
		$number:expr,
		(	
		$($a:expr
		$(,$b:expr
		$(,$c:expr
		$(,$d:expr
		$(,$e:expr
		$(,$f:expr
		)?)?)?)?)?)?
		$(,)?
		)
		$(,)?
	) => {
		{
			let result: $crate::types::c_long;

			core::arch::asm!(
				"syscall",
				
				in("rax") ($number),
				
				$(in("rdi") ($a),
				$(in("rsi") ($b),
				$(in("rdx") ($c),
				$(in("r10") ($d),
				$(in("r8")  ($e),
				$(in("r9")  ($f),
				)?)?)?)?)?)?

				lateout("rax") result,
				lateout("rcx") _,
				lateout("r11") _,

				options(nostack),
			);

			result
		}
	}
}




#[cfg(not(any(
		target_arch = "x86_64",
		target_arch = "aarch64",
	)))
]
compile_error!("linux-syscalls doesn't support your target architecture");

pub mod x86_64;
// pub mod aarch64;

#[cfg(any(target_arch = "x86_64"))] pub use x86_64 as current;

// #[cfg(target_arch = "aarch64")] pub use aarch64 as current;

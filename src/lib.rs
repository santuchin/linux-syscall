
// cargo test -- --nocapture
#![allow(warnings)] #[cfg(test)] mod tests;

pub mod arch;

const _: () = assert!(std::mem::size_of::<usize>() == std::mem::size_of::<libc::size_t>());
#[cfg(not(target_os = "linux"))] compile_error!("linux-syscalls only supports Linux");

pub use arch::current::*;

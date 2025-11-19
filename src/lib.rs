
#![allow(warnings)]
#[cfg(test)] mod tests;

pub mod arch;

/*
#[cfg(not(target_os = "linux"))]
compile_error!("linux-syscalls only supports Linux");

*/

pub use arch::current::*;

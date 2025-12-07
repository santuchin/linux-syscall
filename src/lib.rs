
// cargo test -- --nocapture
#![allow(warnings)] #[cfg(test)] mod tests;

mod arch;

#[cfg(any(target_arch = "x86_64"))] pub use arch::x86_64::*;

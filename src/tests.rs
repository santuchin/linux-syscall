
use crate::*;

#[test]
fn main() {
	println!("============================== TEST START ======================================");

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(start());

	println!("================================ TEST END ====================================");
}

use core::ffi::c_long as CLong;

enum Sys {
	Read = 123,
}

use core::ffi::c_long;

async fn start() {
	
}

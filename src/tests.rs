

#[test]
fn main() {
	println!("============================== TEST START ==================================== ");

	let runtime = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap();

	runtime.block_on(start());

	println!("================================ TEST END ====================================");
}

use std::ffi::CString;

use crate::*;
use l3::*;

async fn start() {

	let socket = FileDesc::socket(
		AddressFamily::IPV6,
		ProtocolSemantic::TCP,
		0
	).unwrap();

	let address = libc::sockaddr {
		sa_family: AddressFamily::IPV6 as _,
		sa_data: [0; 14],
	};

	let connection = socket.accept().unwrap();
}

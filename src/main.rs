mod l1;
mod l3;

use l3::Error;

fn main() -> Result<(), l3::Error> {

	let address = unsafe { libc::in6addr_any };
	let port: u16 = 8080;

	let backlog = 256;

	let socket = l3::File::socket(
		l3::AddressFamily::IPV6,
		libc::SOCK_STREAM | libc::SOCK_NONBLOCK,
	)?;

	let value: libc::c_int = true as _;
	socket.set_socket_options(
		l3::Level::Socket,
		libc::SO_REUSEADDR,
		&value,
	)?;

	let value: libc::c_int = false as _;
	socket.set_socket_options(
		l3::Level::ProtocolIP6,
		libc::IPV6_V6ONLY,
		&value,
	)?;

	let endpoint = libc::sockaddr_in6 {
		sin6_family: libc::AF_INET6 as _,
		sin6_addr: address,
		sin6_port: port.to_be(),
		sin6_flowinfo: 0,
		sin6_scope_id: 0,
	};

	socket.bind(&endpoint)?;

	socket.listen(backlog)?;

	loop {

		let mut connection_endpoint  = std::mem::MaybeUninit::<l3::SocketIPV6>::uninit();

		let result = socket.accept4(
			Some(&mut connection_endpoint),
			libc::SOCK_NONBLOCK,
		);

		match result {
			Err(value) => {

				match value {
					Error::AGAIN => {},
					other => return Err(other),
				}
			},
			Ok(connection_socket) => {
				handle(
					connection_socket,
					unsafe { connection_endpoint.assume_init() },
				);
			}
		}
	}

	Ok(())
}

fn handle(socket: l3::File, endpoint: l3::SocketIPV6) {

	let mut buffer = [0 as u8; 1024];

	loop {
		if socket.read(buffer.as_mut_slice()).unwrap() == 0 {
			println!("break");
			break;
		}
		println!("nope");
	}

	socket.write(b"hello world");

	// socket.shutdown(l3::ShutdownHow::Write).unwrap();
}

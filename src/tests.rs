
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

use std::mem::MaybeUninit;

use crate as linux;

use linux::*;

async fn start() -> core::result::Result<(), result::Error> {

	let address: u128 = 0;
	let port: u16 = 4444;
	let backlog = 512;

	let server = File::socket(
		libc::AF_INET6,
		libc::SOCK_STREAM,
		0,
	)?;

	server.set_socket_option(
		libc::SOL_SOCKET,
		libc::SO_REUSEADDR,
		&(true as types::c_int),
	)?;

	server.set_socket_option(
		libc::SOL_SOCKET,
		libc::SO_REUSEPORT,
		&(true as types::c_int),
	)?;

	server.set_socket_option(
		libc::IPPROTO_IPV6,
		libc::IPV6_V6ONLY,
		&(false as types::c_int),
	)?;

	server.bind(
		&libc::sockaddr_in6 {
			sin6_family: libc::AF_INET6 as _,
			sin6_addr: libc::in6_addr { s6_addr: address.to_ne_bytes() },
			sin6_port: port.to_be(),
			sin6_flowinfo: 0,
			sin6_scope_id: 0,
		}
	)?;

	server.listen(backlog).unwrap();

	loop {

		let connection = retry!(
			unsafe { server.accept_extra::<libc::sockaddr_in6>(true, false) }
		).await;

		match connection {
			Err(error) => println!("Error accepting connection: {}", error),
			Ok((socket, socket_address, _)) => handle(socket, socket_address).await,
		}
	}
}


async fn handle(socket: linux::File, socket_address: libc::sockaddr_in6) {

	dbg!(socket_address);

	let content = include_bytes!("/home/santuchin/desk/santuchin.github.io/test/index.html");

	let content_length = content.len().to_string();

	let data: [&[u8]; _] = [
		b"HTTP/1.1 ", b"200", b"\r\n",
		b"Content-Type: text/html; charset=utf-8\r\n",
		b"Content-Length: ", content_length.as_bytes(), b"\r\n",
		b"\r\n",
		content,
	];
	let data = data.concat();

	if socket.write_all(data.as_slice()).await != IOResult::Ok { return; }

	socket.shutdown(ShutdownHow::ReadWrite);
}

	//io_uring_write(socket.value, data.as_slice()).await;

async fn io_uring_write(file_desc: types::c_int, message: &[u8]) -> core::result::Result<(), result::Error> {

	let entries = 8;
	let mut params = unsafe { core::mem::MaybeUninit::<types::io_uring_params>::zeroed().assume_init() };

	let ring = IORing::new(entries, params)?;

	let submission_queue_ring = ring.map_submission_queue_ring()?;
	let completion_queue_ring = ring.map_completion_queue_ring()?;
	let submission_queue_entries = ring.map_submission_queue_entries()?;

	let sq_head: &mut u32 = unsafe {
		&mut *(submission_queue_ring.as_mut_ptr().add(ring.params.sq_off.head as _) as *mut u32)
	};
	let sq_tail: &mut u32 = unsafe {
		&mut *(submission_queue_ring.as_mut_ptr().add(ring.params.sq_off.tail as _) as *mut u32)
	};
	let sq_mask: &mut u32 = unsafe {
		&mut *(submission_queue_ring.as_mut_ptr().add(ring.params.sq_off.ring_mask as _) as *mut u32)
	};
	let sq_array: *mut u32 = unsafe {
		submission_queue_ring.as_mut_ptr().add(ring.params.sq_off.array as _) as _
	};

	let cq_head: &mut u32 = unsafe {
		&mut *(completion_queue_ring.as_mut_ptr().add(ring.params.cq_off.head as _) as *mut u32)
	};
	let cq_tail: &mut u32 = unsafe {
		&mut *(completion_queue_ring.as_mut_ptr().add(ring.params.cq_off.tail as _) as *mut u32)
	};
	let cq_mask: &mut u32 = unsafe {
		&mut *(completion_queue_ring.as_mut_ptr().add(ring.params.cq_off.ring_mask as _) as *mut u32)
	};

	let cqes: *mut u32 = unsafe {
		completion_queue_ring.as_mut_ptr().add(ring.params.cq_off.cqes as _) as _
	};

	let id = 0xbad00;


	let tail = *sq_tail;
	let mask = *sq_mask;
	let index = tail & mask;


	unsafe {
		*(
			submission_queue_entries
			.as_mut_ptr().add(index as _)
			as *mut types::io_uring_sqe
		) = types::io_uring_sqe {
			opcode: consts::IORING_OP_WRITE,
			flags: 0,
			ioprio: 0,
			fd: file_desc,
			off: 0,
			addr: message.as_ptr() as _,
			len: message.len() as _,
			union1: types::io_uring_sqe_flags_union {
				rw_flags: 0
			},
			user_data: id,
			u: types::io_uring_sqe_union {
				__pad2: [0; 3]
			},
		};
	}
	
	unsafe {
		*sq_array.add(index as _) = index;
	}

	core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
	
	*sq_tail = tail + 1;


	ring.enter(1, 0, 0);
	ring.enter(0, 1, consts::IORING_ENTER_GETEVENTS);

	let mut head = *cq_head;
	let c_mask = *cq_mask;


	while head != *cq_tail {

		let cqe = unsafe {
			&*(
				cqes.add((head & c_mask) as _)
				as *const types::io_uring_cqe
			)
		};

		if cqe.user_data == id {

			if cqe.res < 0 {
				panic!("write failed: {}", -cqe.res);

			} else if (cqe.res as usize) != message.len() {
				println!("partial write: {}", cqe.res);

			} else {
				println!("write succesfull")
			}
		}

		head += 1;
	}

	*cq_head = head;


	Ok(())
}


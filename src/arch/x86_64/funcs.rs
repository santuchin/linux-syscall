
use crate::syscall; 
use crate::sys;
use crate::result::Result;
use crate::types::*;

macro_rules! define_syscall {
    ($name:ident ( $( $arg:ident : $type:ty ),* )) => {
        pub unsafe fn $name( $( $arg : $type ),* ) -> Result {
            syscall!(sys::$name, ($( $arg ),*) ).into()
        }
    }
}
macro_rules! define_syscalls {
    ($($name:ident ( $( $arg:ident : $type:ty ),* );)*) => {
        $(
            define_syscall!($name( $( $arg : $type ),* ));
        )*
    };
}

/*
https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/linux/syscalls.h
*/

define_syscalls!(
	pause();
	exit(error_code: c_int);
	read(fd: c_int, buf: *mut c_void, count: size_t);
	write(fd: c_int, buf: *const c_void, count: size_t);
	close(fd: c_int);
	accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t);
	accept4(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t, flags: c_int);
	bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t);
	listen(fd: c_int, backlog: c_int);
	open(pathname: *const c_char, flags: c_int, mode: mode_t);
	creat(pathname: *const c_char, mode: mode_t);
	openat(dirfd: c_int, pathname: *const c_char, flags: c_int, mode: mode_t);
	openat2(dirfd: c_int, pathname: *const c_char, how: *mut open_how, size: size_t);
	socket(domain: c_int, r#type: c_int, protocol: c_int);
	getsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut socklen_t);
	setsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t);
	lseek(fd: c_int, offset: off_t, whence: c_int);
	io_uring_setup(entries: u32, p: *const io_uring_params); // io_uring_setup(entries: u32, p: *mut io_uring_params);
	io_uring_enter(fd: c_uint, to_submit: u32, min_complete: u32, flags: u32, argp: *const c_void, argsz: size_t);
	io_uring_register(fd: c_uint, op: c_uint, arg: *mut c_void, nr_args: c_uint);
	mmap(addr: *mut c_void, length: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off_t);
	munmap(addr: *mut c_void, length: size_t);
	shutdown(sockfd: c_int, how: c_int);
	splice(fd_in: c_int, off_in: off_t, fd_out: c_int, off_out: off_t, len: size_t, flags: c_uint);
	stat(pathname: *const c_char, statbuf: *mut stat);
	lstat(pathname: *const c_char, statbuf: *mut stat);
	fstat(fd: c_int, statbuf: *mut stat);
	fork();
	execve(filename: *const c_char, argv: *const *const c_char, envp: *const *const c_char);
	exit_group(error_code: c_int);
	wait4(pid: pid_t, stat_addr: *mut c_int, options: c_int, ru: *mut rusage);
	pipe(fildes: *mut c_int);
	pipe2(fildes: *mut c_int, flags: c_int);
	connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t);

	getpid();
	getppid();
);

/*
ssize_t readv(int fd, const struct iovec *iov, int iovcnt);
       ssize_t writev(int fd, const struct iovec *iov, int iovcnt);

       ssize_t preadv(int fd, const struct iovec *iov, int iovcnt,
                       off_t offset);
       ssize_t pwritev(int fd, const struct iovec *iov, int iovcnt,
                       off_t offset);

       ssize_t preadv2(int fd, const struct iovec *iov, int iovcnt,
                       off_t offset, int flags);
       ssize_t pwritev2(int fd, const struct iovec *iov, int iovcnt,
                       off_t offset, int flags);
*/

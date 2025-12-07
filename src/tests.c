#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <unistd.h>
#include <errno.h>
#include <sys/mman.h>
#include <sys/syscall.h>
#include <linux/io_uring.h>

/* Minimal, direct usage of io_uring syscalls and kernel structs.
 * No liburing helpers used. No io_uring_register call.
 */

int main(void) {


	struct io_uring_params params;
	memset(&params, 0, sizeof(params));

	/* 1) setup */
	unsigned int entries = 8;
	int ring_fd = syscall(SYS_io_uring_setup, entries, &params);
	if (ring_fd < 0) {
		perror("io_uring_setup");
		return 1;
	}

	/* 2) mmap the SQ ring */
	size_t sq_ring_size = (size_t) (params.sq_off.array + params.sq_entries * sizeof(__u32));
	void* sq_ring = mmap(
		NULL,
		sq_ring_size,
		PROT_READ | PROT_WRITE,
		MAP_SHARED | MAP_POPULATE,
		ring_fd,
		IORING_OFF_SQ_RING
	);
	if (sq_ring == MAP_FAILED) {
		perror("mmap sq_ring");
		return 1;
	}

	/* 3) mmap the CQ ring */
	size_t cq_ring_size = (size_t) (params.cq_off.cqes + params.cq_entries * sizeof(struct io_uring_cqe));
	void* cq_ring = mmap(
		NULL,
		cq_ring_size,
		PROT_READ | PROT_WRITE,
		MAP_SHARED | MAP_POPULATE,
		ring_fd,
		IORING_OFF_CQ_RING
	);
	if (cq_ring == MAP_FAILED) {
		perror("mmap cq_ring");
		return 1;
	}

	/* 4) mmap the SQEs array */
	size_t sqes_size = (size_t) params.sq_entries * sizeof(struct io_uring_sqe);
	struct io_uring_sqe* sqes = mmap(
		NULL,
		sqes_size,
		PROT_READ | PROT_WRITE,
		MAP_SHARED | MAP_POPULATE,
		ring_fd,
		IORING_OFF_SQES
	);
	if (sqes == MAP_FAILED) {
		perror("mmap sqes");
		return 1;
	}

	/* 5) create pointers into the rings using offsets returned in params */
	volatile unsigned int* sq_head = (volatile unsigned int*) ((char *)sq_ring + params.sq_off.head);
	volatile unsigned int* sq_tail = (volatile unsigned int*) ((char *)sq_ring + params.sq_off.tail);
	volatile unsigned int* sq_mask = (volatile unsigned int*) ((char *)sq_ring + params.sq_off.ring_mask);
	volatile unsigned int* sq_array = (volatile unsigned int*) ((char *)sq_ring + params.sq_off.array);

	volatile unsigned int* cq_head = (volatile unsigned int*) ((char *)cq_ring + params.cq_off.head);
	volatile unsigned int* cq_tail = (volatile unsigned int*) ((char *)cq_ring + params.cq_off.tail);
	volatile unsigned int* cq_mask = (volatile unsigned int*) ((char *)cq_ring + params.cq_off.ring_mask);
	struct io_uring_cqe* cqes = (struct io_uring_cqe *) ((char *)cq_ring + params.cq_off.cqes);

	/* 6) prepare our buffer (plain user pointer - not registered) */
	const char msg[] = "hello, world\n";
	size_t msg_len = sizeof(msg) - 1; /* exclude terminating NUL */

	/* 7) allocate an SQE slot: load tail, compute index, fill sqe */
	unsigned int tail = *sq_tail;
	unsigned int mask = *sq_mask;
	unsigned int idx = tail & mask;
	struct io_uring_sqe* sqe = &sqes[idx];


	unsigned long long int id = 0xbad;

	/* Zero the SQE fields then fill relevant fields */
	memset(sqe, 0, sizeof(*sqe));
	sqe->opcode = IORING_OP_WRITE;          /* write */
	sqe->fd = 1;                            /* stdout */
	sqe->off = 0;                           /* file offset (ignored for fd) */
	sqe->addr = (uint64_t) (uintptr_t) msg;   /* user pointer */
	sqe->len = (uint32_t) msg_len;
	sqe->flags = 0;
	sqe->rw_flags = 0;
	sqe->user_data = id; /* tag to identify completion */

	/* publish the SQE by writing its index into the sq_array and bumping tail */
	sq_array[idx] = idx;
	__sync_synchronize(); /* full memory barrier to ensure visibility */
	*sq_tail = tail + 1;

	/* 8) submit: tell kernel there is 1 SQ entry to consume */
	int to_submit = 1;
	int ret = syscall(SYS_io_uring_enter, ring_fd, to_submit, 0, 0, NULL);
	if (ret < 0) {
		perror("io_uring_enter(submit)");
		return 1;
	}

	/* 9) wait for at least 1 completion (use GETEVENTS to sleep if none) */
	ret = syscall(SYS_io_uring_enter, ring_fd, 0, 1, IORING_ENTER_GETEVENTS, NULL);
	if (ret < 0) {
		perror("io_uring_enter(getevents)");
		return 1;
	}

	/* 10) reap completions from CQ */
	unsigned int head = *cq_head;
	unsigned int c_mask = *cq_mask;

	while (head != *cq_tail) {
		
		struct io_uring_cqe* cqe = &cqes[head & c_mask];

		/* examine result */
		if ((uint64_t) cqe->user_data == id) {

			if (cqe->res < 0) {
				fprintf(stderr, "write failed: %d (%s)\n", cqe->res, strerror(-cqe->res));
				return 1;
				
			} else if ((unsigned int) cqe->res != msg_len) {
				fprintf(stderr, "partial write: %d\n", cqe->res);
			}
		}

		head += 1;
	}

	/* publish new head to indicate we've consumed completions */
	*cq_head = head;

	/* 11) clean up mmaps and close ring_fd (best-effort) */
	munmap((void*) sq_ring, sq_ring_size);
	munmap((void*) cq_ring, cq_ring_size);
	munmap((void*) sqes, sqes_size);
	close(ring_fd);

	return 0;
}

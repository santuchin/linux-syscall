
/*

https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/arch/x86/entry/syscalls/syscall_64.tbl


*/


#[repr(i64)]
pub enum Sys {
	Read = 0,
	Write = 1,
	Open = 2,
	Close = 3,
	Status = 4,
	FileStatus = 5,
	LinkStatus = 6,
	Poll = 7,
	LongSeek = 8,
	MemoryMap = 9,
	MemoryProtect = 10,
	MemoryUnmap = 11,
	Break = 12,
	RealTimeSignalAction = 13,
	RealTimeSignalProcessMask = 14,
	RealTimeSignalReturn = 15,
	InputOutputControl = 16,
	PositionedRead64 = 17,
	PositionedWrite64 = 18,
	ReadVector = 19,
	WriteVector = 20,
	Access = 21,
	Pipe = 22,
	Select = 23,
	SchedulerYield = 24,
	MemoryRemap = 25,
	MemorySync = 26,
	MemoryInCore = 27,
	MemoryAdvice = 28,
	SharedMemoryGet = 29,
	SharedMemoryAt = 30,
	SharedMemoryControl = 31,
	Duplicate = 32,
	Duplicate2 = 33,
	Pause = 34,
	GetProcessId = 39,
	Socket = 41,
	Accept = 43,
	Shutdown = 48,
	Bind = 49,
	Listen = 50,
	SetSocketOption = 54,
	Exit = 60,
	GetThreadId = 186,
	GetUserId = 102,
	GetGroupId = 104,
	GetParentProcessId = 110,
	OpenAt = 257,
	IoUringSetup = 425,
	IoUringEnter = 426,
	IoUringRegister = 427,
	OpenAt2 = 437,
}

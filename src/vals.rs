
pub mod std {
	
	use libc::c_int as int;

	pub const IN: int = 0;
	pub const OUT: int = 1;
	pub const ERR: int = 2;
}

pub mod socket {

	use libc::c_int as int;

    pub const STREAM: int = 1;
    pub const DGRAM: int = 2;
    pub const SEQPACKET: int = 5;
    pub const RAW: int = 3;
    pub const RDM: int = 4;
    pub const PACKET: int = 10;

	pub const NONBLOCK: int = 0o0004000;
    pub const CLOEXEC: int  = 0o02000000;
}

pub mod address_family {

	use libc::c_int as int;

    pub const UNIX: int        = 1;
    pub const LOCAL: int       = UNIX;
    pub const INET: int        = 2;
    pub const AX25: int        = 3;
    pub const IPX: int         = 4;
    pub const APPLETALK: int   = 5;
    pub const NETROM: int      = 6;
    pub const BRIDGE: int      = 7;
    pub const ATMPVC: int      = 8;
    pub const X25: int         = 9;
    pub const INET6: int       = 10;
    pub const ROSE: int        = 11;
    pub const DECNET: int      = 12;
    pub const NETBEUI: int     = 13;
    pub const SECURITY: int    = 14;
    pub const KEY: int         = 15;
    pub const NETLINK: int     = 16;
    pub const PACKET: int      = 17;
    pub const ECONET: int      = 18;
    pub const ATMSVC: int      = 19;
    pub const RDS: int         = 20;
    pub const IRDA: int        = 21;
    pub const PPPOX: int       = 22;
    pub const WANPIPE: int     = 23;
    pub const LLC: int         = 24;
    pub const IB: int          = 25;
    pub const MPLS: int        = 26;
    pub const CAN: int         = 29;
    pub const TIPC: int        = 30;
    pub const BLUETOOTH: int   = 31;
    pub const IUCV: int        = 32;
    pub const RXRPC: int       = 33;
    pub const ISDN: int        = 34;
    pub const PHONET: int      = 35;
    pub const IEEE802154: int  = 36;
    pub const CAIF: int        = 37;
    pub const ALG: int         = 38;
    pub const VSOCK: int       = 39;
    pub const KCM: int         = 40;
    pub const QIPCRTR: int     = 41;
    pub const SMC: int         = 42;
    pub const XDP: int         = 43;
}


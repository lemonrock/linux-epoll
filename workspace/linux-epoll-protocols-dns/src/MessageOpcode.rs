// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// The value encoded in this opcode is NOT the same as that defined by IANA; instead the encoded values are the IANA values left-shifted by 3.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MessageOpcode
{
	/// Query.
	///
	/// Defined in RFC 1035.
	Query = 0,

	/// Inverse Query ('IQuery').
	///
	/// Defined in RFC 1035; made obsolete by RFC 3425.
	InverseQuery = 1 << 3,

	/// Status.
	///
	/// Defined in RFC 1035.
	Status = 2 << 3,

	/// Notify.
	///
	/// Defined in RFC 1996.
	Notify = 4 << 3,

	/// Update.
	///
	/// Defined in RFC 2136.
	Update = 5 << 3,

	/// DNS Stateful Operations, DSO.
	///
	/// Defined in [RFC-ietf-dnsop-session-signal-20](http://www.iana.org/go/draft-ietf-dnsop-session-signal-20).
	DnsStatefulOperations = 6 << 3,
}

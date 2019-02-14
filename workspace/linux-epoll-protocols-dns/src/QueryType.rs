// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// See RFC 6895, Section 3.1, Paragraph 3.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
struct QueryType(pub [u8; 2]);

impl QueryType
{
	/// `IXFR`.
	///
	/// This is a `QTYPE` and is only valid in a query section.
	///
	/// Defined in RFC 1035, clarified in RFC 6895, Section 3.1, Paragraph 3.
	pub const IXFR_higher: u8 = 0x00;
	pub const IXFR_lower: u8 = 0xFB;
	pub const IXFR: Self = Self([Self::IXFR_higher, Self::IXFR_lower]);

	/// `AXFR`.
	///
	/// This is a `QTYPE` and is only valid in a query section.
	///
	/// Defined in RFC 1035, clarified in RFC 6895, Section 3.1, Paragraph 3.
	pub const AXFR_higher: u8 = 0x00;
	pub const AXFR_lower: u8 = 0xFC;
	pub const AXFR: Self = Self([Self::AXFR_higher, Self::AXFR_lower]);

	/// `MAILB`.
	///
	/// This is a `QTYPE` and is only valid in a query section.
	///
	/// Clarified in RFC 6895, Section 3.1, Paragraph 3.
	///
	/// Defined in RFC 883 and made effectively obsolete by RFC 2505.
	pub const MAILB_higher: u8 = 0x00;
	pub const MAILB_lower: u8 = 0xFD;
	pub const MAILB: Self = Self([Self::MAILB_higher, Self::MAILB_lower]);

	/// `MAILA`.
	///
	/// This is a `QTYPE` and is only valid in a query section.
	///
	/// Clarified in RFC 6895, Section 3.1, Paragraph 3.
	///
	/// Defined in RFC 883 and made obsolete in RFC 973.
	pub const MAILA_higher: u8 = 0x00;
	pub const MAILA_lower: u8 = 0xFE;
	pub const MAILA: Self = Self([Self::MAILA_higher, Self::MAILA_lower]);

	/// `*`.
	///
	/// This is a `QTYPE` and is only valid in a query section.
	///
	/// Returns all records of all types currently cached for a domain name from a name server; if no records are cached then the request will be forwarded on.
	///
	/// Also known as `ANY` and as `ALL`.
	///
	/// Defined in RFC 1035, clarified in RFC 6895, Section 3.1, Paragraph 3.
	pub const Asterisk_higher: u8 = 0x00;
	pub const Asterisk_lower: u8 = 0xFF;
	pub const Asterisk: Self = Self([Self::Asterisk_higher, Self::Asterisk_lower]);
}
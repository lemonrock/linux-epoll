// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// See RFC 6895, Section 3.1, paragraph 3.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
struct MetaType(pub [u8; 2]);

impl MetaType
{
	/// Defined in RFC 6891.
	pub const OPT_higher: u8 = 0x00;
	pub const OPT_lower: u8 = 0x41;
	pub const OPT: Self = Self([Self::OPT_higher, Self::OPT_lower]);

	/// Defined in RFC 2930.
	pub const TKEY_higher: u8 = 0x00;
	pub const TKEY_lower: u8 = 0xF9;
	pub const TKEY: Self = Self([Self::TKEY_higher, Self::TKEY_lower]);

	/// Defined in RFC 2845.
	pub const TSIG_higher: u8 = 0x00;
	pub const TSIG_lower: u8 = 0xFA;
	pub const TSIG: Self = Self([Self::TSIG_higher, Self::TSIG_lower]);
}

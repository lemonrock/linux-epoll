// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A 31 bit unsigned integer that specifies the time interval (in seconds) that a resource record (RR) may be cached before it should be discarded.
///
/// See <https://tools.ietf.org/html/rfc2181#section-8>.
///
/// Zero values are interpreted to mean that the resource record can only be used for the transaction in progress, and should not be cached.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
pub struct TimeToLiveInSeconds([u8; 4]);

impl From<[u8; 4]> for TimeToLiveInSeconds
{
	#[inline(always)]
	fn from(seconds: [u8; 4]) -> Self
	{
		Self(seconds)
	}
}

impl Into<u32> for TimeToLiveInSeconds
{
	#[inline(always)]
	fn into(self) -> u32
	{
		let value = self.0.from_network_endian_to_native_endian();

		// RFC 2181, Section 8; if the top bit is set, the value is zero.
		if unlikely!(value & 0x80000000 != 0)
		{
			0
		}
		else
		{
			value
		}
	}
}

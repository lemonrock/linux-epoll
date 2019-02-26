// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A 31 bit unsigned integer that specifies a time in seconds.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
pub struct TimeInSeconds([u8; 4]);

impl From<[u8; 4]> for TimeInSeconds
{
	#[inline(always)]
	fn from(seconds: [u8; 4]) -> Self
	{
		Self(seconds)
	}
}

impl Into<u32> for TimeInSeconds
{
	#[inline(always)]
	fn into(self) -> u32
	{
		let value = u32::from_be_bytes(self.0);

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

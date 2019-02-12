// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A 32 bit unsigned integer that specifies the time interval (in seconds) that a resource record (RR) may be cached before it should be discarded
///
/// Zero values are interpreted to mean that the RR can only be used for the transaction in progress, and should not be cached.
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

impl From<u32> for TimeToLiveInSeconds
{
	#[inline(always)]
	fn from(seconds: u32) -> Self
	{
		Self(seconds.to_be_bytes())
	}
}

impl Into<u32> for TimeToLiveInSeconds
{
	#[inline(always)]
	fn into(self) -> u32
	{
		u32::from_be_bytes(self.0)
	}
}

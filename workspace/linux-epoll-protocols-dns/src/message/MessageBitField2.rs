// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MessageBitField2(u8);

impl MessageBitField2
{
	#[inline(always)]
	fn recursion_available(self) -> bool
	{
		self.0 & 0b1000_000 != 0
	}

	#[inline(always)]
	fn z(self) -> bool
	{
		self.0 & 0b0100_0000 != 0
	}

	#[inline(always)]
	fn authentic_data(self) -> bool
	{
		self.0 & 0b0010_0000 != 0
	}

	#[inline(always)]
	fn checking_disabled(self) -> bool
	{
		self.0 & 0b0001_0000 != 0
	}

	#[inline(always)]
	fn raw_response_code(self) -> u8
	{
		self.0 & 0b0000_1111
	}
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MessageBitField1(u8);

impl MessageBitField1
{
	#[inline(always)]
	fn query_response(self) -> MessageType
	{
		unsafe { transmute(self.0 & 0b1000_000) }
	}
	
	#[inline(always)]
	fn raw_opcode(self) -> u8
	{
		(self.0 & 0b0111_1000) >> 3
	}
	
	#[inline(always)]
	fn assumed_to_be_valid_opcode(self) -> MessageOpcode
	{
		unsafe { transmute(self.0 & 0b0111_1000) }
	}
	
	#[inline(always)]
	fn authoritative_answer(self) -> bool
	{
		self.0 & 0b0000_0100 != 0
	}

	#[inline(always)]
	fn is_truncated(self) -> bool
	{
		self.0 & 0b0000_0010 != 0
	}
	
	#[inline(always)]
	fn recursion_desired(self) -> bool
	{
		self.0 & 0b0000_0001 != 0
	}
}

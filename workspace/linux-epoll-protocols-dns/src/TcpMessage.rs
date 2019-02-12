// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[repr(C, packed)]
struct TcpMessage
{
	length: [u8; 2],
	message: Message,
}

impl TcpMessage
{
	/// Length (excluding the two byte length field).
	#[inline(always)]
	pub fn length(&self) -> u16
	{
		u16::from_be_bytes(self.length)
	}

	/// Set length.
	#[inline(always)]
	pub fn set_length(&mut self, length: u16)
	{
		self.length = length.to_be_bytes()
	}

	/// Message.
	#[inline(always)]
	pub fn message(&self) -> &Message
	{
		&self.message
	}

	/// Mutable message.
	#[inline(always)]
	pub fn message_mutable(&mut self) -> &mut Message
	{
		&mut self.message
	}
}

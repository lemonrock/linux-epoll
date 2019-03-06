// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[repr(C, packed)]
pub(crate) struct TcpMessage
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
		self.length.from_network_endian_to_native_endian()
	}

	/// Message.
	#[inline(always)]
	pub fn message(&self) -> &Message
	{
		&self.message
	}
}

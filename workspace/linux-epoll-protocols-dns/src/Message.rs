// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


extern
{
	type Message;
}

impl Message
{
	/// Message header.
	#[inline(always)]
	pub fn message_header(&self) -> &MessageHeader
	{
		unsafe { & * (self as *const Self as *const MessageHeader) }
	}

	/// Mutable message header.
	#[inline(always)]
	pub fn message_header_mutable(&mut self) -> &mut MessageHeader
	{
		unsafe { &mut * (self as *mut Self as *mut MessageHeader) }
	}

	/// Message body.
	#[inline(always)]
	pub fn message_body(&self) -> &MessageBody
	{
		unsafe { & * ((self as *const Self as usize + size_of::<MessageHeader>()) as *const MessageBody) }
	}

	/// Mutable message body.
	#[inline(always)]
	pub fn message_body_mutable(&mut self) -> &mut MessageBody
	{
		unsafe { &mut * ((self as *mut Self as usize + size_of::<MessageHeader>()) as *mut MessageBody) }
	}
}

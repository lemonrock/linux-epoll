// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug)]
struct MessageHeader
{
	compressed_type_identifier: CompressedTypeIdentifier,
	number_of_bytes_padding_to_align_message_contents: u8,
	message_contents_size: u16,
}

impl MessageHeader
{
	const Size: usize = size_of::<Self>();

	#[inline(always)]
	fn message_contents(&mut self) -> &mut VariablySized
	{
		unsafe { &mut * (self.message_contents_pointer() as *mut VariablySized) }
	}

	#[inline(always)]
	fn total_message_size_including_message_header(&self) -> usize
	{
		let end_of_message_contents_pointer = self.message_contents_pointer() + self.message_contents_size as usize;
		let next_message_pointer = round_up_to_alignment::<Self>(end_of_message_contents_pointer);

		next_message_pointer - self.base_pointer()
	}

	#[inline(always)]
	fn message_contents_pointer(&self) -> usize
	{
		self.base_pointer() + self.number_of_bytes_padding_to_align_message_contents()
	}

	#[inline(always)]
	fn base_pointer(&self) -> usize
	{
		self as *const Self as usize
	}

	#[inline(always)]
	fn number_of_bytes_padding_to_align_message_contents(&self) -> usize
	{
		self.number_of_bytes_padding_to_align_message_contents as usize
	}
}

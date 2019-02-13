// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.
//


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextStringsIterator<'a>
{
	next_string_starts_at_pointer: usize,
	end_of_resource_data_pointer: usize,
	marker: PhantomData<&'a ()>,
}

impl<'a> Iterator for TextStringsIterator<'a>
{
	type Item = Result<&'a [u8], DnsProtocolError>;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if unlikely!(self.next_string_starts_at_pointer == self.end_of_resource_data_pointer)
		{
			return None
		}

		let text_string = unsafe { & * (self.next_string_starts_at_pointer as *const TextString) };
		self.next_string_starts_at_pointer += 1;

		let length = text_string.length as usize;
		let result = if unlikely!(self.next_string_starts_at_pointer + length > self.end_of_resource_data_pointer)
		{
			Err(DnsProtocolError::TextRecordStringLengthIncorrect)
		}
		else
		{
			Ok(text_string.as_slice(length))
		};
		Some(result)
	}
}

impl<'a> TextStringsIterator<'a>
{
	#[inline(always)]
	fn new(resource_data: &'a [u8]) -> Result<Self, DnsProtocolError>
	{
		let length = resource_data.len();
		if unlikely!(length == 0)
		{
			return Err(DnsProtocolError::ResourceRecordForTypeTXTHasNoTextStrings)
		}

		let next_string_starts_at_pointer = resource_data.as_ptr() as usize;

		Ok
		(
			Self
			{
				next_string_starts_at_pointer,
				end_of_resource_data_pointer: next_string_starts_at_pointer + length,
				marker: PhantomData,
			}
		)
	}
}

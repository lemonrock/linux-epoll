// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedLabels
{
	pub(crate) start_of_message_pointer: usize,

	parsed_labels: HashSet<u16>,
}

impl ParsedLabels
{
	#[inline(always)]
	pub(crate) fn new(start_of_message_pointer: usize) -> Self
	{
		Self
		{
			start_of_message_pointer,
			parsed_labels: HashSet::with_capacity(128),
		}
	}

	#[inline(always)]
	pub(crate) fn parse_name_in_slice_with_nothing_left<'message>(&mut self, slice: &'message [u8]) -> Result<WithCompressionParsedNameIterator<'message>, DnsProtocolError>
	{
		match self.parse_name_in_slice(slice)
		{
			Err(error) => Err(error),

			Ok((parsed_name_iterator, end_of_name_pointer)) => if unlikely!(end_of_name_pointer - slice.len() != slice.as_ptr() as usize)
			{
				Err(NameWasNotLongEnough)
			}
			else
			{
				Ok(parsed_name_iterator)
			}
		}
	}

	#[inline(always)]
	pub(crate) fn parse_name_in_slice<'message>(&mut self, slice: &'message [u8]) -> Result<(WithCompressionParsedNameIterator<'message>, usize), DnsProtocolError>
	{
		let length = slice.len();
		if unlikely!(length == 0)
		{
			return Err(NameIsEmpty)
		}

		let start_of_name_pointer = slice.as_ptr() as usize;
		self.parse_name(start_of_name_pointer, start_of_name_pointer + length)
	}

	#[inline(always)]
	pub(crate) fn parse_name<'message>(&mut self, start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<(WithCompressionParsedNameIterator<'message>, usize), DnsProtocolError>
	{
		WithCompressionParsedNameIterator::parse_with_compression(self, start_of_name_pointer, end_of_data_section_containing_name_pointer)
	}

	#[inline(always)]
	pub(crate) fn guard_pointer_points_forward_to_unparsed_data(&self, offset: usize, current_label_starts_at_pointer: usize) -> Result<(), DnsProtocolError>
	{
		let pointer_points_forward_to_unparsed_data = self.start_of_message_pointer + offset >= current_label_starts_at_pointer;

		if unlikely!(pointer_points_forward_to_unparsed_data)
		{
			Err(LabelPointerOffsetPointsForwardToUnparsedData)
		}
		else
		{
			Ok(())
		}
	}

	#[inline(always)]
	pub(crate) fn guard_contains(&self, offset: usize, next_label_starts_at_pointer: usize) -> Result<(), DnsProtocolError>
	{
		debug_assert!(offset <= ::std::u16::MAX as usize, "offset is larger than ::std::u16::MAX");

		if unlikely!(self.start_of_message_pointer + offset >= next_label_starts_at_pointer)
		{
			return Err(LabelPointerOffsetPointsForwardToUnparsedData)
		}

		let compressed_offset = (self.start_of_message_pointer + offset) as u16;
		if likely!(self.parsed_labels.contains(&(compressed_offset)))
		{
			Ok(())
		}
		else
		{
			Err(InvalidLabelPointerOffset(offset))
		}
	}

	#[inline(always)]
	pub(crate) fn insert(&mut self, label_starts_at_pointer: usize)
	{
		debug_assert!(label_starts_at_pointer >= self.start_of_message_pointer, "offset occurs before start_of_message_pointer");

		let offset = label_starts_at_pointer - self.start_of_message_pointer;
		debug_assert!(offset <= ::std::u16::MAX as usize, "offset is larger than ::std::u16::MAX");

		let compressed_offset = offset as u16;

		let was_first_time_this_compressed_offset_was_inserted = self.parsed_labels.insert(compressed_offset);
		debug_assert!(was_first_time_this_compressed_offset_was_inserted, "There was a previous label at this offset {}", offset);
	}
}

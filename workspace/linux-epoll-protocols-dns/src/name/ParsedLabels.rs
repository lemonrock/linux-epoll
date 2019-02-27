// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


pub(crate) struct ParsedLabels<'a>
{
	start_of_message_pointer: usize,

	/// Alternative hash maps include https://github.com/mersinvald/primitive-map-rs and heapless (https://japaric.github.io/heapless/heapless/struct.IndexMap.html).
	parsed_labels: HashMap<u16, ParsedLabel<'a>>,
}

impl<'a> ParsedLabels<'a>
{
	#[inline(always)]
	pub(crate) fn parse_name_in_slice_with_nothing_left(&mut self, slice: &'a [u8]) -> Result<ParsedNameIterator<'a>, DnsProtocolError>
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
	pub(crate) fn parse_name_in_slice(&mut self, slice: &'a [u8]) -> Result<(ParsedNameIterator<'a>, usize), DnsProtocolError>
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
	pub(crate) fn parse_name(&mut self, start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<(ParsedNameIterator<'a>, usize), DnsProtocolError>
	{
		ParsedNameIterator::parse(self, start_of_name_pointer, end_of_data_section_containing_name_pointer)
	}

	#[inline(always)]
	fn get(&self, offset: usize, next_label_starts_at_pointer: usize) -> Result<&ParsedLabel<'a>, DnsProtocolError>
	{
		debug_assert!(offset <= ::std::u16::MAX as usize, "offset is larger than ::std::u16::MAX");

		if unlikely!(self.start_of_message_pointer + offset >= next_label_starts_at_pointer)
		{
			return Err(LabelPointerOffsetPointsForwardToUnparsedData)
		}

		let compressed_offset = (self.start_of_message_pointer + offset) as u16;
		self.parsed_labels.get(&(compressed_offset)).ok_or(InvalidLabelPointerOffset(offset))
	}

	#[inline(always)]
	fn insert(&mut self, parsed_label: ParsedLabel<'a>) -> &ParsedLabel<'a>
	{
		let label_starts_at_pointer = parsed_label.this_label_starts_at_pointer();

		debug_assert!(label_starts_at_pointer >= self.start_of_message_pointer, "offset occurs before start_of_message_pointer");
		debug_assert!(label_starts_at_pointer - self.start_of_message_pointer <= ::std::u16::MAX as usize, "offset is larger than ::std::u16::MAX");

		let compressed_offset = (label_starts_at_pointer - self.start_of_message_pointer) as u16;

		use self::Entry::*;

		match self.parsed_labels.entry(compressed_offset)
		{
			Vacant(vacant) =>
			{
				vacant.insert(parsed_label)
			}

			#[cfg(debug_assertions)] _ => panic!("There was a previous label at this offset"),
			#[cfg(not(debug_assertions))] _ => unreachable!()
		}
	}
}

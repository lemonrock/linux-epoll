// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Will always contain at least one label.
///
/// The final label is always an empty (root) label, ie `LabelBytes.is_empty()` is `true`.
///
/// RFC 2065 asserts that the maximum number of labels is 127; this makes sense if every label bar the last (which is Root) is 1 byte long and so occupies 2 bytes.
/// However, the maximum reasonable length is an IPv6 reverse DNS look up, which requires 34 labels (32 for each nibble and 2 for `ip6.arpa`).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParsedNameIterator<'a>(Option<NonNull<ParsedLabel<'a>>>);

impl<'a> Iterator for ParsedNameIterator<'a>
{
	type Item = LabelBytes<'a>;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		match self.0.take()
		{
			None => None,

			Some(non_null_parsed_label) =>
			{
				let parsed_label_reference = unsafe { non_null_parsed_label.as_ref() };
				self.current = parsed_label_reference.next();

				Some(parsed_label_reference.label_bytes())
			}
		}
	}
}

impl<'a> ParsedNameIterator<'a>
{
	fn parse(parsed_labels: &mut ParsedLabels<'a>, start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<(Self, usize), DnsProtocolError>
	{
		use self::DnsProtocolError::*;

		let maximum_potential_name_length = Self::maximum_potential_name_length(start_of_name_pointer, end_of_data_section_containing_name_pointer);
		let end_of_name_data_pointer = start_of_name_pointer + maximum_potential_name_length;

		let mut parsed_name = Self::default();
		let mut current_label_starts_at_pointer = start_of_name_pointer;

		let initial_parsed_label = ParsedLabel::Fake;

		let mut previous_parsed_label_reference = &initial_parsed_label;

		let true_end_of_name_pointer = loop
		{
			if unlikely!(current_label_starts_at_pointer == end_of_name_data_pointer)
			{
				return Err(DnsProtocolError::NoTerminalRootLabel)
			}

			let label = unsafe { & * (current_label_starts_at_pointer as *const Label) };

			const Bytes: u8 = 0b00;
			const CompressedOffsetPointer: u8 = 0b11;

			match label.raw_kind()
			{
				Bytes =>
				{
					let length = label.length_or_offset();
					let parsed_label = ParsedLabel::new(label.bytes(), length);

					let next_label_starts_at_pointer = parsed_label.next_label_starts_at_pointer();

					if unlikely!(next_label_starts_at_pointer > end_of_name_data_pointer)
					{
						return Err(DnsProtocolError::LabelLengthOverflows)
					}

					let parsed_label_reference = parsed_labels.insert(parsed_label);
					previous_parsed_label_reference.set_next(parsed_label_reference);
					previous_parsed_label_reference = parsed_label_reference;

					if unlikely!(parsed_label_reference.is_terminal_root_label())
					{
						break next_label_starts_at_pointer
					}
					else
					{
						current_label_starts_at_pointer = next_label_starts_at_pointer
					}
				}

				CompressedOffsetPointer =>
				{
					let offset = label.length_or_offset();

					if unlikely!(parsed_labels.start_of_message_pointer + offset >= current_label_starts_at_pointer)
					{
						return Err(DnsProtocolError::LabelPointerOffsetPointsForwardToUnparsedData)
					}

					let parsed_label_reference = parsed_labels.get(offset, current_label_starts_at_pointer)?;

					previous_parsed_label_reference.set_next(parsed_label_reference);

					break current_label_starts_at_pointer + 1
				}

				unsupported @ _ => return Err(UnsupportedNameLabelKind(unsupported))
			}
		};

		Ok(Self(initial_parsed_label.next()), true_end_of_name_pointer)
	}

	#[inline(always)]
	fn maximum_potential_name_length(start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<usize, DnsProtocolError>
	{
		debug_assert!(end_of_data_section_containing_name_pointer >= start_of_name_pointer, "end_of_data_section_containing_name_pointer occurs before start_of_name_pointer");

		if unlikely!(start_of_name_pointer == end_of_data_section_containing_name_pointer)
		{
			return Err(DnsProtocolError::NameIsEmpty)
		}

		let unconstrained_maximum_potential_name_length = end_of_data_section_containing_name_pointer - start_of_name_pointer;
		const MaximumLengthOfName: usize = 255;
		Ok(min(unconstrained_maximum_potential_name_length, MaximumLengthOfName))
	}
}

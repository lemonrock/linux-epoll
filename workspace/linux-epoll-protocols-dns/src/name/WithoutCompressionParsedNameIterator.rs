// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Will always contain at least one label.
///
/// The final label is always an empty (root) label, ie `LabelBytes.is_empty()` is `true`.
///
/// RFC 2065 asserts that the maximum number of labels is 127; this makes sense if every label bar the last (which is Root) is 1 byte long and so occupies 2 bytes.
/// However, the maximum reasonable length is an IPv6 reverse DNS look up, which requires 34 labels (32 for each nibble and 2 for `ip6.arpa`) of a `SRV` entry such as `_mqtt._tcp`, thus 36 labels.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WithoutCompressionParsedNameIterator<'a>
{
	pointer_to_label: usize,
	marker: PhantomData<&'a ()>,
}

impl<'a> Iterator for WithoutCompressionParsedNameIterator<'a>
{
	type Item = LabelBytes<'a>;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if unlikely!(self.pointer_to_label == 0)
		{
			return None
		}

		const LabelKindSize: usize = 1;
		let label = ParsedNameIterator::label(self.pointer_to_label);
		let length = label.length();

		let label_bytes = Some(unsafe { from_raw_parts((self.pointer_to_label + LabelKindSize) as *const u8, length) });

		if unlikely!(length == 0)
		{
			self.pointer_to_label = 0
		}
		else
		{
			self.pointer_to_label += LabelKindSize + length;
		}

		label_bytes
	}
}

impl<'a> WithoutCompressionParsedNameIterator<'a>
{
	#[inline(always)]
	pub(crate) fn parse_without_compression(start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<(Self, usize), DnsProtocolError>
	{
		let maximum_for_end_of_name_pointer = ParsedNameIterator::maximum_for_end_of_name_pointer(start_of_name_pointer, end_of_data_section_containing_name_pointer)?;

		let mut current_label_starts_at_pointer = start_of_name_pointer;
		let initial_parsed_label = ParsedLabel::Fake;
		let mut previous_parsed_label_reference = &initial_parsed_label;

		let true_end_of_name_pointer = loop
		{
			if unlikely!(current_label_starts_at_pointer == maximum_for_end_of_name_pointer)
			{
				return Err(NoTerminalRootLabel)
			}

			let label = ParsedNameIterator::label(current_label_starts_at_pointer);

			match label.raw_kind()
			{
				ParsedNameIterator::Bytes =>
				{
					let length = label.length();
					let parsed_label = ParsedLabel::new(label.bytes(), length);

					let next_label_starts_at_pointer = parsed_label.next_label_starts_at_pointer();

					if unlikely!(next_label_starts_at_pointer > maximum_for_end_of_name_pointer)
					{
						return Err(LabelLengthOverflows)
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

				ParsedNameIterator::Extended => return Err(ExtendedNameLabelsAreUnused),

				ParsedNameIterator::Unallocated => return Err(UnallocatedNameLabelsAreUnused),

				ParsedNameIterator::CompressedOffsetPointer => return Err(CompressedNameLabelsAreDisallowedInThisResourceRecord),
			}
		};

		let this = Self
		{
			pointer_to_label: start_of_name_pointer,
			marker: PhantomData,
		};

		Ok((this, true_end_of_name_pointer))
	}
}

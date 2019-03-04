// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Will always contain at least one label.
///
/// The final label is always an empty (root) label, ie `LabelBytes.is_empty()` is `true`.
///
/// RFC 2065 asserts that the maximum number of labels is 127; this makes sense if every label bar the last (which is Root) is 1 byte long and so occupies 2 bytes.
/// However, the maximum reasonable length is an IPv6 reverse DNS look up, which requires 34 labels (32 for each nibble and 2 for `ip6.arpa`) of a `SRV` entry such as `_mqtt._tcp`, thus 36 labels.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WithCompressionParsedNameIterator<'message>
{
	pointer_to_label: usize,
	marker: PhantomData<&'message ()>,
	start_of_message_pointer: usize,
}

impl<'message> Iterator for WithCompressionParsedNameIterator<'message>
{
	type Item = LabelBytes<'message>;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let (label, pointer_to_label) = iterator_next_label!(self);

		use self::LabelKind::*;
		match label.raw_kind()
		{
			Bytes => bytes_label!(self, label, pointer_to_label),

			CompressedOffsetPointer =>
			{
				self.pointer_to_label = self.start_of_message_pointer + label.offset();
				let label = Label::label(pointer_to_label);
				bytes_label!(self, label, pointer_to_label)
			}

			_ => unreachable!(),
		}
	}
}

macro_rules! guard_current_label
{
	($current_label_starts_at_pointer: ident, $maximum_for_end_of_name_pointer: ident) =>
	{
		{
			if unlikely!($current_label_starts_at_pointer == $maximum_for_end_of_name_pointer)
			{
				return Err(NoTerminalRootLabel)
			}
			Label::label($current_label_starts_at_pointer)
		}
	}
}

impl<'message> WithCompressionParsedNameIterator<'message>
{
	pub(crate) fn parse_with_compression(parsed_labels: &mut ParsedLabels, start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<(Self, usize), DnsProtocolError>
	{
		let maximum_for_end_of_name_pointer = Name::maximum_for_end_of_name_pointer(start_of_name_pointer, end_of_data_section_containing_name_pointer)?;
		let mut current_label_starts_at_pointer = start_of_name_pointer;

		let true_end_of_name_pointer = loop
		{
			let label = guard_current_label!(current_label_starts_at_pointer, maximum_for_end_of_name_pointer);

			use self::LabelKind::*;

			match label.raw_kind()
			{
				Bytes =>
				{
					// NOTE: This design will NOT record a root label as `parse_bytes_label!` calls `break` internally; there is never any point in compressing by using a pointer (2 bytes) to a root label (1 byte).
					let previous_label_started_at_pointer = current_label_starts_at_pointer;
					current_label_starts_at_pointer = parse_bytes_label!(label, current_label_starts_at_pointer, maximum_for_end_of_name_pointer);
					parsed_labels.insert(previous_label_started_at_pointer)
				}

				Extended => return Err(ExtendedNameLabelsAreUnused),

				Unallocated => return Err(UnallocatedNameLabelsAreUnused),

				CompressedOffsetPointer =>
				{
					const length: usize = 1;
					let next_label_starts_at_pointer = guard_next_label_starts_at_pointer!(current_label_starts_at_pointer, length, maximum_for_end_of_name_pointer);

					let offset = label.offset();
					parsed_labels.guard_pointer_points_forward_to_unparsed_data(offset, current_label_starts_at_pointer)?;
					parsed_labels.guard_contains(offset, current_label_starts_at_pointer)?;

					break next_label_starts_at_pointer
				}
			}
		};

		let this = Self
		{
			pointer_to_label: start_of_name_pointer,
			marker: PhantomData,
			start_of_message_pointer: parsed_labels.start_of_message_pointer,
		};

		Ok((this, true_end_of_name_pointer))
	}
}

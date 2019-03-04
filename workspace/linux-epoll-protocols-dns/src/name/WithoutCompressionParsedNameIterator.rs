// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Will always contain at least one label.
///
/// The final label is always an empty (root) label, ie `LabelBytes.is_empty()` is `true`.
///
/// RFC 2065 asserts that the maximum number of labels is 127; this makes sense if every label bar the last (which is Root) is 1 byte long and so occupies 2 bytes.
/// However, the maximum reasonable length is an IPv6 reverse DNS look up, which requires 34 labels (32 for each nibble and 2 for `ip6.arpa`) of a `SRV` entry such as `_mqtt._tcp`, thus 36 labels.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WithoutCompressionParsedNameIterator<'message>
{
	pointer_to_label: usize,
	marker: PhantomData<&'message ()>,
}

impl<'message> Iterator for WithoutCompressionParsedNameIterator<'message>
{
	type Item = LabelBytes<'message>;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let (label, pointer_to_label) = iterator_next_label!(self);
		bytes_label!(self, label, pointer_to_label)
	}
}

impl<'message> WithoutCompressionParsedNameIterator<'message>
{
	#[inline(always)]
	pub(crate) fn parse_without_compression(start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<(Self, usize), DnsProtocolError>
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
					current_label_starts_at_pointer = parse_bytes_label!(label, current_label_starts_at_pointer, maximum_for_end_of_name_pointer)
				}

				Extended => return Err(ExtendedNameLabelsAreUnused),

				Unallocated => return Err(UnallocatedNameLabelsAreUnused),

				CompressedOffsetPointer => return Err(CompressedNameLabelsAreDisallowedInThisResourceRecord),
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

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Does not contain the root, empty label.
///
/// RFC 2065 asserts that the maximum number of labels is 127; this makes sense if every label bar the last (which is Root) is 1 byte long and so occupies 2 bytes.
/// However, the maximum reasonable length is an IPv6 reverse DNS look up, which requires 33 labels (32 for each nibble and 2 for `ip6.arpa` less 1 for the omitted root label) of a `SRV` entry such as `_mqtt._tcp`, thus 35 labels.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WithCompressionParsedNameIterator<'message>
{
	pointer_to_label: usize,
	pub(crate) number_of_labels: u8,
	pub(crate) name_length: u8,
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

impl<'message> WithCompressionParsedNameIterator<'message>
{
	pub(crate) const MaximumNumberOfLabels: usize = 127;

	pub(crate) fn parse_with_compression(parsed_labels: &mut ParsedLabels, start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<(Self, usize), DnsProtocolError>
	{
		macro_rules! compressed_implementation
		{
			($label: ident, $current_label_starts_at_pointer: ident, $maximum_for_end_of_name_pointer: ident, $start_of_name_pointer: ident, $labels_register_reference: ident, $parsed_labels: ident, $number_of_labels: ident, $name_length: ident) =>
			{
				{
					const length: usize = 1;
					let true_end_of_name_pointer = guard_next_label_starts_at_pointer!($current_label_starts_at_pointer, length, $maximum_for_end_of_name_pointer);

					// NOTE: Must call `guard_next_label_starts_at_pointer!` before `offset()` to make sure that the `length` that offset occupies is present (ie there's enough bytes).
					let offset = $label.offset();
					let (number_of_labels_, name_length_) = parsed_labels.guard(offset, $start_of_name_pointer, $labels_register_reference)?;
					$number_of_labels = number_of_labels_;
					$name_length = name_length_;

					break true_end_of_name_pointer
				}
			}
		}

		let mut labels_register: LabelsRegister = ArrayVec::new();
		let labels_register_reference = &mut labels_register;
		let (true_end_of_name_pointer, number_of_labels, name_length) = parse_name!(start_of_name_pointer, end_of_data_section_containing_name_pointer, labels_register_reference, parsed_labels, parse_and_register_bytes_label, compressed_implementation);

		let this = Self
		{
			pointer_to_label: start_of_name_pointer,
			number_of_labels,
			name_length,
			marker: PhantomData,
			start_of_message_pointer: parsed_labels.start_of_message_pointer,
		};

		Ok((this, true_end_of_name_pointer))
	}
}

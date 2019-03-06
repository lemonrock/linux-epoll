// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Does not contain the root, empty label.
///
/// RFC 2065 asserts that the maximum number of labels is 127; this makes sense if every label bar the last (which is Root) is 1 byte long and so occupies 2 bytes.
/// However, the maximum reasonable length is an IPv6 reverse DNS look up, which requires 33 labels (32 for each nibble and 2 for `ip6.arpa` less 1 for the omitted root label) of a `SRV` entry such as `_mqtt._tcp`, thus 35 labels.
#[derive(Default, Debug, Clone)]
pub struct WithoutCompressionParsedName<'message>
{
	pub(crate) number_of_labels: u8,
	pub(crate) name_length: u8,
	iterator: WithoutCompressionParsedNameIterator<'message>,
}

impl<'message> IntoIterator for WithoutCompressionParsedName<'message>
{
	type Item = LabelBytes<'message>;

	type IntoIter = WithoutCompressionParsedNameIterator<'message>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter
	{
		self.iterator
	}
}

impl<'a, 'message> IntoIterator for &'a WithoutCompressionParsedName<'message>
{
	type Item = LabelBytes<'message>;

	type IntoIter = WithoutCompressionParsedNameIterator<'message>;

	#[inline(always)]
	fn into_iter(&self) -> Self::IntoIter
	{
		self.iterator.clone()
	}
}

impl<'message> PartialEq for WithoutCompressionParsedName<'message>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		if likely!(self.number_of_labels != other.number_of_labels)
		{
			return false
		}
		if likely!(self.name_length != other.name_length)
		{
			return false
		}
		let left = self.into_iter();
		let right = other.into_iter();
		for (left_label, right_label) in left.zip(right)
		{
			if likely!(left_label != right_label)
			{
				return false
			}
		}
		true
	}
}

impl<'message> Eq for WithoutCompressionParsedName<'message>
{
}

impl<'message> Hash for WithoutCompressionParsedName<'message>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		for label in self
		{
			label.hash(state)
		}
	}
}

impl<'message> WithoutCompressionParsedName<'message>
{
	#[inline(always)]
	pub(crate) fn parse_without_compression_but_register_labels_for_compression(parsed_labels: &mut ParsedLabels, start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<(Self, usize), DnsProtocolError>
	{
		let mut labels_register: LabelsRegister = ArrayVec::new();
		let labels_register_reference = &mut labels_register;
		let (pointer_to_label, true_end_of_name_pointer, number_of_labels, name_length) = parse_name!(start_of_name_pointer, end_of_data_section_containing_name_pointer, labels_register_reference, parsed_labels, parse_and_register_bytes_label, compressed_error);

		let this = Self
		{
			number_of_labels,
			name_length,
			iterator: WithoutCompressionParsedNameIterator
			{
				pointer_to_label,
				marker: PhantomData,
			}
		};

		Ok((this, true_end_of_name_pointer))
	}

	#[inline(always)]
	pub(crate) fn parse_without_compression(start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<(Self, usize), DnsProtocolError>
	{
		const _labels_register_reference: usize = 0;
		const _parsed_labels: usize = 0;
		let (pointer_to_label, true_end_of_name_pointer, number_of_labels, name_length) = parse_name!(start_of_name_pointer, end_of_data_section_containing_name_pointer, _labels_register_reference, _parsed_labels, parse_and_ignore_bytes_label, compressed_error);

		let this = Self
		{
			number_of_labels,
			name_length,
			iterator: WithoutCompressionParsedNameIterator
			{
				pointer_to_label,
				marker: PhantomData,
			}
		};

		Ok((this, true_end_of_name_pointer))
	}
}

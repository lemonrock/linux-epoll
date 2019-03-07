// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Iterating this *excludes* the root label.
#[derive(Default, Debug,  Clone)]
pub struct WithCompressionParsedName<'message>
{
	/// This *includes* the root label.
	pub(crate) number_of_labels: u8,

	/// This *includes* the root label.
	pub(crate) name_length: u8,

	iterator: WithCompressionParsedNameIterator<'message>,
}

impl<'message> IntoIterator for WithCompressionParsedName<'message>
{
	type Item = LabelBytes<'message>;

	type IntoIter = WithCompressionParsedNameIterator<'message>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter
	{
		self.iterator
	}
}

impl<'a, 'message> IntoIterator for &'a WithCompressionParsedName<'message>
{
	type Item = LabelBytes<'message>;

	type IntoIter = WithCompressionParsedNameIterator<'message>;

	#[inline(always)]
	fn into_iter(&self) -> Self::IntoIter
	{
		self.iterator.clone()
	}
}

impl<'message> PartialEq for WithCompressionParsedName<'message>
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

impl<'message> PartialEq<WithoutCompressionParsedName<'message>> for WithCompressionParsedName<'message>
{
	#[inline(always)]
	fn eq(&self, other: &WithoutCompressionParsedName<'message>) -> bool
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

impl<'message> PartialEq<UncompressedName> for WithCompressionParsedName<'message>
{
	#[inline(always)]
	fn eq(&self, other: &UncompressedName) -> bool
	{
		self.eq(other.name())
	}
}

impl<'message> Eq for WithCompressionParsedName<'message>
{
}

impl<'message> Hash for WithCompressionParsedName<'message>
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

impl<'message> WithCompressionParsedName<'message>
{
	pub(crate) const MaximumNumberOfLabels: usize = 127;

	#[inline(always)]
	pub(crate) fn new(number_of_labels: u8, name_length: u8, pointer_to_label: usize, start_of_message_pointer: usize) -> Self
	{
		Self
		{
			number_of_labels,
			name_length,
			iterator: WithCompressionParsedNameIterator::new(pointer_to_label, start_of_message_pointer),
		}
	}

	pub(crate) fn parse_with_compression(parsed_labels: &mut ParsedLabels, start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<(Self, usize), DnsProtocolError>
	{
		macro_rules! compressed_implementation
		{
			($label: ident, $current_label_starts_at_pointer: ident, $maximum_for_end_of_name_pointer: ident, $start_of_name_pointer: ident, $pointer_to_label: ident, $labels_register_reference: ident, $parsed_labels: ident, $number_of_labels: ident, $name_length: ident) =>
			{
				{
					const length: usize = 1;
					let true_end_of_name_pointer = guard_next_label_starts_at_pointer!($current_label_starts_at_pointer, length, $maximum_for_end_of_name_pointer);

					// NOTE: Must call `guard_next_label_starts_at_pointer!` before `offset()` to make sure that the `length` that offset occupies is present (ie there's enough bytes).
					let offset = $label.offset();
					let (points_to_label_at_, number_of_labels_, name_length_) = parsed_labels.guard(offset, $start_of_name_pointer, $labels_register_reference)?;

					// NOTE: Optimisation to avoid dereferencing a pointer to an entire name.
					if likely!($number_of_labels == 0)
					{
						$pointer_to_label = points_to_label_at_;
					}

					$number_of_labels = number_of_labels_;
					$name_length = name_length_;

					break true_end_of_name_pointer
				}
			}
		}

		let mut labels_register: LabelsRegister = ArrayVec::new();
		let labels_register_reference = &mut labels_register;
		let (pointer_to_label, true_end_of_name_pointer, number_of_labels, name_length) = parse_name!(start_of_name_pointer, end_of_data_section_containing_name_pointer, labels_register_reference, parsed_labels, parse_and_register_bytes_label, compressed_implementation);

		let this = Self::new(number_of_labels, name_length, pointer_to_label, parsed_labels.start_of_message_pointer);

		Ok((this, true_end_of_name_pointer))
	}
}

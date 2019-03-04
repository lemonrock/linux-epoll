// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


extern
{
	/// A Name consists of one or more labels.
	pub(crate) type Name;
}

impl Name
{
	/// The smallest Name consists of one label, which is the Root label, which is one byte.
	pub(crate) const MinimumSize: usize = 1;

	#[inline(always)]
	pub(crate) fn maximum_for_end_of_name_pointer(start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<usize, DnsProtocolError>
	{
		let maximum_potential_name_length = Self::maximum_potential_name_length(start_of_name_pointer, end_of_data_section_containing_name_pointer)?;
		let end_of_name_data_pointer = start_of_name_pointer + maximum_potential_name_length;
		Ok(end_of_name_data_pointer)
	}

	#[inline(always)]
	fn maximum_potential_name_length(start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<usize, DnsProtocolError>
	{
		debug_assert!(end_of_data_section_containing_name_pointer >= start_of_name_pointer, "end_of_data_section_containing_name_pointer occurs before start_of_name_pointer");

		if unlikely!(start_of_name_pointer == end_of_data_section_containing_name_pointer)
		{
			return Err(NameIsEmpty)
		}

		let unconstrained_maximum_potential_name_length = end_of_data_section_containing_name_pointer - start_of_name_pointer;
		const MaximumLengthOfName: usize = 255;
		Ok(min(unconstrained_maximum_potential_name_length, MaximumLengthOfName))
	}
}

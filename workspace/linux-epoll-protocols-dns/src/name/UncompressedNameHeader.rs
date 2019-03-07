// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct UncompressedNameHeader
{
	pointer_to_label: usize,

	/// This *includes* the root label.
	number_of_labels: u8,

	/// This *includes* the root label.
	name_length: u8,
}

impl UncompressedNameHeader
{
	#[inline(always)]
	fn new(pointer_to_label: usize, number_of_labels: u8, name_length: u8) -> Self
	{
		Self
			{
				pointer_to_label,
				number_of_labels,
				name_length,
			}
	}

	#[inline(always)]
	fn iterator<'message>(&self) -> WithoutCompressionParsedNameIterator<'message>
	{
		WithoutCompressionParsedNameIterator::new(self.pointer_to_label)
	}
}

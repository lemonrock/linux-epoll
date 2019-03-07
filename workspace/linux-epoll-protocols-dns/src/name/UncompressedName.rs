// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct UncompressedName<Allocator: Alloc>
{
	allocator: Allocator,
	pointer: NonNull<(UncompressedNameHeader, UpTo255Bytes)>,
}

impl<Allocator: Alloc> Drop for UncompressedName<Allocator>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let pointer = self.pointer.unsafe_cast_mut_non_null::<u8>();
		let layout = Self::layout(self.header_mut().name_length as usize);
		self.allocator.dealloc(pointer, layout)
	}
}

impl<'message, Allocator: Alloc> IntoIterator for &'message UncompressedName<Allocator>
{
	type Item = LabelBytes<'message>;

	type IntoIter = WithoutCompressionParsedNameIterator<'message>;

	#[inline(always)]
	fn into_iter(&self) -> Self::IntoIter
	{
		self.header_mut().iterator::<'message>()
	}
}

impl<Allocator: Alloc> PartialEq for UncompressedName<Allocator>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		let this_number_of_labels = self.qname.number_of_labels;
		let other_number_of_labels = other.qname.number_of_labels;
		if unlikely!(this_number_of_labels != other_number_of_labels)
		{
			return false
		}

		let this_name_length = self.qname.name_length;
		let other_name_length = other.qname.name_length;
		if unlikely!(this_name_length != other_name_length)
		{
			return false
		}

		let name_length = this_name_length as usize;

		&self.raw_qname[ .. name_length] == &other.raw_qname[ .. name_length]
	}
}

impl<'message, Allocator: Alloc> PartialEq<WithoutCompressionParsedName<'message>> for UncompressedName<Allocator>
{
	#[inline(always)]
	fn eq(&self, other: &WithCompressionParsedName<'message>) -> bool
	{
		other.eq(self)
	}
}

impl<'message, Allocator: Alloc> PartialEq<WithCompressionParsedName<'message>> for UncompressedName<Allocator>
{
	#[inline(always)]
	fn eq(&self, other: &WithCompressionParsedName<'message>) -> bool
	{
		other.eq(self)
	}
}

impl<Allocator: Alloc> Eq for UncompressedName<Allocator>
{
}

impl<Allocator: Alloc> Hash for UncompressedName<Allocator>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		let this_name_length = self.qname.name_length as usize;
		(&self.raw_qname[ .. name_length]).hash(state)
	}
}

impl<Allocator: Alloc> UncompressedName<Allocator>
{
	// TODO: conversions from a dotted name, and from a dotted name with / without a trailing .
	// TODO: conversions from a relative name + domain name (which may not have a trailing .)

	/// Creates a new instance.
	#[inline(always)]
	pub fn new(mut allocator: Allocator, name: &[u8], number_of_labels: u8) -> Option<Self>
	{
		debug_assert_ne!(number_of_labels, 0, "number_of_labels is zero");
		debug_assert!(number_of_labels <= 127, "number_of_labels `{}` exceeds 127", number_of_labels);

		let name_length_usize = name.len();
		debug_assert_ne!(name_length_usize, 0, "name is empty");
		debug_assert!(name_length_usize <= 255, "name `{}` exceeds 255 bytes", name_length_usize);
		debug_assert_eq!(name.get(name_length_usize - 1), 0x00, "final byte of name is not 0x00 (a root label)");

		let allocator: &mut Global = &mut Global;
		let allocation = allocator.alloc(Self::layout(name_length_usize));

		let this = match allocation
		{
			None => return None,
			Some(non_null_u8) => Self
			{
				allocator,
				pointer: unsafe { transmute(non_null_u8) },
			},
		};

		let pointer_to_label = this.qname_bytes_mut().as_usize_pointer_mut();
		unsafe { write(this.header_mut(), UncompressedNameHeader::new(pointer_to_label, number_of_labels, name_length_usize as u8)) }
		unsafe { copy_nonoverlapping(name.as_ptr(), this.qname_bytes_mut(), name_length_usize) }

		Ok(this)
	}

	/// Obtains a name.
	#[inline(always)]
	pub fn name<'query>(&'query self) -> WithoutCompressionParsedName<'query>
	{
		let pointer_to_label = self.as_usize_pointer() + size_of::<UncompressedNameFixed>();
		WithoutCompressionParsedName::new(self.fixed.number_of_labels, self.fixed.name_length, pointer_to_label)
	}

	#[inline(always)]
	fn layout(name_length_usize: usize) -> Layout
	{
		unsafe { Layout::from_size_align_unchecked(size_of::<UncompressedNameHeader>() + name_length_usize, align_of::<UncompressedNameHeader>()) }
	}

	#[inline(always)]
	fn header_mut(&mut self) -> &mut UncompressedNameHeader
	{
		&mut self.pointer_mut().0
	}

	#[inline(always)]
	fn qname_bytes_mut(&mut self) -> &mut UpTo255Bytes
	{
		&mut self.pointer_mut().1
	}

	#[inline(always)]
	fn pointer_mut(&mut self) -> &mut (UncompressedNameHeader, UpTo255Bytes)
	{
		unsafe { &mut * self.pointer.as_ptr() }
	}
}

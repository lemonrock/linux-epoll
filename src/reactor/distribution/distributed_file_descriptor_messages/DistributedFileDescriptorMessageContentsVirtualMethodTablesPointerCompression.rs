// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Maps a virtual method table pointer to a 256-bit (u8 sized) index in either direction (from pointer to index, or index to pointer).
///
/// When sending messages where the contents can vary, this allows a 8:1 memory saving on virtual memory pointers at the cost of an additional look up.
///
/// A maximum of 255 registrations can be made as a sentinel value is reserved.
pub struct DistributedFileDescriptorMessageContentsVirtualMethodTablesPointerCompression
{
	virtual_method_table_to_index: HashMap<NonNull<()>, u8>,
	index_to_virtual_method_table: [NonNull<()>; size_of::<u8>()],
	next_index: u8,
}

impl DistributedFileDescriptorMessageContentsVirtualMethodTablesPointerCompression
{
	const InclusiveMaximum: u8 = (size_of::<u8>() - 1) as u8;

	#[inline(always)]
	pub fn new() -> Self
	{
		Self
		{
			virtual_method_table_to_index: HashMap::with_capacity(size_of::<u8>()),
			index_to_virtual_method_table: unsafe { uninitialized() },
			next_index: 0,
		}
	}

	/// Register a concrete type of `Concrete` (the type system can not enforce this).
	///
	/// Panics in debug if too many registrations occur or the type is registered more than once.
	#[inline(always)]
	pub fn register<Concrete: DistributedFileDescriptorMessageContents>(&mut self)
	{
		debug_assert_ne!(self.next_index, Self::InclusiveMaximum, "Maximum number of registrations (255) has been reached");

		let vtable = Self::find_virtual_method_table::<Concrete>();

		let previous = self.virtual_method_table_to_index.insert(vtable, self.next_index);
		debug_assert_eq!(previous, None, "This type has already been registered");

		unsafe { write(self.index_to_virtual_method_table.get_unchecked_mut(self.next_index as usize), vtable) };

		self.next_index += 1;
	}

	/// Gets the virtual method table for this index.
	///
	/// Very fast array look up.
	///
	/// Panics in debug if index is too large.
	#[inline(always)]
	pub fn get_virtual_method_table(&self, index: u8) -> NonNull<()>
	{
		if cfg!(debug_assertions)
		{
			debug_assert!(index < self.next_index);
			self.index_to_virtual_method_table[index as usize]
		}
		else
		{
			unsafe { *self.index_to_virtual_method_table.get_unchecked(index as usize) }
		}
	}

	/// Gets the index for the concrete type `Concrete`.
	///
	/// Highly inefficient and the results should be cached.
	///
	/// Panics if there is no registration for the `virtual_method_table`.
	#[inline(always)]
	pub fn get_index<Concrete: DistributedFileDescriptorMessageContents>(&self, virtual_method_table: NonNull<()>) -> u8
	{
		let virtual_method_table = Self::find_virtual_method_table::<Concrete>();

		self.virtual_method_table_to_index.get(virtual_method_table).unwrap()
	}

	#[inline(always)]
	fn find_virtual_method_table<Concrete: DistributedFileDescriptorMessageContents>() -> NonNull<()>
	{
		let concrete: Concrete = unsafe { uninitialized() };
		let fat_pointer: &dyn DistributedFileDescriptorMessageContents = &concrete;
		let trait_object: TraitObject = unsafe { transmute(fat_pointer) };
		forget(concrete);
		unsafe { NonNull::new_unchecked(trait_object.vtable) }
	}
}

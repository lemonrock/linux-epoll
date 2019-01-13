// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug)]
struct ArenaElement<T: Sized>
{
	next_available_slot_index: Cell<ArenaIndex>,
	value: ManuallyDrop<UnsafeCell<T>>,
}

impl<T: Sized> Drop for ArenaElement<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.is_occupied()
		{
			self.drop_occupied()
		}
	}
}

impl<T: Sized> ArenaElement<T>
{
	const IsFullyAllocatedNextAvailableSlotIndexSentinel: ArenaIndex = ::std::usize::MAX;

	const IsOccupiedNextAvailableSlotIndexSentinel: ArenaIndex = ::std::usize::MAX - 1;

	#[inline(always)]
	fn new(unoccupied_next_available_slot_index: ArenaIndex) -> Self
	{
		Self
		{
			next_available_slot_index: Cell::new(unoccupied_next_available_slot_index),
			value: ManuallyDrop::new(UnsafeCell::new(unsafe { uninitialized() })),
		}
	}

	#[inline(always)]
	fn fully_allocated() -> Self
	{
		Self::new(Self::IsFullyAllocatedNextAvailableSlotIndexSentinel)
	}

	#[inline(always)]
	fn first(maximum_number_of_elements: usize) -> ArenaIndex
	{
		if unlikely!(maximum_number_of_elements == 0)
		{
			Self::IsFullyAllocatedNextAvailableSlotIndexSentinel
		}
		else
		{
			1
		}
	}

	#[inline(always)]
	const fn is_fully_allocated(next_available_slot_index: ArenaIndex) -> bool
	{
		next_available_slot_index == Self::IsFullyAllocatedNextAvailableSlotIndexSentinel
	}

	#[inline(always)]
	fn is_occupied(&self) -> bool
	{
		self.next_available_slot_index() == Self::IsOccupiedNextAvailableSlotIndexSentinel
	}

	#[inline(always)]
	fn next_available_slot_index(&self) -> ArenaIndex
	{
		self.next_available_slot_index.get()
	}

	#[inline(always)]
	fn set_unoccupied_next_available_slot_index(&self, arena_index: ArenaIndex)
	{
		self.next_available_slot_index.set(arena_index)
	}

	#[inline(always)]
	fn set_occupied_next_available_slot_index(&self)
	{
		self.next_available_slot_index.set(Self::IsOccupiedNextAvailableSlotIndexSentinel)
	}

	#[inline(always)]
	fn get_occupied(&self) -> *mut T
	{
		self.value.get()
	}

	#[inline(always)]
	fn get_occupied_mut_ref(&self) -> &mut T
	{
		unsafe { &mut * self.get_occupied() }
	}

	#[inline(always)]
	fn get_occupied_non_null(&self) -> NonNull<T>
	{
		unsafe { NonNull::new_unchecked(self.get_occupied()) }
	}

	#[inline(always)]
	fn drop_occupied(&self)
	{
		unsafe { drop_in_place(self.get_occupied()) }
	}

	#[inline(always)]
	fn push_into_vector(self, vector_allocation: &mut Vec<Self>)
	{
		vector_allocation.push(self)
	}

	#[inline(always)]
	fn push_into_slice(self, array_allocation: &mut [Self], index: ArenaIndex)
	{
		unsafe
		{
			let slot = array_allocation.get_unchecked_mut(index);
			(slot as *mut Self).write(self)
		}
	}

}

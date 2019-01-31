// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A simple arena.
///
/// Dropping this arena will free all memory for all elements `Holds`, irrespective if they are still being referenced.
/// Ordinarily, since an arena lasts at least as long as an `EPollFileDescriptor`, this is not an issue.
///
/// If `Holds` implements `Drop`, it will be honoured on drop of this arena.
/// It will also be honoured when `reclaim()` is called; do not call `reclaim()` after `allocate()` without initializing `Holds` to a known, valid state.
///
/// Default creates an empty arena.
#[derive(Debug)]
pub struct SimpleArena<Holds>
{
	next_available_slot_index: Cell<ArenaIndex>,

	// Sadly this causes pointer-chasing as Rust does not yet permit creation of dynamically sized types.
	allocation: Box<[ArenaElement<Holds>]>,
}

impl<Holds> Default for SimpleArena<Holds>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::new(0)
	}
}

impl<Holds> Arena<Holds> for SimpleArena<Holds>
{
	#[inline(always)]
	fn allocate(&self) -> Result<(NonNull<Holds>, ArenaIndex), ArenaAllocationError>
	{
		let next_available_slot_index = self.next_available_slot_index.get();

		if unlikely!(ArenaElement::<Holds>::is_fully_allocated(next_available_slot_index))
		{
			return Err(ArenaAllocationError::MaximumPreAllocatedMemoryReached)
		}

		let next = self.element(next_available_slot_index);
		self.next_available_slot_index.set(next.next_available_slot_index());
		next.set_occupied_next_available_slot_index();

		Ok((next.get_occupied_non_null(), next_available_slot_index))
	}

	#[inline(always)]
	fn get(&self, arena_index: ArenaIndex) -> &mut Holds
	{
		let element = self.element(arena_index);
		debug_assert!(element.is_occupied(), "arena_index was not for an occupied element");

		element.get_occupied_mut_ref()
	}

	#[inline(always)]
	fn reclaim(&self, arena_index: ArenaIndex)
	{
		let allocated = self.element(arena_index);

		allocated.drop_occupied();
		allocated.set_unoccupied_next_available_slot_index(self.next_available_slot_index.get());

		self.next_available_slot_index.set(arena_index);
	}
}

impl<Holds> SimpleArena<Holds>
{
	/// Creates a new instance.
	///
	/// `maximum_number_of_elements` may validly be zero.
	pub fn new(maximum_number_of_elements: usize) -> Self
	{
		Self
		{
			next_available_slot_index: Cell::new(ArenaElement::<Holds>::first(maximum_number_of_elements)),

			allocation:
			{
				let mut vector_allocation = Vec::with_capacity(maximum_number_of_elements);

				for index in 1 .. maximum_number_of_elements
				{
					ArenaElement::<Holds>::new(index).push_into_vector(&mut vector_allocation)
				}
				ArenaElement::<Holds>::fully_allocated().push_into_vector(&mut vector_allocation);

				vector_allocation.into_boxed_slice()
			}
		}
	}

	#[inline(always)]
	fn element(&self, arena_index: ArenaIndex) -> &ArenaElement<Holds>
	{
		debug_assert_ne!(arena_index, ArenaElement::<Holds>::IsFullyAllocatedNextAvailableSlotIndexSentinel, "Should never get IsFullyAllocatedNextAvailableSlotIndexSentinel for `arena_index`");
		debug_assert_ne!(arena_index, ArenaElement::<Holds>::IsOccupiedNextAvailableSlotIndexSentinel, "Should never get IsOccupiedNextAvailableSlotIndexSentinel for `arena_index`");
		debug_assert!(arena_index < self.allocation.len(), "Arena index was out-of-range");

		unsafe { self.allocation.get_unchecked(arena_index) }
	}
}

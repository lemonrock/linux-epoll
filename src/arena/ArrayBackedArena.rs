// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


macro_rules! array_backed_arena
{
	($name: ident, $size: expr) =>
	{
		/// An array-backed arena of size $size.
		///
		/// Dropping this arena will free all memory for all elements `Hold`, irrespective if they are still being referenced.
		/// Ordinarily, since an arena lasts at least as long as an epoll file descriptor, this is not an issue.
		///
		/// If `Hold` implements `Drop`, it will be honoured on drop of this arena.
		/// It will also be honoured when `reclaim()` is called; do not call `reclaim()` after `allocate()` without initializing `Hold` to a known, valid state.
		///
		/// Default creates an empty arena.
		pub struct $name<Holds: Reactor>
		where Holds::FileDescriptor: FromRawFd
		{
			next_available_slot_index: Cell<ArenaIndex>,

			allocation: [ArenaElement<Holds>; $size],
		}

		impl<Holds: Reactor> Debug for $name<Holds>
		where Holds::FileDescriptor: FromRawFd
		{
			#[inline(always)]
			fn fmt(&self, f: &mut Formatter) -> fmt::Result
			{
				write!(f, "$name {{ next_available_slot: {:?}, allocation: [_; {:?}] }}", self.next_available_slot_index, $size)
			}
		}

		impl<Holds: Reactor> Default for $name<Holds>
		where Holds::FileDescriptor: FromRawFd
		{
			#[inline(always)]
			fn default() -> Self
			{
				Self::new()
			}
		}

		impl<Holds: Reactor> Arena<Holds> for $name<Holds>
		where Holds::FileDescriptor: FromRawFd
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
			fn get(&self, arena_index: ArenaIndex, raw_file_descriptor: RawFd) -> (&mut Holds, Holds::FileDescriptor)
			{
				let file_descriptor = unsafe { Holds::FileDescriptor::from_raw_fd(raw_file_descriptor) };

				let element = self.element(arena_index);
				debug_assert!(element.is_occupied(), "arena_index was not for an occupied element");

				(element.get_occupied_mut_ref(), file_descriptor)
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

		impl<Holds: Reactor> $name<Holds>
		where Holds::FileDescriptor: FromRawFd
		{
			const Size: usize = $size;

			/// Creates a new instance.
			pub fn new() -> Self
			{
				const maximum_number_of_elements: usize = $size;

				Self
				{
					next_available_slot_index: Cell::new(ArenaElement::<Holds>::first(maximum_number_of_elements)),

					allocation:
					{
						let mut array_allocation: [ArenaElement<Holds>; maximum_number_of_elements] = unsafe { uninitialized() };

						for index in 1 .. maximum_number_of_elements
						{
							ArenaElement::<Holds>::new(index).push_into_slice(&mut array_allocation[..], index - 1)
						}
						ArenaElement::<Holds>::fully_allocated().push_into_slice(&mut array_allocation[..], maximum_number_of_elements - 1);

						array_allocation
					}
				}
			}

			#[inline(always)]
			fn element(&self, arena_index: ArenaIndex) -> &ArenaElement<Holds>
			{
				debug_assert_ne!(arena_index, ArenaElement::<Holds>::IsFullyAllocatedNextAvailableSlotIndexSentinel, "Should never get IsFullyAllocatedNextAvailableSlotIndexSentinel for `arena_index`");
				debug_assert_ne!(arena_index, ArenaElement::<Holds>::IsOccupiedNextAvailableSlotIndexSentinel, "Should never get IsOccupiedNextAvailableSlotIndexSentinel for `arena_index`");
				debug_assert!(arena_index < Self::Size, "Arena index was out-of-range");

				unsafe { self.allocation.get_unchecked(arena_index) }
			}
		}
	}
}

array_backed_arena!(ArrayBacked0Arena, 0);

array_backed_arena!(ArrayBacked1Arena, 1);

array_backed_arena!(ArrayBacked2Arena, 2);

array_backed_arena!(ArrayBacked4Arena, 4);

array_backed_arena!(ArrayBacked8Arena, 8);

array_backed_arena!(ArrayBacked16Arena, 16);

array_backed_arena!(ArrayBacked32Arena, 32);

array_backed_arena!(ArrayBacked64Arena, 64);

array_backed_arena!(ArrayBacked128Arena, 128);

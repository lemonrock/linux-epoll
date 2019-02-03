// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// An arena.
pub trait Arena<Holds>: Sized
{
	/// Used to hold this arena instance.
	///
	/// The naive implementation is to Box this instance.
	///
	/// However, a more efficient strategy, particularly for large allocations, would be to use mmap or an unsized type.
	///
	/// If overriding `Self::drop_from_non_null()` make sure this is also overridden or memory leaks (or worse) will occur.
	#[inline(always)]
	fn to_non_null(self) -> NonNull<Self>
	{
		unsafe{ NonNull::new_unchecked(Box::into_raw(Box::new(self))) }
	}

	/// Used to drop an arena instance previously converted to a pointer with `Self::to_non_null()`.
	///
	/// The naive implementation is to un-Box this instance.
	///
	/// If overriding `Self::to_null_null()` make sure this is also overridden or memory leaks (or worse) will occur.
	#[inline(always)]
	fn drop_from_non_null(this: NonNull<Self>)
	{
		unsafe { drop(Box::from_raw(this.as_ptr())) }
	}

	/// Allocate a `Holds` within this arena.
	///
	/// The returned pointer should be considered uninitialized.
	///
	/// None is returned if allocation failed.
	fn allocate(&self) -> Result<(NonNull<Holds>, ArenaIndex), ArenaAllocationError>;

	/// Get a `Holds` within this arena.
	fn get(&self, arena_index: ArenaIndex) -> &mut Holds;

	/// Reclaim (drop, destroy or recycle) `Holds` within this arena.
	fn reclaim(&self, arena_index: ArenaIndex);
}

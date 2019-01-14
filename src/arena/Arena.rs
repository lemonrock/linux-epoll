// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// An arena.
pub trait Arena<Holds: Reactor>
{
	/// Allocate a `Holds` within this arena.
	///
	/// The returned pointer should be considered uninitialized.
	///
	/// None is returned if allocation failed.
	fn allocate(&self) -> Result<(NonNull<Holds>, ArenaIndex), ArenaAllocationError>;

	/// Get a `Holds` within this arena and rehydrate its raw file descriptor.
	///
	/// The `arena_index` can be tagged if desired to internally access different instances of the arena, eg to optimize memory usage for two internal implementations of `Holds`.
	/// The `arena_index` can be used by file descriptors that are wrapped in enums.
	fn get(&self, arena_index: ArenaIndex, raw_file_descriptor: RawFd) -> (&mut Holds, Holds::FileDescriptor);

	/// Reclaim (drop, destroy or recycle) `Holds` within this arena.
	fn reclaim(&self, arena_index: ArenaIndex);
}

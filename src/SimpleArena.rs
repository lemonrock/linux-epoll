// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A simple arena.
#[derive(Debug)]
pub struct SimpleArena<Holds: UsesFileDescriptor>
where Holds::FileDescriptor: FromRawFd
{
	marker: PhantomData<Holds>,
}

impl<Holds: UsesFileDescriptor> Arena<Holds> for SimpleArena<Holds>
where Holds::FileDescriptor: FromRawFd
{
	#[inline(always)]
	fn allocate(&self) -> Result<(NonNull<Holds>, ArenaIndex), ()>
	{
		unimplemented!();
	}

	#[inline(always)]
	fn get(&self, arena_index: ArenaIndex, raw_file_descriptor: RawFd) -> (&mut Holds, Holds::FileDescriptor)
	{
		let file_descriptor = unsafe { Holds::FileDescriptor::from_raw_fd(raw_file_descriptor) };
		unimplemented!();
	}

	#[inline(always)]
	fn reclaim(&self, arena_index: ArenaIndex)
	{
		unimplemented!();
	}
}

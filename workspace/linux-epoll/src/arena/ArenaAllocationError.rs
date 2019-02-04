// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An error that can occur during allocation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArenaAllocationError
{
	/// No more slots available.
	MaximumPreAllocatedMemoryReached,
}

impl Display for ArenaAllocationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<ArenaAllocationError as Debug>::fmt(self, f)
	}
}

impl error::Error for ArenaAllocationError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(error::Error + 'static)>
	{
		use self::ArenaAllocationError::*;

		match self
		{
			&MaximumPreAllocatedMemoryReached => None,
		}
	}
}

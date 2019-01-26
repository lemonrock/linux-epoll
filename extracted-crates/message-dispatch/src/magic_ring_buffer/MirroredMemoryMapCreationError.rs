// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// An error that can occur during creation of a file descriptor instance.
#[derive(Debug)]
pub enum MirroredMemoryMapCreationError
{
	/// Could not open memory mapping file.
	CouldNotOpenMemoryMappingFile(io::Error),

	/// Could not unlink memory mapping file.
	CouldNotUnlinkMemoryMappingFile(io::Error),

	/// Could not truncate memory mapping file.
	CouldNotTruncateMemoryMappingFile(io::Error),

	/// The per-process limit on the number of open file descriptors would be exceeded.
	PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,

	/// Kernel would be out of memory or the process limit on the number of memory mappings has been reached.
	KernelWouldBeOutOfMemory,
}

impl Display for MirroredMemoryMapCreationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<MirroredMemoryMapCreationError as Debug>::fmt(self, f)
	}
}

impl error::Error for MirroredMemoryMapCreationError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(error::Error + 'static)>
	{
		use self::MirroredMemoryMapCreationError::*;

		match self
		{
			&CouldNotOpenMemoryMappingFile(ref error) => Some(error),

			&CouldNotUnlinkMemoryMappingFile(ref error) => Some(error),

			&CouldNotTruncateMemoryMappingFile(ref error) => Some(error),

			&PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded => None,

			&KernelWouldBeOutOfMemory => None,
		}
	}
}
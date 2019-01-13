// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A dummy structure suitable when not using a particular kind of file descriptor.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unused;

impl FromRawFd for Unused
{
	#[inline(always)]
	unsafe fn from_raw_fd(_raw_file_descriptor: RawFd) -> Self
	{
		Self
	}
}

impl UsesFileDescriptor for Unused
{
	type FileDescriptor = Self;
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ByteCounter
{
	total_bytes_written: usize,
	total_bytes_read: usize,
}

impl ByteCounter
{
	#[inline(always)]
	pub(crate) fn bytes_written(&mut self, bytes_written: usize)
	{
		self.total_bytes_written += bytes_written;
	}

	#[inline(always)]
	pub(crate) fn bytes_read(&mut self, bytes_read: usize)
	{
		self.total_bytes_read += bytes_read;
	}
}

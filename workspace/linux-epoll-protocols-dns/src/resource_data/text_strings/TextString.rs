// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


struct TextString
{
	length: u8,
	bytes: UpTo255Bytes,
}

impl TextString
{
	#[inline(always)]
	fn as_slice(&self, length: usize) -> &[u8]
	{
		unsafe { from_raw_parts((&self.bytes) as *const UpTo255Bytes as *const u8, length) }
	}
}

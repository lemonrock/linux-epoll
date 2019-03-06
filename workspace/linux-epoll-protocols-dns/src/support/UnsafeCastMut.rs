// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


pub(crate) trait UnsafeCastMut: UnsafeCast
{
	#[inline(always)]
	fn as_usize_pointer_mut(&mut self) -> usize
	{
		self as *mut Self as *mut () as usize
	}

	#[inline(always)]
	fn unsafe_cast_mut<To>(&mut self) -> &mut To
	{
		unsafe { &mut * (self.as_usize_pointer_mut() as *mut To) }
	}

	#[inline(always)]
	fn unsafe_cast_slice_mut<To>(&mut self, length: usize) -> &mut [To]
	{
		unsafe { from_raw_parts_mut(self.unsafe_cast_mut::<To>(), length) }
	}
}

impl<T> UnsafeCastMut for T
{
}

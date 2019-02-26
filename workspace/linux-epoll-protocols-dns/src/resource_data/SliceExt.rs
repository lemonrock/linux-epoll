// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


pub(crate) trait SliceExt
{
	fn pointer(&self) -> usize;

	fn end_pointer(&self) -> usize;

	fn cast<T>(&self) -> &T;

	fn u8(&self, offset: usize) -> u8;

	fn u16(&self, offset: usize) -> u16;
}

impl<'a> SliceExt for &'a [u8]
{
	#[inline(always)]
	fn pointer(&self) -> usize
	{
		self.as_ptr() as usize
	}

	#[inline(always)]
	fn end_pointer(&self) -> usize
	{
		self.pointer() + self.len()
	}

	#[inline(always)]
	fn cast<T>(&self) -> &T
	{
		unsafe { & * (self.as_ptr() as *const T) }
	}

	#[inline(always)]
	fn u8(&self, offset: usize) -> u8
	{
		unsafe { * self.get_unchecked(offset) }
	}

	#[inline(always)]
	fn u16(&self, offset: usize) -> u16
	{
		u16::from_be_bytes(unsafe { *(self.get_unchecked(0) as *const u8 as *const [u8; 2]) })
	}
}

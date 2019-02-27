// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


pub(crate) trait SliceExt
{
	fn pointer(&self) -> usize;

	fn end_pointer(&self) -> usize;

	#[inline(always)]
	fn cast<T>(&self, offset: usize) -> &T
	{
		unsafe { & * (self.get_::<T>(offset)) }
	}

	#[inline(always)]
	fn u8(&self, offset: usize) -> u8
	{
		self.value_::<u8>(offset)
	}

	#[inline(always)]
	fn u16(&self, offset: usize) -> u16
	{
		u16::from_be_bytes(self.value_::<[u8; size_of::<u16>()]>(offset))
	}

	#[inline(always)]
	fn u32(&self, offset: usize) -> u32
	{
		u32::from_be_bytes(self.value_::<[u8; size_of::<u32>()]>(offset))
	}

	#[inline(always)]
	fn u64(&self, offset: usize) -> u64
	{
		u64::from_be_bytes(self.value_::<[u8; size_of::<u64>()]>(offset))
	}

	#[inline(always)]
	fn value<T>(&self, offset: usize) -> T
	{
		unsafe { *self.get_::<T>(offset) }
	}

	#[doc(hidden)]
	fn get_<T>(&self, offset: usize) -> *const T;
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
	fn get_<T>(&self, offset: usize) -> *const T
	{
		(unsafe { self.get_unchecked(offset) }) as *const T
	}
}

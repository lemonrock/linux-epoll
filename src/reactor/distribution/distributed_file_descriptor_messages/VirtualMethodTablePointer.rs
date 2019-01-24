// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct VirtualMethodTablePointer(NonNull<()>);

impl Into<*mut ()> for VirtualMethodTablePointer
{
	#[inline(always)]
	fn into(self) -> *mut ()
	{
		self.0.as_ptr()
	}
}

impl VirtualMethodTablePointer
{
	#[inline(always)]
	pub fn drop_in_place_function_pointer<T>(self) -> *const fn(*mut T)
	{
		unsafe { *(self.0.as_ptr() as *const () as *const fn(*mut T)) }
	}

	#[inline(always)]
	pub fn size(self) -> usize
	{
		unsafe { *(self.0.as_ptr().add(1) as *mut usize) }
	}

	#[inline(always)]
	pub fn alignment(self) -> usize
	{
		unsafe { *(self.0.as_ptr().add(2) as *mut usize) }
	}

	#[inline(always)]
	pub fn first_trait_function_pointer(self) -> *const ()
	{
		let raw_pointer = unsafe { *(self.0.as_ptr().add(3) as *usize) };
		raw_pointer as *const ()
	}
}

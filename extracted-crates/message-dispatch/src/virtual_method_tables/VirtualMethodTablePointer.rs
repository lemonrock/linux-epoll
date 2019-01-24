// This file is part of message-dispatch. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT. No part of message-dispatch, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of message-dispatch. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT.


/// Represents a Rust virtual method table pointer.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VirtualMethodTablePointer(NonNull<()>);

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
	/// The function pointer to drop an instance in place.
	#[inline(always)]
	pub fn drop_in_place_function_pointer<T>(self) -> fn(*mut T)
	{
		let raw_pointer = unsafe { *(self.0.as_ptr() as *const () as *const usize)};
		unsafe { transmute(raw_pointer) }
	}

	/// Size.
	#[inline(always)]
	pub fn size(self) -> usize
	{
		unsafe { *(self.0.as_ptr().add(1) as *mut usize) }
	}

	/// Alignment.
	#[inline(always)]
	pub fn alignment(self) -> usize
	{
		unsafe { *(self.0.as_ptr().add(2) as *mut usize) }
	}

	/// A pointer to the first function in the trait definition.
	#[inline(always)]
	pub fn first_trait_function_pointer(self) -> usize
	{
		let raw_pointer = unsafe { *(self.0.as_ptr().add(3) as *mut usize) };
		raw_pointer
	}
}

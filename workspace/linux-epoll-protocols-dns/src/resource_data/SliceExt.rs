// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


pub(crate) trait SliceExt
{
	fn pointer(&self) -> usize;

	fn end_pointer(&self) -> usize;

	/// RFC 4034, Appendix B.
	#[inline(always)]
	fn key_tag(&self) -> KeyTag;

	#[inline(always)]
	fn cast<'a, T>(&self, offset: usize) -> &'a T
	{
		unsafe { & * (self.get_::<T>(offset)) }
	}

	#[inline(always)]
	fn u8(&self, offset: usize) -> u8
	{
		self.value::<u8>(offset)
	}

	#[inline(always)]
	fn u8_as_u32(&self, offset: usize) -> u32
	{
		self.u8(offset) as u32
	}

	#[inline(always)]
	fn u8_as_usize(&self, offset: usize) -> usize
	{
		self.u8(offset) as usize
	}

	#[inline(always)]
	fn u16(&self, offset: usize) -> u16
	{
		u16::from_be_bytes(self.value::<[u8; size_of::<u16>()]>(offset))
	}

	#[inline(always)]
	fn u16_as_u32(&self, offset: usize) -> u32
	{
		self.u16(offset) as u32
	}

	#[inline(always)]
	fn u16_as_usize(&self, offset: usize) -> usize
	{
		self.u16(offset) as usize
	}

	#[inline(always)]
	fn u32(&self, offset: usize) -> u32
	{
		u32::from_be_bytes(self.value::<[u8; size_of::<u32>()]>(offset))
	}

	#[inline(always)]
	fn u64(&self, offset: usize) -> u64
	{
		u64::from_be_bytes(self.value::<[u8; size_of::<u64>()]>(offset))
	}

	#[inline(always)]
	fn u16_network_endian(&self, offset: usize) -> u16
	{
		self.value::<u16>(offset)
	}

	#[inline(always)]
	fn value<T: Copy>(&self, offset: usize) -> T
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

	/// RFC 4034, Appendix B.
	#[inline(always)]
	fn key_tag(&self) -> KeyTag
	{
		#[inline(always)]
		fn accumulate(data: &[u8], length: usize) -> u32
		{
			let mut accumulator: u32 = 0;

			for index in 0 .. length
			{
				let value = data.u16_as_u32(index);
				accumulator += value;
			}

			accumulator
		}

		let length = self.len();

		let accumulator = if length % 2 == 0
		{
			accumulate(self, length)
		}
		else
		{
			let last = length - 1;
			accumulate(self, last) + self.u8_as_u32(last) << 8
		};

		let accumulator = accumulator + ((accumulator >> 16) & 0xFFFF);
		KeyTag((accumulator & 0xFFFF) as u16)
	}

	#[inline(always)]
	fn get_<T>(&self, offset: usize) -> *const T
	{
		(unsafe { self.get_unchecked(offset) }) as *const u8 as *const T
	}
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


struct Label
{
	bitfield: LabelBitfield,
	bytes: UpTo63Bytes,
}

impl Label
{
	/// Two bits, `u2`.
	#[inline(always)]
	fn raw_kind(&self) -> u8
	{
		self.bitfield.raw_kind()
	}

	/// Actually `u6` (an inclusive maximum of 63).
	#[inline(always)]
	fn length(&self) -> usize
	{
		self.bitfield.bottom_6_bits() as usize
	}

	/// Actually `u14`.
	#[inline(always)]
	fn offset(&self) -> u16
	{
		(self.bitfield.bottom_6_bits() as u16) << 8 | (unsafe { * (self.bytes() as *const UpTo63Bytes as *const u8) }) as u16
	}

	#[inline(always)]
	fn bytes(&self) -> &UpTo63Bytes
	{
		&self.bytes
	}
}

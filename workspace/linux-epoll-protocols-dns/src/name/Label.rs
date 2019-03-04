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
	fn raw_kind(&self) -> LabelKind
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
	fn offset(&self) -> usize
	{
		(self.bitfield.bottom_6_bits() as usize) << 8 | (unsafe { * (self.bytes() as *const UpTo63Bytes as *const u8) }) as usize
	}

	#[inline(always)]
	fn bytes(&self) -> &UpTo63Bytes
	{
		&self.bytes
	}

	#[inline(always)]
	fn label<'message>(label_starts_at_pointer: usize) -> &'message Label
	{
		unsafe { & * (label_starts_at_pointer as *const Label) }
	}
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, align(8))]
struct OnlyEverIncreasesMonotonicallyOffset(u64);

impl Add<Size> for OnlyEverIncreasesMonotonicallyOffset
{
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Size) -> Self::Output
	{
		OnlyEverIncreasesMonotonicallyOffset(self.0 + rhs.0)
	}
}

impl Into<usize> for OnlyEverIncreasesMonotonicallyOffset
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0 as usize
	}
}

impl Sub<Self> for OnlyEverIncreasesMonotonicallyOffset
{
	type Output = Size;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output
	{
		Size(self.0 - rhs.0)
	}
}

impl Rem<Size> for OnlyEverIncreasesMonotonicallyOffset
{
	type Output = Size;

	#[inline(always)]
	fn rem(self, rhs: Size) -> Self::Output
	{
		Size(self.0 % rhs.0)
	}
}

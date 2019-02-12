// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Location centimetres.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocationCentimetres(u8);

impl LocationCentimetres
{
	/// For example, the value 0x12 means 1 * 10^2 or 100cm.
	/// 0x99 means 9 * 10^9 or 90,000,000m.
	#[inline(always)]
	pub fn as_centimetres(self) -> u64
	{
		let scalar = (self.0 >> 4) as u64;

		let power_of_ten = (self.0 & b1111) as u64;

		scalar * 10.pow(power_of_ten)
	}
}

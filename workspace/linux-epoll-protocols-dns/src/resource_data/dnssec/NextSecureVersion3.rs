// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Next secure version 3 (`NSEC3`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NextSecureVersion3<'a>
{
	/// Opt-Out.
	pub opt_out: bool,

	/// Iteration count.
	pub iterations: u16,

	/// Salt.
	pub salt: &'a [u8],

	/// Next owner name, hashed.
	pub next_hashed_owner_name: NextSecureVersion3Hash<'a>,

	/// Type bitmaps.
	pub type_bitmaps: TypeBitmaps,
}

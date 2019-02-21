// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// An `IPSECKEY` resource record.
///
/// It seems to be valid to have both `gateway` and `public_key` as `None`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IpsecPublicKey<'a>
{
	/// Precedence, interpreted similarly to a `MX` record precendence (but a smaller range of possible values).
	pub precedence: u8,

	/// Gateway.
	pub gateway: Option<Gateway<'a>>,

	/// Public key, if any.
	pub public_key: Option<PublicKey<'a>>,
}

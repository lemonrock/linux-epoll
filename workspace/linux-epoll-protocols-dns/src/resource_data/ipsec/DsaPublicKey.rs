// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A DSA public key in the format stated in RFC 2536, Section 2, used for DNS IPSECKEY and (formerly) KEY resource records.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DsaPublicKey<'a>
{
	/// `T`.
	///
	/// * The size (`len()`) of `P` is `64 + T * 8`.
	/// * The size (`len()`) of `G` is `64 + T * 8`.
	/// * The size (`len()`) of `Y` is `64 + T * 8`.
	pub T: u8,

	/// `Q`
	pub Q: &'a [u8; 20],

	/// `P`.
	///
	/// The size (`len()`) of `P` is `64 + T * 8`.
	pub P: &'a [u8],

	/// `G`.
	///
	/// The size (`len()`) of `G` is `64 + T * 8`.
	pub G: &'a [u8],

	/// `Y`.
	///
	/// The size (`len()`) of `Y` is `64 + T * 8`.
	pub Y: &'a [u8],
}

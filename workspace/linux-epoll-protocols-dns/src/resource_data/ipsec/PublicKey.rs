// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// An `IPSECKEY` (or `KEY`) resource record public key.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PublicKey<'a>
{
	/// A DSA public key.
	DSA(DsaPublicKey<'a>),

	/// A RSA public key.
	RSA(RsaPublicKey<'a>),

	/// A ECDSA (Elliptic Curve DSA) public key.
	ECDSA(EcDsaPublicKey<'a>),

	/// An unknown public key.
	Unknown
	{
		/// Public key algorithm type which is not `0` (None), `1` (DSA) `2` (RSA) or `3` (ECDSA).
		public_key_algorithm_type: u8,

		/// Data.
		public_key_data: &'a [u8],
	}
}

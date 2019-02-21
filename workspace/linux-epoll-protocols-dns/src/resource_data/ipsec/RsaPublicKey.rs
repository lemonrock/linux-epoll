// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A RSA public key in the format stated in RFC 3110, Section 2, used for DNS IPSECKEY and (formerly) KEY resource records.
///
/// RFC 4025 Section 2.6 Final Paragraph increases the maximum size of `exponent` and `modulus` to 65,535 bytes (lifting the restrction of 4096 bits in RFC 3110 Section 2).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RsaPublicKey<'a>
{
	/// An unsigned variable length integer.
	///
	/// Must not start with leading zeros (`0x00`) but this is not validated or checked when data is received.
	///
	/// Will never have a length of `0`.
	pub exponent: &'a [u8],

	/// An unsigned variable length integer.
	///
	/// Must not start with leading zeros (`0x00`) but this is not validated or checked when data is received.
	///
	/// Will never have a length of `0`.
	pub modulus: &'a [u8],
}

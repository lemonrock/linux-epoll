// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// See <https://www.iana.org/assignments/dane-parameters/dane-parameters.xhtml>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MatchingType<'a>
{
	/// 'Full'.
	///
	/// No hash used; an exact match is required.
	///
	/// Defined by RFC 6698.
	NoHashUsed,

	/// 'SHA2-256'.
	///
	/// 256 bit hash by SHA2; an exact match of SHA2-256 hash digests is required.
	///
	/// Defined by RFC 6234.
	Sha2_256(&'a [u8; 256 / 8]),

	/// 'SHA2-512'.
	///
	/// 512 bit hash by SHA2; an exact match of SHA2-512 hash digests is required.
	///
	/// Defined by RFC 6234.
	Sha2_512(&'a [u8; 512 / 8]),
}

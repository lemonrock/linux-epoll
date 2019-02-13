// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// TLS-DANE record data.
#[derive(Debug)]
pub struct TlsDane<'a>
{
	/// Certificate usage.
	pub certificate_usage: CertificateUsage,

	/// Selector.
	pub selector: Selector,

	/// Matching type.
	pub matching_type: MatchingType,

	/// Certificae association data.
	///
	/// If the matching type is a hash, then the length of this digest has been validated.
	pub certificate_association_data: &'a [u8],
}

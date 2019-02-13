// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// See <https://www.iana.org/assignments/dane-parameters/dane-parameters.xhtml>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CertificateUsage
{
	/// PKIX-TA.
	///
	/// Certificate Authority constraint.
	///
	/// Defined by RFC 6698.
	PKIX_TA = 0,

	/// PKIX-EE.
	///
	/// Service certificate constraint.
	///
	/// Defined by RFC 6698.
	PKIX_EE = 1,

	/// DANE-TA.
	///
	/// Trust anchor assertion.
	///
	/// Defined by RFC 6698.
	DANE_TA = 2,

	/// DANE-EE.
	///
	/// Domain-issued certificate.
	///
	/// Defined by RFC 6698.
	DANE_EE = 3,
}

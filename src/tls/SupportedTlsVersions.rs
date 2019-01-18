// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Which TLS versions should be configured?
///
/// The default is to support TLS v1.3 and TLS v1.2.
///
/// Note that support for TLS v1.0, TLS v1.1 and legacy SSL (SSLv2 and SSLv3) is not possible.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SupportedTlsVersions
{
	/// Only support TLS v1.2.
	Only_TLS_v1_2,

	/// Only support TLS v1.3.
	Only_TLS_v1_3,

	/// Support TLS v1.3 and TLS v1.2.
	TLS_v1_3_and_TLS_v1_2,
}

impl Default for SupportedTlsVersions
{
	#[inline(always)]
	fn default() -> Self
	{
		SupportedTlsVersions::TLS_v1_3_and_TLS_v1_2
	}
}

impl SupportedTlsVersions
{
	#[inline(always)]
	pub(crate) fn versions(self) -> Vec<ProtocolVersion>
	{
		use self::SupportedTlsVersions::*;
		use self::ProtocolVersion::*;

		match self
		{
			Only_TLS_v1_2 => vec![TLSv1_2],

			Only_TLS_v1_3 => vec![TLSv1_3],

			TLS_v1_3_and_TLS_v1_2 => vec![TLSv1_3, TLSv1_2],
		}
	}
}

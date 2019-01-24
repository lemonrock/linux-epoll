// This file is part of rustls-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT. No part of rustls-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of rustls-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT.


/// Represents an error when configuring a server.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum TlsClientConfigurationError
{
	ServerCertificateAuthority(RootCertificateStoreLoadError),

	CertificateChainAndPrivateKeyError(CertificateChainAndPrivateKeyError),
}

impl Display for TlsClientConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for TlsClientConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(error::Error + 'static)>
	{
		use self::TlsClientConfigurationError::*;

		match self
		{
			&ServerCertificateAuthority(ref error) => Some(error),

			&CertificateChainAndPrivateKeyError(ref error) => Some(error),
		}
	}
}

impl From<RootCertificateStoreLoadError> for TlsClientConfigurationError
{
	#[inline(always)]
	fn from(error: RootCertificateStoreLoadError) -> Self
	{
		TlsClientConfigurationError::ServerCertificateAuthority(error)
	}
}

impl From<CertificateChainAndPrivateKeyError> for TlsClientConfigurationError
{
	#[inline(always)]
	fn from(error: CertificateChainAndPrivateKeyError) -> Self
	{
		TlsClientConfigurationError::CertificateChainAndPrivateKeyError(error)
	}
}

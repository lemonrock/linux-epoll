// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Represents an error when configuring a server.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum TlsServerConfigurationError
{
	ClientCertificateAuthority(RootCertificateStoreLoadError),

	CertificateChainAndPrivateKeyError(CertificateChainAndPrivateKeyError),

	CouldNotOpenOnlineCertificateStatusProtocolFile(io::Error),

	CouldNotReadOnlineCertificateStatusProtocolFile(io::Error),

	CouldNotOpenSignedCertificateTimestampStatusFile(io::Error),

	CouldNotReadSignedCertificateTimestampStatusFile(io::Error),

	CouldNotSetCertificateChainPrivateKeyOcspAndSct(TLSError),
}

impl Display for TlsServerConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for TlsServerConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(error::Error + 'static)>
	{
		use self::TlsServerConfigurationError::*;

		match self
		{
			&ClientCertificateAuthority(ref error) => Some(error),

			&CertificateChainAndPrivateKeyError(ref error) => Some(error),

			&CouldNotOpenOnlineCertificateStatusProtocolFile(ref error) => Some(error),

			&CouldNotReadOnlineCertificateStatusProtocolFile(ref error) => Some(error),

			&CouldNotOpenSignedCertificateTimestampStatusFile(ref error) => Some(error),

			&CouldNotReadSignedCertificateTimestampStatusFile(ref error) => Some(error),

			&CouldNotSetCertificateChainPrivateKeyOcspAndSct(ref error) => Some(error),
		}
	}
}

impl From<RootCertificateStoreLoadError> for TlsServerConfigurationError
{
	#[inline(always)]
	fn from(error: RootCertificateStoreLoadError) -> Self
	{
		TlsServerConfigurationError::ClientCertificateAuthority(error)
	}
}

impl From<CertificateChainAndPrivateKeyError> for TlsServerConfigurationError
{
	#[inline(always)]
	fn from(error: CertificateChainAndPrivateKeyError) -> Self
	{
		TlsServerConfigurationError::CertificateChainAndPrivateKeyError(error)
	}
}

impl From<TLSError> for TlsServerConfigurationError
{
	#[inline(always)]
	fn from(error: TLSError) -> Self
	{
		TlsServerConfigurationError::CouldNotSetCertificateChainPrivateKeyOcspAndSct(error)
	}
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Represents an error when configuring a server.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum TlsServerConfigurationError
{
	CouldNotOpenCertificateAuthoritiesPemFile(io::Error),

	CouldNotReadCertificateAuthoritiesPemFile,

	NoValidCertificateAuthoritiesInCertificateAuthoritiesPemFiles,

	CouldNotOpenServerCertificateFile(io::Error),

	CouldNotReadServerCertificateFile,

	CouldNotOpenServerPrivateKeyFile(io::Error),

	CouldNotReadServerPkcs8PrivateKey,

	CouldNotReadServerRsaPrivateKey,

	ThereIsNeitherAPkcs8OrRsaServerPrivateKey,

	CouldNotOpenOnlineCertificateStatusProtocolFile(io::Error),

	CouldNotReadOnlineCertificateStatusProtocolFile(io::Error),

	CouldNotOpenSignedCertificateTimestampStatusFile(io::Error),

	CouldNotReadSignedCertificateTimestampStatusFile(io::Error),

	CouldNotSetCertificateChainAndPrivateKey(TLSError),
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
			&CouldNotOpenCertificateAuthoritiesPemFile(ref error) => Some(error),

			&CouldNotReadCertificateAuthoritiesPemFile => None,

			&NoValidCertificateAuthoritiesInCertificateAuthoritiesPemFile => None,

			&CouldNotOpenServerCertificateFile(ref error) => Some(error),

			&CouldNotReadServerCertificateFile => None,

			&CouldNotOpenServerPrivateKeyFile(ref error) => Some(error),

			&CouldNotReadServerPkcs8PrivateKey => None,

			&CouldNotReadServerRsaPrivateKey => None,

			&ThereIsNeitherAPkcs8OrRsaServerPrivateKey => None,

			&CouldNotOpenOnlineCertificateStatusProtocolFile(ref error) => Some(error),

			&CouldNotReadOnlineCertificateStatusProtocolFile(ref error) => Some(error),

			&CouldNotOpenSignedCertificateTimestampStatusFile(ref error) => Some(error),

			&CouldNotReadSignedCertificateTimestampStatusFile(ref error) => Some(error),

			&CouldNotSetCertificateChainAndPrivateKey(ref error) => Some(error),
		}
	}
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Represents an error when configuring a server.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum TlsClientConfigurationError
{
	CouldNotOpenCertificateAuthoritiesPemFile(io::Error),

	CouldNotReadCertificateAuthoritiesPemFile,

	NoValidCertificateAuthoritiesInCertificateAuthoritiesPemFiles,

	CouldNotOpenClientCertificateFile(io::Error),

	CouldNotReadClientCertificateFile,

	CouldNotOpenClientPrivateKeyFile(io::Error),

	CouldNotReadClientPkcs8PrivateKey,

	CouldNotReadClientRsaPrivateKey,

	ThereIsNeitherAPkcs8OrRsaClientPrivateKey,
	
	CouldNotSetCertificateChainAndPrivateKey(TLSError),
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
			&CouldNotOpenCertificateAuthoritiesPemFile(ref error) => Some(error),

			&CouldNotReadCertificateAuthoritiesPemFile => None,

			&NoValidCertificateAuthoritiesInCertificateAuthoritiesPemFile => None,

			&CouldNotOpenClientCertificateFile(ref error) => Some(error),

			&CouldNotReadClientCertificateFile => None,

			&CouldNotOpenClientPrivateKeyFile(ref error) => Some(error),

			&CouldNotReadClientPkcs8PrivateKey => None,

			&CouldNotReadClientRsaPrivateKey => None,

			&ThereIsNeitherAPkcs8OrRsaClientPrivateKey => None,

			&CouldNotSetCertificateChainAndPrivateKey(ref error) => Some(error),
		}
	}
}

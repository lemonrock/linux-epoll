// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Represents an error when parsing a certificate chain and private key.
#[derive(Debug)]
pub enum CertificateChainAndPrivateKeyError
{
	CouldNotOpenCertificateFile(io::Error),

	CouldNotReadClientCertificateFile,

	CouldNotOpenPrivateKeysFile(io::Error),

	CouldNotReadPkcs8PrivateKey,

	CouldNotReadRsaPrivateKey,

	ThereIsNeitherAPkcs8OrRsaClientPrivateKey,
}

impl Display for CertificateChainAndPrivateKeyError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for CertificateChainAndPrivateKeyError
{
	#[inline(always)]
	fn source(&self) -> Option<&(error::Error + 'static)>
	{
		use self::CertificateChainAndPrivateKeyError::*;

		match self
		{
			&CouldNotOpenCertificateFile(ref error) => Some(error),

			&CouldNotReadClientCertificateFile => None,

			&CouldNotOpenPrivateKeysFile(ref error) => Some(error),

			&CouldNotReadPkcs8PrivateKey => None,

			&CouldNotReadRsaPrivateKey => None,

			&ThereIsNeitherAPkcs8OrRsaClientPrivateKey => None,
		}
	}
}

impl CertificateChainAndPrivateKeyError
{
}

// This file is part of rustls-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT. No part of rustls-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of rustls-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT.


/// Represents an error when parsing a certificate chain and private key.
#[derive(Debug)]
pub enum CertificateChainAndPrivateKeyError
{
	/// Could not open the certificate file.
	CouldNotOpenCertificateFile(io::Error),

	/// Could not read the certificate file.
	CouldNotReadClientCertificateFile,

	/// Could not open the file containing one or more private keys.
	CouldNotOpenPrivateKeysFile(io::Error),

	/// Could not read a PKCS8 format private key.
	CouldNotReadPkcs8PrivateKey,

	/// Could not read a RSA format private key.
	CouldNotReadRsaPrivateKey,

	/// The private keys file does not contain either a PKCS8 or a RSA private key.
	ThereIsNeitherAPkcs8OrRsaPrivateKey,
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

			&ThereIsNeitherAPkcs8OrRsaPrivateKey => None,
		}
	}
}

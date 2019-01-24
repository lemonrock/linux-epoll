// This file is part of rustls-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT. No part of rustls-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of rustls-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT.


/// A certificate and private key to authenticate with a TLS server.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CertificateChainAndPrivateKey
{
	/// PEM-encoded file containing the certificate chain, from most derived to least.
	pub certificate_chain_file: PathBuf,

	/// PEM-encoded file containing the private key for the certificate chain, ether RSA or PKCS8.
	///
	/// Only the first key found of each type is used; if both are found, then PKCS8 is preferred.
	///
	/// The private key must be capable of signing the first certificate in the `certificate_chain_file`.
	pub private_keys_file: PathBuf
}

impl CertificateChainAndPrivateKey
{
	/// Loads a certificate chain and private key.
	#[inline(always)]
	pub fn load_certificate_chain_and_private_key(&self) -> Result<(Vec<Certificate>, PrivateKey), CertificateChainAndPrivateKeyError>
	{
		Ok
		(
			(
				self.load_certificate_chain()?,
				self.load_private_key()?,
			)
		)
	}

	#[inline(always)]
	fn load_certificate_chain(&self) -> Result<Vec<Certificate>, CertificateChainAndPrivateKeyError>
	{
		use self::CertificateChainAndPrivateKeyError::*;

		let file = File::open(&self.certificate_chain_file).map_err(|error| CouldNotOpenCertificateFile(error))?;
		let mut reader = BufReader::new(file);
		certs(&mut reader).map_err(|_| CouldNotReadClientCertificateFile)
	}

	fn load_private_key(&self) -> Result<PrivateKey, CertificateChainAndPrivateKeyError>
	{
		use self::CertificateChainAndPrivateKeyError::*;

		let pkcs8_private_keys = pkcs8_private_keys(&mut self.open_private_keys_file()?).map_err(|_| CouldNotReadPkcs8PrivateKey)?;

		let rsa_private_keys = rsa_private_keys(&mut self.open_private_keys_file()?).map_err(|_| CouldNotReadRsaPrivateKey)?;

		if pkcs8_private_keys.is_empty()
		{
			if rsa_private_keys.is_empty()
			{
				Err(ThereIsNeitherAPkcs8OrRsaPrivateKey)
			}
			else
			{
				Ok((unsafe { rsa_private_keys.get_unchecked(0) }).clone())
			}
		}
		else
		{
			Ok((unsafe { pkcs8_private_keys.get_unchecked(0) }).clone())
		}
	}

	#[inline(always)]
	fn open_private_keys_file(&self) -> Result<BufReader<File>, CertificateChainAndPrivateKeyError>
	{
		let file = File::open(&self.private_keys_file).map_err(|error| CertificateChainAndPrivateKeyError::CouldNotOpenPrivateKeysFile(error))?;
		Ok(BufReader::new(file))
	}
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A certificate and private key to authenticate with a TLS server.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TlsClientCredentials
{
	/// PEM-encoded file containing the client's certificate chain, from most derived to least.
	pub client_certificate_chain_file: PathBuf,

	/// PEM-encoded file containing the client's private keys, ether RSA or PKCS8.
	///
	/// Only the first key found of each type is used; if both are found, then PKCS8 is preferred.
	///
	/// Private key must be capable of signing the first certificate in the client's `client_certificate_chain_file`.
	pub client_private_key_file: PathBuf
}

impl TlsClientCredentials
{
	#[inline(always)]
	fn load_certificate_chain(&self) -> Result<Vec<Certificate>, TlsClientConfigurationError>
	{
		use self::TlsClientConfigurationError::*;

		let file = File::open(&self.client_certificate_chain_file).map_err(|error| CouldNotOpenServerCertificateFile(error))?;
		let mut reader = BufReader::new(file);
		certs(&mut reader).map_err(|_| CouldNotReadServerCertificateFile)
	}

	fn load_private_key(&self) -> Result<PrivateKey, TlsClientConfigurationError>
	{
		use self::TlsClientConfigurationError::*;

		let pkcs8_private_keys = pkcs8_private_keys(&mut self.open_private_key_file()).map_err(|_| CouldNotReadServerPkcs8PrivateKey);
		let rsa_private_keys = rsa_private_keys(&mut self.open_private_key_file()).map_err(|_| CouldNotReadServerRsaPrivateKey);

		if pkcs8_private_keys.is_empty()
		{
			if rsa_private_keys.is_empty()
			{
				Err(ThereIsNeitherAPkcs8OrRsaServerPrivateKey)
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
}

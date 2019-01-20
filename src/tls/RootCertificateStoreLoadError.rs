// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Represents an error when configuring a server.
#[derive(Debug)]
pub enum RootCertificateStoreLoadError
{
	/// Could not open a certificate authorities (CA) PEM file.
	CouldNotOpenCertificateAuthoritiesPemFile(io::Error),

	/// After reading all certificate authority (CA) PEM files not a single valid certificate authority certificate (sic) was found.
	NoValidCertificateAuthoritiesInCertificateAuthoritiesPemFiles,
}

impl Display for RootCertificateStoreLoadError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for RootCertificateStoreLoadError
{
	#[inline(always)]
	fn source(&self) -> Option<&(error::Error + 'static)>
	{
		use self::TlsClientConfigurationError::*;

		match self
		{
			&CouldNotOpenCertificateAuthoritiesPemFile(ref error) => Some(error),

			&CouldNotReadCertificateAuthoritiesPemFile => None,
		}
	}
}

impl RootCertificateStoreLoadError
{
	/// Opens and parses a root certificate store.
	#[inline(always)]
	pub	fn root_certificate_store(certificate_authority_root_certificates_files: &PathBuf) -> Result<RootCertStore, Self>
	{
		use self::Self::*;

		let mut root_certificate_store = RootCertStore::empty();

		let mut total_valid_count = 0;
		for certificate_authority_root_certificates_file in certificate_authority_root_certificates_files.iter()
		{
			let file = File::open(certificate_authority_root_certificates_file).map_err(|error| CouldNotOpenCertificateAuthoritiesPemFile(error))?;
			let mut buf_reader = BufReader::new(file);
			let (valid_count, _invalid_count) = root_certificate_store.add_pem_file(&mut buf_reader).map_err(|_| CouldNotReadCertificateAuthoritiesPemFile)?;
			total_valid_count += valid_count;
		}

		if unlikely!(total_valid_count == 0)
		{
			Err(NoValidCertificateAuthoritiesInCertificateAuthoritiesPemFiles)
		}
		else
		{
			Ok(root_certificate_store)
		}
	}
}

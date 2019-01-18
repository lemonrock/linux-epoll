// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Defaults to `AllowAnyAnonymous`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClientAuthenticationConfiguration
{
	/// Only clients presenting a client certificate which is authenticated against an issuing Certificate Authority in `certificate_authority_root_certificates_file` are permitted.
	///
	/// Client distinguished names are not checked.
	AllowAnyAuthenticated
	{
		/// PEM-encoded file of certificate authority certificates.
		certificate_authority_root_certificates_file: PathBuf,
	},

	/// Either annoymous clients (those not presenting a client certificate) clients presenting a client certificate which is authenticated against an issuing Certificate Authority in `certificate_authority_root_certificates_file` are permitted.
	///
	/// Client distinguished names are not checked.
	AllowAnyAnonymousOrAuthenticated
	{
		/// PEM-encoded file of certificate authority certificates.
		certificate_authority_root_certificates_file: PathBuf,
	},

	/// Only clients which do not present a client certificate are permitted.
	///
	/// This is the most common setting for web servers.
	///
	/// This is the default.
	AllowAnyAnonymous,
}

impl Default for ClientAuthenticationConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		ClientAuthenticationConfiguration::AllowAnyAnonymous
	}
}

impl ClientAuthenticationConfiguration
{
	#[inline(always)]
	pub(crate) fn client_certificate_verifier(&self) -> Result<Arc<ClientCertVerifier>, TlsServerConfigurationError>
	{
		use self::ClientAuthenticationConfiguration::*;

		match self
		{
			AllowAnyAuthenticated { ref certificate_authority_root_certificates_file } => Ok(AllowAnyAuthenticatedClient::new(Self::root_certificate_store(certificate_authority_root_certificates_file)?)),

			AllowAnyAnonymousOrAuthenticated { ref certificate_authority_root_certificates_file } => Ok(AllowAnyAnonymousOrAuthenticatedClient::new(Self::root_certificate_store(certificate_authority_root_certificates_file)?)),

			AllowAnyAnonymous => Ok(NoClientAuth::new()),
		}
	}

	#[inline(always)]
	fn root_certificate_store(certificate_authority_root_certificates_file: &PathBuf) -> Result<RootCertStore, TlsServerConfigurationError>
	{
		use self::TlsServerConfigurationError::*;

		let mut root_certificate_store = RootCertStore::empty();

		let file = File::open(certificate_authority_root_certificates_file).map_err(|error| CouldNotOpenCertificateAuthoritiesPemFile(error))?;
		let mut buf_reader = BufReader::new(file);
		let (valid_count, invalid_count) = root_certificate_store.add_pem_file(&mut buf_reader).map_err(|_| CouldNotReadCertificateAuthoritiesPemFile)?;

		if unlikely!(valid_count == 0)
		{
			Err(NoValidCertificateAuthoritiesInCertificateAuthoritiesPemFile)
		}
		else
		{
			Ok(root_certificate_store)
		}
	}
}

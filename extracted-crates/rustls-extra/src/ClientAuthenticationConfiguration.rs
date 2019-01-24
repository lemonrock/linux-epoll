// This file is part of rustls-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT. No part of rustls-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of rustls-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT.


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
		certificate_authority_root_certificates_files: Vec<PathBuf>,
	},

	/// Either annoymous clients (those not presenting a client certificate) clients presenting a client certificate which is authenticated against an issuing Certificate Authority in `certificate_authority_root_certificates_file` are permitted.
	///
	/// Client distinguished names are not checked.
	AllowAnyAnonymousOrAuthenticated
	{
		/// PEM-encoded file of certificate authority certificates.
		certificate_authority_root_certificates_files: Vec<PathBuf>,
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
	pub(crate) fn client_certificate_verifier(&self) -> Result<Arc<ClientCertVerifier>, RootCertificateStoreLoadError>
	{
		use self::ClientAuthenticationConfiguration::*;

		match self
		{
			AllowAnyAuthenticated { ref certificate_authority_root_certificates_files } => Ok(AllowAnyAuthenticatedClient::new(RootCertificateStoreLoadError::root_certificate_store(certificate_authority_root_certificates_files)?)),

			AllowAnyAnonymousOrAuthenticated { ref certificate_authority_root_certificates_files } => Ok(AllowAnyAnonymousOrAuthenticatedClient::new(RootCertificateStoreLoadError::root_certificate_store(certificate_authority_root_certificates_files)?)),

			AllowAnyAnonymous => Ok(NoClientAuth::new()),
		}
	}
}

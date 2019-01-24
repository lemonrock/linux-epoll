// This file is part of rustls-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT. No part of rustls-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of rustls-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT.


/// TLS configuration for a server.
///
/// TLS is implemented using the rustls TLS library.
///
/// Note that it is not possible to configure which cipher suites are used; rustls chooses a minimal, currently known to be secure set with a preference for CHA-CHA.
#[derive(Debug, PartialEq)]
pub struct TlsServerConfiguration
{
	/// Configuration common to clients and servers.
	pub common: TlsCommonConfiguration,

	/// Maximum number of TLS sessions to store in memory.
	///
	/// If zero no sessions are stored.
	pub tls_maximum_sessions_to_store_in_memory: usize,

	/// Certificate chain.
	pub certificate_chain_and_private_key: CertificateChainAndPrivateKey,

	/// Location of certificate authorities for client certificates if used.
	pub client_authentication_configuration: ClientAuthenticationConfiguration,

	/// Online Certificate Status Protocol (OCSP) file, if any.
	pub online_certificate_status_protocol_file: Option<PathBuf>,

	/// Signed Certificate Timestamp List (SCT) file, if any.
	pub signed_certificate_timestamp_list_file: Option<PathBuf>,
}

impl TlsServerConfiguration
{
	/// Similar to a `default()` but permits specifying client authentication configuration and the location of the server's certificate chain and private keys.
	///
	/// `application_layer_protocol_negotiation_protocols` must not contain `ApplicationLayerProtocolNegotiationProtocol::HTTP_2_over_TCP` or a panic will occur when creating the server configuration.
	#[inline(always)]
	pub fn new(certificate_chain_and_private_key: CertificateChainAndPrivateKey, client_authentication_configuration: ClientAuthenticationConfiguration) -> Self
	{
		Self
		{
			common: TlsCommonConfiguration::default(),
			tls_maximum_sessions_to_store_in_memory: Self::default_tls_maximum_sessions_to_store_in_memory(),
			certificate_chain_and_private_key,
			client_authentication_configuration,
			online_certificate_status_protocol_file: Self::default_online_certificate_status_protocol_file(),
			signed_certificate_timestamp_list_file: Self::default_signed_certificate_timestamp_list_file(),
		}
	}

	/// Create server configuration.
	#[inline(always)]
	pub fn server_configuration(&self) -> Result<Arc<ServerConfig>, TlsServerConfigurationError>
	{
		let mut server_configuration = ServerConfig::new(self.client_authentication_configuration.client_certificate_verifier()?);

		server_configuration.set_protocols(&(self.common.application_layer_protocol_negotiation_protocols.to_rustls_form())[..]);

		server_configuration.ciphersuites = self.common.cipher_suites();

		server_configuration.mtu = self.common.tls_mtu;

		server_configuration.versions = self.common.supported_tls_versions.versions();

		{
			let (certificate_chain, private_key) = self.certificate_chain_and_private_key.load_certificate_chain_and_private_key()?;
			let online_certificate_status_protocol = self.load_online_certificate_status_protocol_file()?;
			let signed_certificate_timestamp_list = self.load_signed_certificate_timestamp_list_file()?;
			server_configuration.set_single_cert_with_ocsp_and_sct(certificate_chain, private_key, online_certificate_status_protocol, signed_certificate_timestamp_list)?;
		}

		server_configuration.session_storage = if self.tls_maximum_sessions_to_store_in_memory == 0
		{
			Arc::new(NoServerSessionStorage {})
		}
		else
		{
			server_configuration.ticketer = Ticketer::new();
			ServerSessionMemoryCache::new(self.tls_maximum_sessions_to_store_in_memory)
		};

		server_configuration.ignore_client_order = true;

		Ok(Arc::new(server_configuration))
	}

	/// Defaults to 256.
	#[inline(always)]
	pub const fn default_tls_maximum_sessions_to_store_in_memory() -> usize
	{
		256
	}

	/// Defaults to None.
	#[inline(always)]
	pub fn default_online_certificate_status_protocol_file() -> Option<PathBuf>
	{
		None
	}

	/// Defaults to None.
	#[inline(always)]
	pub fn default_signed_certificate_timestamp_list_file() -> Option<PathBuf>
	{
		None
	}

	#[inline(always)]
	fn load_online_certificate_status_protocol_file(&self) -> Result<Vec<u8>, TlsServerConfigurationError>
	{
		Self::load_optional_file(&self.online_certificate_status_protocol_file, TlsServerConfigurationError::CouldNotOpenOnlineCertificateStatusProtocolFile, TlsServerConfigurationError::CouldNotReadOnlineCertificateStatusProtocolFile)
	}

	#[inline(always)]
	fn load_signed_certificate_timestamp_list_file(&self) -> Result<Vec<u8>, TlsServerConfigurationError>
	{
		Self::load_optional_file(&self.signed_certificate_timestamp_list_file, TlsServerConfigurationError::CouldNotOpenSignedCertificateTimestampStatusFile, TlsServerConfigurationError::CouldNotReadSignedCertificateTimestampStatusFile)
	}

	#[inline(always)]
	fn load_optional_file(file_path: &Option<PathBuf>, open_error: impl FnOnce(io::Error) -> TlsServerConfigurationError, read_error: impl FnOnce(io::Error) -> TlsServerConfigurationError) -> Result<Vec<u8>, TlsServerConfigurationError>
	{
		let mut data = Vec::new();

		if let Some(file_path) = file_path.as_ref()
		{
			let mut file = File::open(file_path).map_err(open_error)?;
			file.read_to_end(&mut data).map_err(read_error)?;
		}

		Ok(data)
	}
}

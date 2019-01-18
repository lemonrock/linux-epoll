// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// TLS Configuration for a server.
///
/// TLS is implemented using the rustls TLS library.
///
/// Note that it is not possible to configure which cipher suites are used; rustls chooses a minimal, currently known to be secure set with a preference for CHA-CHA.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TlsConfiguration
{
	/// Location of certificate authorities for client certificates if used.
	pub client_authentication_configuration: ClientAuthenticationConfiguration,

	/// A static slice of supported signature algorithms.
	pub supported_client_signature_algorithms: SignatureAlgorithms,

	/// Which TLS versions to support?
	pub supported_tls_versions: SupportedTlsVersions,

	/// TLS message size Maximum Transmission Unit (MTU) in bytes.
	pub tls_mtu: Option<usize>,

	/// Maximum number of TLS sessions to store in memory.
	///
	/// If zero no sessions are stored.
	pub tls_maximum_sessions_to_store_in_memory: usize,

	/// Whether to support TLS session tickets
	///
	/// If false then tickets are not issued.
	pub support_tls_session_tickets: bool,

	/// PEM-encoded file containing the server's certificate chain, from most derived to least.
	pub server_certificate_chain_file: PathBuf,

	/// PEM-encoded file containing the server's private keys, ether RSA or PKCS8.
	///
	/// Only the first key found of each type is used; if both are found, then PKCS8 is preferred.
	///
	/// Private key must be capable of signing the first certificate in the server's `server_certificate_chain_file`.
	pub server_private_key_file: PathBuf,

	/// Online Certificate Status Protocol (OCSP) file, if any.
	pub online_certificate_status_protocol_file: Option<PathBuf>,

	/// Signed Certificate Timestamp List (SCT) file, if any.
	pub signed_certificate_timestamp_list_file: Option<PathBuf>,

	/// ALPN protocols, such as `http/1.1` and `http/1.0`, in preference order.
	pub application_layer_protocol_negotiation_protocols: IndexSet<ApplicationLayerProtocolNegotiationProtocol>,

	/// Session buffer limit (in bytes).
	///
	/// A value of 0 implies no limit and infinite potential growth.
	pub session_buffer_limit: usize,
}

impl TlsConfiguration
{
	/// Similar to a `default()` but permits specifying client authentication configuration and the location of the server's certificate chain and private keys.
	///
	/// `application_layer_protocol_negotiation_protocols` must not contain `ApplicationLayerProtocolNegotiationProtocol::HTTP_2_over_TCP` or a panic will occur when creating the server configuration.
	#[inline(always)]
	pub fn new(client_authentication_configuration: ClientAuthenticationConfiguration, server_certificate_chain_file: PathBuf, server_private_key_file: PathBuf, application_layer_protocol_negotiation_protocols: IndexSet<ApplicationLayerProtocolNegotiationProtocol>) -> Self
	{
		Self
		{
			client_authentication_configuration,
			supported_client_signature_algorithms: Self::default_supported_client_signature_algorithms(),
			supported_tls_versions: Self::default_supported_tls_versions(),
			tls_mtu: Self::default_tls_mtu(),
			tls_maximum_sessions_to_store_in_memory: Self::default_tls_maximum_sessions_to_store_in_memory(),
			support_tls_session_tickets: Self::default_support_tls_session_tickets(),
			server_certificate_chain_file,
			server_private_key_file,
			online_certificate_status_protocol_file: Self::default_online_certificate_status_protocol_file(),
			signed_certificate_timestamp_list_file: Self::default_signed_certificate_timestamp_list_file(),
			application_layer_protocol_negotiation_protocols,
			session_buffer_limit: Self::default_session_buffer_limit()
		}
	}

	#[inline(always)]
	pub(crate) fn server_configuration(&self) -> Result<Arc<ServerConfig>, TlsServerConfigurationError>
	{
		let mut server_configuration = ServerConfig::new(self.client_authentication_configuration.client_certificate_verifier());

		{
			let certificate_chain = self.load_certificate_chain()?;
			let private_key = self.load_private_key()?;
			let online_certificate_status_protocol = self.load_online_certificate_status_protocol_file()?;
			let signed_certificate_timestamp_list = self.load_signed_certificate_timestamp_list_file()?;
			server_configuration.set_single_cert_with_ocsp_and_sct(certificate_chain, private_key, online_certificate_status_protocol, signed_certificate_timestamp_list).map_err(|error| TlsServerConfigurationError::CouldNotSetCertificateChainAndPrivateKey(error))?;
		}

		{
			let mut protocols = Vec::with_capacity(application_layer_protocol_negotiation_protocols.len());
			for application_layer_protocol_negotiation_protocol in application_layer_protocol_negotiation_protocols
			{
				assert_ne!(application_layer_protocol_negotiation_protocol, ApplicationLayerProtocolNegotiationProtocol::HTTP_2_over_TCP, "HTTP_2_over_TCP can not be used with TLS");
				protocols.push(application_layer_protocol_negotiation_protocol.to_string())
			}

			server_configuration.set_protocols(&protocols[..]);
		}

		server_configuration.ignore_client_order = true;

		server_configuration.versions = self.supported_tls_versions.versions();

		server_configuration.session_storage = if self.tls_maximum_sessions_to_store_in_memory == 0
		{
			Arc::new(NoServerSessionStorage)
		}
		else
		{
			ServerSessionMemoryCache::new(self.tls_maximum_sessions_to_store_in_memory)
		};

		if self.support_tls_session_tickets
		{
			server_configuration.ticketer = Ticketer::new();
		}

		Ok(Arc::new(server_configuration))
	}

	/// Defaults to:-
	///
	/// *`ECDSA_P256_SHA256`
	/// *`ECDSA_P256_SHA384`
	/// *`ECDSA_P384_SHA256`
	/// *`ECDSA_P384_SHA384`
	/// *`RSA_PSS_2048_8192_SHA256_LEGACY_KEY`
	/// *`RSA_PSS_2048_8192_SHA384_LEGACY_KEY`
	/// *`RSA_PSS_2048_8192_SHA512_LEGACY_KEY`
	/// *`RSA_PKCS1_2048_8192_SHA256`
	/// *`RSA_PKCS1_2048_8192_SHA384`
	/// *`RSA_PKCS1_2048_8192_SHA512`
	/// *`RSA_PKCS1_3072_8192_SHA384`
	#[inline(always)]
	pub const fn default_supported_client_signature_algorithms() -> SignatureAlgorithms
	{
		&[
			&ECDSA_P256_SHA256,
			&ECDSA_P256_SHA384,
			&ECDSA_P384_SHA256,
			&ECDSA_P384_SHA384,
			&RSA_PSS_2048_8192_SHA256_LEGACY_KEY,
			&RSA_PSS_2048_8192_SHA384_LEGACY_KEY,
			&RSA_PSS_2048_8192_SHA512_LEGACY_KEY,
			&RSA_PKCS1_2048_8192_SHA256,
			&RSA_PKCS1_2048_8192_SHA384,
			&RSA_PKCS1_2048_8192_SHA512,
			&RSA_PKCS1_3072_8192_SHA384
		]
	}
	
	/// Defaults to TLS 1.3 and TLS 1.2.
	#[inline(always)]
	pub const fn default_supported_tls_versions() -> SupportedTlsVersions
	{
		SupportedTlsVersions::default()
	}

	/// Defaults to `None`.
	#[inline(always)]
	pub const fn default_tls_mtu() -> Option<usize>
	{
		None
	}

	/// Defaults to 256.
	#[inline(always)]
	pub const fn default_tls_maximum_sessions_to_store_in_memoryu() -> usize
	{
		256
	}

	/// Defaults to true.
	#[inline(always)]
	pub const fn default_support_tls_session_tickets() -> bool
	{
		true
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

	/// Defaults to 16Kb.
	#[inline(always)]
	pub fn default_session_buffer_limit() -> usize
	{
		16 * 1024
	}

	#[inline(always)]
	fn load_certificate_chain(&self) -> Result<Vec<Certificate>, TlsServerConfigurationError>
	{
		use self::TlsServerConfigurationError::*;

		let file = File::open(&self.server_certificate_chain_file).map_err(|error| CouldNotOpenServerCertificateFile(error))?;
		let mut reader = BufReader::new(file);
		certs(&mut reader).map_err(|_| CouldNotReadServerCertificateFile)
	}

	fn load_private_key(&self) -> Result<PrivateKey, TlsServerConfigurationError>
	{
		use self::TlsServerConfigurationError::*;

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

	#[inline(always)]
	fn open_private_key_file(&self) -> Result<BufReader, TlsServerConfigurationError>
	{
		let file = File::open(&self.server_private_key_file).map_err(|error| TlsServerConfigurationError::CouldNotOpenServerPrivateKeysFile(error))?;
		Ok(BufReader::new(file))
	}

	#[inline(always)]
	fn load_online_certificate_status_protocol_file(&self) -> Result<Vec<u8>, ServerConfigurationError>
	{
		Self::load_optional_file(&self.online_certificate_status_protocol_file, ServerConfigurationError::CouldNotOpenOnlineCertificateStatusProtocolFile, TlsServerConfigurationError::CouldNotReadOnlineCertificateStatusProtocolFile)
	}

	#[inline(always)]
	fn load_signed_certificate_timestamp_list_file(&self) -> Result<Vec<u8>, ServerConfigurationError>
	{
		Self::load_optional_file(&self.signed_certificate_timestamp_list_file, ServerConfigurationError::CouldNotOpenSignedCertificateTimestampStatusFile, TlsServerConfigurationError::CouldNotReadSignedCertificateTimestampStatusFile)
	}

	#[inline(always)]
	fn load_optional_file(file_path: &Option<PathBuf>, open_error: impl FnOnce(io::Error) -> ServerConfigurationError, read_error: impl FnOnce(io::Error) -> TlsServerConfigurationError) -> Result<Vec<u8>, TlsServerConfigurationError>
	{
		let mut data = Vec::new();

		if let Some(file_path) = file_path.as_ref()
		{
			let file = File::open(file_path).map_err(open_error)?;
			file.read_to_end(&mut data).map_err(read_error)?;
		}

		data
	}
}

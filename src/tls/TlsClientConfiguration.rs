// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// TLS configuration for a client.
///
/// TLS is implemented using the rustls TLS library.
///
/// Note that it is not possible to configure which cipher suites are used; rustls chooses a minimal, currently known to be secure set with a preference for CHA-CHA.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TlsClientConfiguration
{
	/// A file or files that contains desired server Trust Anchor roots.
	pub root_store: Vec<PathBuf>,

	/// A static slice of supported signature algorithms.
	pub supported_signature_algorithms: SignatureAlgorithms,

	/// Which TLS versions to support?
	pub supported_tls_versions: SupportedTlsVersions,

	/// TLS message size Maximum Transmission Unit (MTU) in bytes.
	pub tls_mtu: Option<usize>,

	/// Maximum number of TLS sessions to store in memory.
	///
	/// If zero no sessions are stored.
	pub tls_maximum_sessions_to_store_in_memory: usize,

	/// A certificate and private key to authenticate with a TLS server.
	pub client_credentials: OptioN<TlsClientCredentials>,

	/// ALPN protocols, such as `http/1.1` and `http/1.0`, in preference order.
	pub application_layer_protocol_negotiation_protocols: IndexSet<ApplicationLayerProtocolNegotiationProtocol>,

	/// Server name indication (SNI).
	///
	/// *Disabled* by default as it leaks information to third parties.
	pub enable_server_name_indication: bool,

	/// Will use Google's known list of certificate transparency (CT) logs.
	///
	/// List is updated via the `ct_logs` crate and so is fixed at compile time.
	pub enable_certificate_transparency_logs: bool,

	/// Session buffer limit (in bytes).
	///
	/// A value of 0 implies no limit and infinite potential growth.
	pub session_buffer_limit: usize,
}

impl TlsClientConfiguration
{
	/// Similar to a `default()` but permits specifying client authentication configuration and the location of the server's certificate chain and private keys.
	///
	/// `application_layer_protocol_negotiation_protocols` must not contain `ApplicationLayerProtocolNegotiationProtocol::HTTP_2_over_TCP` or a panic will occur when creating the server configuration.
	#[inline(always)]
	pub fn new(root_store: Vec<PathBuf>, client_credentials: Option<TlsClientCredentials>, application_layer_protocol_negotiation_protocols: IndexSet<ApplicationLayerProtocolNegotiationProtocol>) -> Self
	{
		Self
		{
			root_store,
			supported_signature_algorithms: Self::default_supported_signature_algorithms(),
			supported_tls_versions: Self::default_supported_tls_versions(),
			tls_mtu: Self::default_tls_mtu(),
			tls_maximum_sessions_to_store_in_memory: Self::default_tls_maximum_sessions_to_store_in_memory(),
			client_credentials,
			application_layer_protocol_negotiation_protocols,
			enable_server_name_indication: Self::default_enable_server_name_indication(),
			enable_certificate_transparency_logs: Self::default_enable_certificate_transparency_logs(),
			session_buffer_limit: Self::default_session_buffer_limit()
		}
	}
	
	// ct logs come from the `ct-logs` crate.
	#[inline(always)]
	pub(crate) fn client_configuration(&self) -> Result<Arc<ClientConfig>, TlsClientConfigurationError>
	{
		let mut client_configuration = ClientConfig::new();

		{
			let mut protocols = Vec::with_capacity(application_layer_protocol_negotiation_protocols.len());
			for application_layer_protocol_negotiation_protocol in application_layer_protocol_negotiation_protocols
			{
				assert_ne!(application_layer_protocol_negotiation_protocol, ApplicationLayerProtocolNegotiationProtocol::HTTP_2_over_TCP, "HTTP_2_over_TCP can not be used with TLS");
				protocols.push(application_layer_protocol_negotiation_protocol.to_string())
			}

			client_configuration.set_protocols(&protocols[..]);
		}

		client_configuration.ciphersuites = self.supported_signature_algorithms;

		client_configuration.set_mtu(self.tls_mtu);

		client_configuration.versions = self.supported_tls_versions.versions();

		if let Some(ref client_credentials) = self.client_credentials
		{
			let certificate_chain = client_credentials.load_certificate_chain()?;
			let private_key = client_credentials.load_private_key()?;
			client_configuration.set_single_client_cert(certificate_chain, private_key);
		}

		if self.tls_maximum_sessions_to_store_in_memory == 0
		{
			client_configuration.enable_tickets = false;
			client_configuration.set_persistence(Arc::new(NoClientSessionStorage));
		}
		else
		{
			client_configuration.enable_tickets = true;
			client_configuration.set_persistence(Arc::new(ClientSessionMemoryCache::new(self.tls_maximum_sessions_to_store_in_memory)));
		}

		client_configuration.root_store = Self::root_certificate_store(&self.root_store)?;

		client_configuration.ct_logs = if self.enable_certificate_transparency_logs
		{
			Some(GooglesKnownListOfCertificateTransparencyLogs)
		}
		else
		{
			None
		};

		client_configuration.enable_sni = self.enable_server_name_indication;

		client_configuration.enable_early_data = false;

		Ok(Arc::new(client_configuration))
	}

	/// Defaults to the value used for a `TlsServerConfiguration`.
	#[inline(always)]
	pub const fn default_supported_signature_algorithms() -> SignatureAlgorithms
	{
		TlsServerConfiguration::default_supported_signature_algorithms()
	}

	/// Defaults to the value used for a `TlsServerConfiguration`.
	#[inline(always)]
	pub const fn default_supported_tls_versions() -> SupportedTlsVersions
	{
		TlsServerConfiguration::default_supported_tls_versions()
	}

	/// Defaults to the value used for a `TlsServerConfiguration`.
	#[inline(always)]
	pub const fn default_tls_mtu() -> Option<usize>
	{
		TlsServerConfiguration::default_tls_mtu()
	}

	/// Defaults to 32.
	#[inline(always)]
	pub(crate) const fn default_tls_maximum_sessions_to_store_in_memory() -> usize
	{
		32
	}

	/// Defaults to `false`, as it is not encrypted.
	#[inline(always)]
	pub(crate) fn default_server_name_indication() -> bool
	{
		false
	}

	/// Defaults to `true`.
	///
	/// Uses Google's known list of certificate transparency logs from the `ct-logs` crate.
	#[inline(always)]
	pub(crate) fn default_enable_certificate_transparency_logs() -> bool
	{
		true
	}

	/// Defaults to the value used for a `TlsServerConfiguration`.
	#[inline(always)]
	pub(crate) fn default_session_buffer_limit() -> usize
	{
		TlsServerConfiguration::default_session_buffer_limit()
	}

	#[inline(always)]
	fn open_private_key_file(&self) -> Result<BufReader, TlsClientConfigurationError>
	{
		let file = File::open(&self.client_private_key_file).map_err(|error| TlsClientConfigurationError::CouldNotOpenServerPrivateKeysFile(error))?;
		Ok(BufReader::new(file))
	}

	#[inline(always)]
	fn root_certificate_store(certificate_authority_root_certificates_files: &Vec<PathBuf>) -> Result<RootCertStore, TlsClientConfigurationError>
	{
		use self::TlsClientConfigurationError::*;

		let mut root_certificate_store = RootCertStore::empty();

		let mut total_valid_count = 0;
		for certificate_authority_root_certificates_file in certificate_authority_root_certificates_files.iter()
		{
			let file = File::open(certificate_authority_root_certificates_file).map_err(|error| CouldNotOpenCertificateAuthoritiesPemFile(error))?;
			let mut buf_reader = BufReader::new(file);
			let (valid_count, _invalid_count) = root_certificate_store.add_pem_file(&mut buf_reader).map_err(|_| CouldNotReadCertificateAuthoritiesPemFile)?;
			total_valid_count += valid_count;
		}

		if unlikely!(valid_count == 0)
		{
			Err(NoValidCertificateAuthoritiesInCertificateAuthoritiesPemFiles)
		}
		else
		{
			Ok(root_certificate_store)
		}
	}
}

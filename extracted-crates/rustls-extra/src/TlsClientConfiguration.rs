// This file is part of rustls-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT. No part of rustls-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of rustls-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT.


/// TLS configuration for a client.
///
/// TLS is implemented using the rustls TLS library.
///
/// Note that it is not possible to configure which cipher suites are used; rustls chooses a minimal, currently known to be secure set with a preference for CHA-CHA.
#[derive(Debug, PartialEq)]
pub struct TlsClientConfiguration
{
	/// Configuration common to clients and servers.
	pub common: TlsCommonConfiguration,

	/// Maximum number of TLS sessions to store in memory.
	///
	/// If zero no sessions are stored.
	pub tls_maximum_sessions_to_store_in_memory: usize,

	/// A file or files that contains desired server Trust Anchor roots.
	pub root_store: Vec<PathBuf>,

	/// A certificate and private key to authenticate with a TLS server.
	pub certificate_chain_and_private_key: Option<CertificateChainAndPrivateKey>,

	/// Server name indication (SNI).
	///
	/// *Disabled* by default as it leaks information to third parties.
	pub enable_server_name_indication: bool,

	/// Will use Google's known list of certificate transparency (CT) logs.
	///
	/// List is updated via the `ct_logs` crate and so is fixed at compile time.
	pub enable_certificate_transparency_logs: bool,
}

impl TlsClientConfiguration
{
	/// Similar to a `default()` but permits specifying client authentication configuration and the location of the server's certificate chain and private keys.
	///
	/// `application_layer_protocol_negotiation_protocols` must not contain `ApplicationLayerProtocolNegotiationProtocol::HTTP_2_over_TCP` or a panic will occur when creating the server configuration.
	#[inline(always)]
	pub fn new(root_store: Vec<PathBuf>, certificate_chain_and_private_key: Option<CertificateChainAndPrivateKey>) -> Self
	{
		Self
		{
			common: TlsCommonConfiguration::default(),
			tls_maximum_sessions_to_store_in_memory: Self::default_tls_maximum_sessions_to_store_in_memory(),
			root_store,
			certificate_chain_and_private_key,
			enable_server_name_indication: Self::default_enable_server_name_indication(),
			enable_certificate_transparency_logs: Self::default_enable_certificate_transparency_logs(),
		}
	}

	/// Create client configuration.
	#[inline(always)]
	pub fn client_configuration(&self) -> Result<Arc<ClientConfig>, TlsClientConfigurationError>
	{
		let mut client_configuration = ClientConfig::new();

		client_configuration.set_protocols(&(self.common.application_layer_protocol_negotiation_protocols.to_rustls_form())[..]);

		client_configuration.ciphersuites = self.common.cipher_suites();

		client_configuration.set_mtu(&self.common.tls_mtu);

		client_configuration.versions = self.common.supported_tls_versions.versions();

		client_configuration.root_store = RootCertificateStoreLoadError::root_certificate_store(&self.root_store)?;

		if let Some(ref certificate_chain_and_private_key) = self.certificate_chain_and_private_key
		{
			let (certificate_chain, private_key) = certificate_chain_and_private_key.load_certificate_chain_and_private_key()?;
			client_configuration.set_single_client_cert(certificate_chain, private_key);
		}

		if self.tls_maximum_sessions_to_store_in_memory == 0
		{
			client_configuration.enable_tickets = false;
			client_configuration.set_persistence(Arc::new(NoClientSessionStorage {}));
		}
		else
		{
			client_configuration.enable_tickets = true;
			client_configuration.set_persistence(ClientSessionMemoryCache::new(self.tls_maximum_sessions_to_store_in_memory));
		}

		client_configuration.ct_logs = if self.enable_certificate_transparency_logs
		{
			Some(&GooglesKnownListOfCertificateTransparencyLogs)
		}
		else
		{
			None
		};

		client_configuration.enable_sni = self.enable_server_name_indication;

		client_configuration.enable_early_data = false;

		Ok(Arc::new(client_configuration))
	}

	/// Defaults to 32.
	#[inline(always)]
	pub(crate) const fn default_tls_maximum_sessions_to_store_in_memory() -> usize
	{
		32
	}

	/// Defaults to `false`, as it is not encrypted.
	#[inline(always)]
	pub(crate) fn default_enable_server_name_indication() -> bool
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
}

// This file is part of rustls-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT. No part of rustls-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of rustls-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT.


/// TLS configuration common to clients and servers.
#[derive(Debug, PartialEq)]
pub struct TlsCommonConfiguration
{
	/// The preferred cipher suites.
	pub cipher_suites: Vec<&'static SupportedCipherSuite>,

	/// Which TLS versions to support?
	pub supported_tls_versions: SupportedTlsVersions,

	/// TLS message size Maximum Transmission Unit (MTU) in bytes.
	pub tls_mtu: Option<usize>,

	/// ALPN protocols, such as `http/1.1` and `http/1.0`, in preference order.
	pub application_layer_protocol_negotiation_protocols: ApplicationLayerProtocolNegotiationProtocols,

	/// Session buffer limit (in bytes).
	///
	/// A value of 0 implies no limit and infinite potential growth.
	pub session_buffer_limit: usize,
}

impl Default for TlsCommonConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			cipher_suites: Self::default_cipher_suites(),
			supported_tls_versions: Self::default_supported_tls_versions(),
			tls_mtu: Self::default_tls_mtu(),
			application_layer_protocol_negotiation_protocols: Self::default_application_layer_protocol_negotiation_protocols(),
			session_buffer_limit: Self::default_session_buffer_limit(),
		}
	}
}

impl TlsCommonConfiguration
{
	/// Defaults to:-
	///
	/// * `TLS13_CHACHA20_POLY1305_SHA256`
	/// * `TLS13_AES_256_GCM_SHA384`
	/// * `TLS13_AES_128_GCM_SHA256`
	/// * `TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256`
	/// * `TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256`
	/// * `TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384`
	/// * `TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256`
	/// * `TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384`
	/// * `TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256`
	#[inline(always)]
	pub fn default_cipher_suites() -> Vec<&'static SupportedCipherSuite>
	{
		vec!
		[
			TLS13_CHACHA20_POLY1305_SHA256,
			TLS13_AES_256_GCM_SHA384,
			TLS13_AES_128_GCM_SHA256,
			TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256,
			TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256,
			TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
			TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,
			TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,
			TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256
		]
	}

	/// A vector, in preference order, of cipher suites.
	#[inline(always)]
	pub fn cipher_suites(&self) -> Vec<&'static SupportedCipherSuite>
	{
		self.cipher_suites.clone()
	}

	/// Defaults to TLS 1.3 and TLS 1.2.
	#[inline(always)]
	pub fn default_supported_tls_versions() -> SupportedTlsVersions
	{
		SupportedTlsVersions::default()
	}

	/// Defaults to `None`.
	#[inline(always)]
	pub const fn default_tls_mtu() -> Option<usize>
	{
		None
	}

	/// Defaults to no protocols.
	#[inline(always)]
	pub fn default_application_layer_protocol_negotiation_protocols() -> ApplicationLayerProtocolNegotiationProtocols
	{
		ApplicationLayerProtocolNegotiationProtocols::default()
	}

	/// Defaults to 16Kb.
	#[inline(always)]
	pub fn default_session_buffer_limit() -> usize
	{
		16 * 1024
	}
}

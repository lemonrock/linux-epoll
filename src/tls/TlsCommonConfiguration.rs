// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// TLS configuration common to clients and servers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TlsCommonConfiguration
{
	/// A static slice of supported signature algorithms.
	pub supported_signature_algorithms: SignatureAlgorithms,

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
			supported_signature_algorithms: Self::default_supported_signature_algorithms(),
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
	pub const fn default_supported_signature_algorithms() -> SignatureAlgorithms
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

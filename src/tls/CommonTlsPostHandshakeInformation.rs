// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// TLS information available after successful handshaking that is common to both clients and servers.
#[derive(Debug)]
pub struct CommonTlsPostHandshakeInformation<'a>
{
	/// Negotiated protocol version.
	pub negotiated_protocol_version: ProtocolVersion,

	/// Negotiated cipher suite.
	pub negotiated_cipher_suite: &'static SupportedCipherSuite,

	/// Agreed application layer protocol negotation (ALPN) protocol.
	///
	/// None if no protocol was agreed or no protocol was supplied.
	pub agreed_application_layer_protocol_negotiation_protocol: Option<&'a str>,

	/// Peer certificates.
	pub peer_certificates: Vec<Certificate>,
}

impl<'a> CommonTlsPostHandshakeInformation<'a>
{
	#[inline(always)]
	pub(crate) fn from_tls_session(tls_session: &'a impl Session) -> Self
	{
		Self
		{
			negotiated_protocol_version: tls_session.get_protocol_version().unwrap(),
			negotiated_cipher_suite: tls_session.get_negotiated_ciphersuite().unwrap(),
			agreed_application_layer_protocol_negotiation_protocol: tls_session.get_alpn_protocol(),
			peer_certificates: tls_session.get_peer_certificates().unwrap(),
		}
	}

	/// RFC defines 5929 three channel bindings: `tls-unique`, `tls-server-end-point` and `tls-unique-for-telnet`.
	///
	/// These are officially not defined for TLS 1.3 (RFC 8446).
	///
	/// It is actually impossible to calculate `tls-unique` for TLS 1.3.
	/// `tls-unique-for-telnet` should be considered dead.
	///
	/// Thus only `tls-server-end-point` is potentially usable.
	/// Additionally, it is supported by common database products, including Postgres and MongoDb, with the `SCRAM-SHA-256-PLUS` SASL authentication mechanism.
	#[inline(always)]
	pub fn calculate_tls_server_end_point_channel_binding_certificate_hash(&self) -> Vec<u8>
	{
		// See postgres: be_tls_get_certificate_hash and <https://github.com/postgres/postgres/blob/97c39498e5ca9208d3de5a443a2282923619bf91/src/backend/libpq/auth-scram.c#L1195>.
		unimplemented!("Will be implemented when a dependent protocol first needs SCRAM-SHA-256-PLUS SASL authentication")
	}
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// List from <https://www.iana.org/assignments/tls-extensiontype-values/tls-extensiontype-values.xhtml#alpn-protocol-ids> as of January 15th, 2019.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ApplicationLayerProtocolNegotiationProtocol
{
	/// HTTP/0.9 defined in RFC 1945.
	HTTP_0_9,

	/// HTTP/1.0 defined in RFC 1945.
	HTTP_1_0,

	/// HTTP/1.1 defined in RFC 7230.
	HTTP_1_1,

	/// SPDY/1 defined in <http://dev.chromium.org/spdy/spdy-protocol/spdy-protocol-draft1>.
	SPDY_1,

	/// SPDY/2 defined in <http://dev.chromium.org/spdy/spdy-protocol/spdy-protocol-draft2>.
	SPDY_2,

	/// SPDY/3 defined in <http://dev.chromium.org/spdy/spdy-protocol/spdy-protocol-draft3>.
	SPDY_3,

	/// Traversal Using Relays around NAT (TURN) defined in RFC 7443.
	StunTurn,

	/// NAT discovery using Session Traversal Utilities for NAT (STUN) defined in RFC 7443.
	StunNatDiscovery,

	/// HTTP/2 over TLS defined in RFC 7540.
	HTTP_2_over_TLS,

	/// HTTP/2 over TCP defined in RFC 7540.
	///
	/// ***NOTE: This identifier is for use within a cleartext version of a protocol and is not allowed to appear in a TLS ALPN negotiation.***
	HTTP_2_over_TCP,

	/// WebRTC Media and Data defined in RFC-ietf-rtcweb-alpn-04.
	WebRTC,

	/// Confidential WebRTC Media and Data defined in RFC-ietf-rtcweb-alpn-04.
	ConfidentialWebRTC,

	/// FTP defined in RFC 959 and RFC 4217.
	FTP,

	/// IMAP defined in RFC 2595.
	IMAP,

	/// POP3 defined in RFC 2595.
	POP3,

	/// ManageSieve defined in RFC 5804.
	ManageSieve,

	/// CoAP defined in RFC C8323.
	CoAP,

	/// XMPP jabber:client namespace defined in <https://xmpp.org/extensions/xep-0368.html>.
	XMPP_Client,

	/// XMPP jabber:server namespace defined in <https://xmpp.org/extensions/xep-0368.html>.
	XMPP_Server,

	/// Unofficial
	Unofficial(&'static str),
}

impl ApplicationLayerProtocolNegotiationProtocol
{
	#[inline(always)]
	pub fn to_string(&self) -> String
	{
		use self::ApplicationLayerProtocolNegotiationProtocol::*;

		let value = match *self
		{
			HTTP_0_9 => "http/0.9",
			HTTP_1_0 => "http/1.0",
			HTTP_1_1 => "http/1.1",
			SPDY_1 => "spdy/1",
			SPDY_2 => "spdy/2",
			SPDY_3 => "spdy/3",
			StunTurn => "stun.turn",
			StunNatDiscovery => "stun.nat-discovery",
			HTTP_2_over_TLS => "h2",
			HTTP_2_over_TCP => "h2c",
			WebRTC => "webrtc",
			ConfidentialWebRTC => "c-webrtc",
			FTP => "ftp",
			IMAP => "imap",
			POP3 => "pop3",
			ManageSieve => "managesieve",
			CoAP => "coap",
			XMPP_Client => "xmpp-client",
			XMPP_Server => "xmpp-server",
			Unofficial(value) => value,
		};

		value.to_string()
	}
}

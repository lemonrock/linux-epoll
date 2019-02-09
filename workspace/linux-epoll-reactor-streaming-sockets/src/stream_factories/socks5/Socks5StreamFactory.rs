// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A factory to create instances of any stream where there is an initial handshake with a SOCKS5 server.
#[derive(Debug)]
pub struct Socks5StreamFactory<SF: StreamFactory<SD>, SD: SocketData>(pub SF);

impl<SF: StreamFactory<SD>, SD: SocketData> StreamFactory<SD> for Socks5StreamFactory<SF, SD>
{
	/// This is a lie; the lifetime is ***NOT*** `'static` but actually `'yielder` in `new_stream_and_handshake()`.
	/// Rust's lack of associated type constructors makes this impossible to express; `yielder` is ***NOT*** a parameter to put on `UnencryptedStreamFactory` (eg `UnencryptedStreamFactory<'yielder>`) because the lifetime has nothing to do with the lifetime of the factory (indeed, factories live far longer than the coroutines they are invovled in instantiating).
	type S = SF::S;

	/// Returns a bound socket address.
	type ProxyOrTunnelInformation = (BoundSocketAddress, SF::ProxyOrTunnelInformation);

	type AdditionalArguments = (Socks5Connect, Rc<Socks5AuthenticationCredentials>, SF::AdditionalArguments);

	#[inline(always)]
	fn new_stream_and_handshake<'yielder>(&self, streaming_socket_file_descriptor: StreamingSocketFileDescriptor<SD>, yielder: Yielder<'yielder, ReactEdgeTriggeredStatus, (), Result<(), CompleteError>>, additional_arguments: Self::AdditionalArguments) -> Result<(Self::S, Self::ProxyOrTunnelInformation), CompleteError>
	{
		let generic_stream = GenericStream::wrap(streaming_socket_file_descriptor, yielder);

		let unencrypted_stream = UnencryptedStream::new(generic_stream);

		let socks5_authentication_credentials = additional_arguments.1;

		let unencrypted_stream = socks5_authentication_credentials.send_request(unencrypted_stream)?;

		let (unencrypted_stream, credential) = Socks5CredentialReply::read_reply(unencrypted_stream)?;

		let unencrypted_stream = credential.negotiate(unencrypted_stream)?;

		let socks5_connect = additional_arguments.0;

		let unencrypted_stream = socks5_connect.send_request(unencrypted_stream)?;

		let (unencrypted_stream, bound_socket) = Socks5ConnectReply::read_reply(unencrypted_stream)?;


		// TODO: HTTP CONNECT proxies.

			// HTTP CONNECT
				// Done in plaintext
			// HTTP CONNECT with ?SSL to proxy?: https://dev.chromium.org/developers/design-documents/secure-web-proxy
		/*
		chrome --proxy-server=https://secure-proxy.example.com:443
Since the communication between Chrome and the proxy uses SSL, next protocol negotiation will be used.
If the servers supports SPDY, then the proxy will act as a  SPDY Proxy.
		*/

		/*
		HTTP CONNECT method

The most common form of HTTP tunneling is the standardized HTTP CONNECT method.[1][2] In this mechanism, the client asks an HTTP proxy server to forward the TCP connection to the desired destination. The server then proceeds to make the connection on behalf of the client. Once the connection has been established by the server, the proxy server continues to proxy the TCP stream to and from the client. Only the initial connection request is HTTP - after that, the server simply proxies the established TCP connection.

This mechanism is how a client behind an HTTP proxy can access websites using SSL or TLS (i.e. HTTPS). Proxy servers may also limit connections by only allowing connections to the default HTTPS port 443, whitelisting hosts, or blocking traffic which doesn't appear to be SSL.
Example negotiation

The client connects to the proxy server and requests tunneling by specifying the port and the host computer it would like to connect to. The port is used to indicate the protocol being requested.[3]

CONNECT example.host.com:22 HTTP/1.1
Proxy-Authorization: Basic encoded-credentials

If the connection was allowed and the proxy has connected to the specified host then the proxy will return a 2XX success response.[3]

HTTP/1.1 200 OK


		*/


		let (streaming_socket_file_descriptor, yielder) = unencrypted_stream.unwrap();

		let (stream, proxy_or_tunnel_information) = self.0.new_stream_and_handshake(streaming_socket_file_descriptor, yielder, additional_arguments.2);
	}
}

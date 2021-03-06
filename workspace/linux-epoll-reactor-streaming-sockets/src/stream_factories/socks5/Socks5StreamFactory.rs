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

		let (streaming_socket_file_descriptor, yielder) = unencrypted_stream.unwrap();

		let (stream, proxy_or_tunnel_information) = self.0.new_stream_and_handshake(streaming_socket_file_descriptor, yielder, additional_arguments.2);
	}
}

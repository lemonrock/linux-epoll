// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A TLS server stream.
pub struct TlsServerStream<'a, SD: SocketData>
{
	generic_stream: GenericStream<'a, SD>,
	tls_session: ServerSession,
}

impl<'a, SD: SocketData> Stream<'a> for TlsServerStream<'a, SD>
{
	type PostHandshakeInformation = (CommonTlsPostHandshakeInformation<'a>, ServerNameIndication<'a>);

	#[inline(always)]
	fn post_handshake_information(&'a self) -> Self::PostHandshakeInformation
	{
		(CommonTlsPostHandshakeInformation::from_tls_session(&self.tls_session), self.tls_session.get_sni_hostname())
	}

	#[inline(always)]
	fn read_data(&mut self, read_into_buffer: &mut [u8]) -> Result<usize, CompleteError>
	{
		self.generic_stream.tls_read(&self.tls_session, read_into_buffer)
	}

	#[inline(always)]
	fn write_data(&mut self, write_from_buffer: &[u8]) -> Result<usize, CompleteError>
	{
		self.generic_stream.tls_write(&self.tls_session, write_from_buffer)
	}

	#[inline(always)]
	fn finish(mut self) -> Result<(), CompleteError>
	{
		self.tls_session.stream_close()
	}
}

impl<'a, SD: SocketData> TlsServerStream<'a, SD>
{
	#[inline(always)]
	pub(crate) fn new(generic_stream: GenericStream<'a, SD>, tls_configuration: &Arc<ServerConfig>, session_buffer_limit: usize) -> Result<Self, CompleteError>
	{
		let mut tls_session = ServerSession::new(tls_configuration);
		tls_session.set_buffer_limit(session_buffer_limit);

		generic_stream.tls_handshake(&mut tls_session)?;

		Ok
		(
			Self
			{
				generic_stream,
				tls_session,
			}
		)
	}
}

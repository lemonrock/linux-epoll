// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A TLS client stream.
pub struct TlsClientStream<'a, SD: SocketData>
{
	generic_stream: GenericStream<'a>,
	tls_session: ClientSession,
}

impl<'a, SD: SocketData> Stream <'a>for TlsClientStream<'a, SD>
{
	type PostHandshakeInformation = CommonTlsPostHandshakeInformation<'a>;

	#[inline(always)]
	fn post_handshake_information(&'a self) -> Self::PostHandshakeInformation
	{
		CommonTlsPostHandshakeInformation::from_tls_session(&self.tls_session)
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

impl<'a, SD: SocketData> TlsClientStream<'a, SD>
{
	#[inline(always)]
	pub(crate) fn new(generic_stream: GenericStream<'a>, tls_configuration: &Arc<ClientConfig>, session_buffer_limit: usize, ascii_host_name: &str) -> Self
	{
		let hostname = DNSNameRef::try_from_ascii_str(ascii_host_name).expect("Invalid ASCII host name");
		let mut tls_session = ClientSession::new(tls_configuration, hostname);
		tls_session.set_buffer_limit(session_buffer_limit);

		Self
		{
			generic_stream,
			tls_session,
		}
	}
}

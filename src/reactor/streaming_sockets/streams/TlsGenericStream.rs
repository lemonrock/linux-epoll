// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug)]
struct TlsGenericStream<'a, SD: SocketData, S: SessionExt>
{
	generic_stream: GenericStream<'a, SD>,
	tls_session: S,
}

impl<'a, SD: SocketData> TlsGenericStream<'a, SD, ServerSession>
{
	#[inline(always)]
	fn server_name_indication_handshake_information(&'a self) -> Option<&'a str>
	{
		self.tls_session.get_sni_hostname()
	}
}

impl<'a, SD: SocketData, S: SessionExt> TlsGenericStream<'a, SD, S>
{
	#[inline(always)]
	fn configure_and_handshake(generic_stream: GenericStream<'a, SD>, tls_session: S, session_buffer_limit: usize) -> Result<Self, CompleteError>
	{
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

	#[inline(always)]
	fn common_tls_post_handshake_information(&'a self) -> CommonTlsPostHandshakeInformation
	{
		CommonTlsPostHandshakeInformation::from_tls_session(&self.tls_session)
	}

	#[inline(always)]
	fn read_data(&mut self, read_into_buffer: &mut [u8]) -> Result<usize, CompleteError>
	{
		self.generic_stream.tls_read(&mut self.tls_session, read_into_buffer)
	}

	#[inline(always)]
	fn write_data(&mut self, write_from_buffer: &[u8]) -> Result<usize, CompleteError>
	{
		self.generic_stream.tls_write(&mut self.tls_session, write_from_buffer)
	}

	#[inline(always)]
	fn flush_written_data(&mut self) -> Result<(), CompleteError>
	{
		self.generic_stream.tls_flush_written_data(&mut self.tls_session)
	}

	#[inline(always)]
	fn finish(mut self) -> Result<(), CompleteError>
	{
		self.generic_stream.tls_finish(&mut self.tls_session)
	}
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A TLS client stream.
#[derive(Debug)]
pub struct TlsClientStream<'yielder, SD: SocketData>
{
	tls_generic_stream: TlsGenericStream<'yielder, SD, ClientSession>,
}

stream_read_write!(TlsClientStream);

impl<'yielder, SD: SocketData> Stream for TlsClientStream<'yielder, SD>
{
	/// This is a lie (ie the lifetime is ***NOT*** `'static`); the actual lifetime is ***LESS THAN*** `'yielder` and is the same as the lifetime of the underlying TLS `ClientSession`, ie the lifetime of an instance of this struct.
	type PostHandshakeInformation = CommonTlsPostHandshakeInformation<'static>;

	#[inline(always)]
	fn post_handshake_information(&self) -> Self::PostHandshakeInformation
	{
		self.tls_generic_stream.common_tls_post_handshake_information()
	}

	#[inline(always)]
	fn read_data(&mut self, read_into_buffer: &mut [u8]) -> Result<usize, CompleteError>
	{
		self.tls_generic_stream.read_data(read_into_buffer)
	}

	#[inline(always)]
	fn write_data(&mut self, write_from_buffer: &[u8]) -> Result<usize, CompleteError>
	{
		self.tls_generic_stream.write_data(write_from_buffer)
	}

	#[inline(always)]
	fn flush_written_data(&mut self) -> Result<(), CompleteError>
	{
		self.tls_generic_stream.flush_written_data()
	}

	#[inline(always)]
	fn finish(self) -> Result<(), CompleteError>
	{
		self.tls_generic_stream.finish()
	}
}

impl<'yielder, SD: SocketData> TlsClientStream<'yielder, SD>
{
	#[inline(always)]
	pub(crate) fn new(generic_stream: GenericStream<'yielder, SD>, tls_configuration: &Arc<ClientConfig>, session_buffer_limit: usize, ascii_host_name: Rc<DNSName>) -> Result<Self, CompleteError>
	{
		let tls_session = ClientSession::new(tls_configuration, ascii_host_name.as_ref());

		Ok
		(
			Self
			{
				tls_generic_stream: TlsGenericStream::configure(generic_stream, tls_session, session_buffer_limit)?,
			}
		)
	}
}

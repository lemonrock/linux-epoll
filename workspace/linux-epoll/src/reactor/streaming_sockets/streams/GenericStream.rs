// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct GenericStream<'yielder, SD: SocketData>
{
	streaming_socket_file_descriptor: StreamingSocketFileDescriptor<SD>,
	input_output_yielder: InputOutputYielder<'yielder>,
	byte_counter: ByteCounter,
}

impl<'yielder, SD: SocketData> GenericStream<'yielder, SD>
{
	#[inline(always)]
	fn tls_handshake(&mut self, tls_session: &mut impl SessionExt) -> Result<(), CompleteError>
	{
		tls_session.complete_handshaking::<SD>(&self.streaming_socket_file_descriptor, &mut self.input_output_yielder, &mut self.byte_counter)
	}

	#[inline(always)]
	fn tls_read(&mut self, tls_session: &mut impl SessionExt, read_into_buffer: &mut [u8]) -> Result<usize, CompleteError>
	{
		tls_session.stream_read::<SD>(&self.streaming_socket_file_descriptor, &mut self.input_output_yielder, &mut self.byte_counter, read_into_buffer)
	}

	#[inline(always)]
	fn tls_write(&mut self, tls_session: &mut impl SessionExt, write_from_buffer: &[u8]) -> Result<usize, CompleteError>
	{
		tls_session.stream_write::<SD>(&self.streaming_socket_file_descriptor, &mut self.input_output_yielder, &mut self.byte_counter, write_from_buffer)
	}

	#[inline(always)]
	fn tls_flush_written_data(&mut self, tls_session: &mut impl SessionExt) -> Result<(), CompleteError>
	{
		tls_session.stream_flush::<SD>(&self.streaming_socket_file_descriptor, &mut self.input_output_yielder, &mut self.byte_counter)
	}

	#[inline(always)]
	fn tls_finish(&mut self, tls_session: &mut impl SessionExt) -> Result<(), CompleteError>
	{
		tls_session.stream_close::<SD>(&self.streaming_socket_file_descriptor, &mut self.input_output_yielder, &mut self.byte_counter)
	}
}

impl<'yielder, SD: SocketData> GenericStream<'yielder, SD>
{
	#[inline(always)]
	pub(crate) fn wrap(streaming_socket_file_descriptor: StreamingSocketFileDescriptor<SD>, yielder: Yielder<'yielder, ReactEdgeTriggeredStatus, (), Result<(), CompleteError>>) -> Self
	{
		Self::new(streaming_socket_file_descriptor, InputOutputYielder::new(yielder), ByteCounter::default())
	}

	#[inline(always)]
	fn new(streaming_socket_file_descriptor: StreamingSocketFileDescriptor<SD>, input_output_yielder: InputOutputYielder<'yielder>, byte_counter: ByteCounter) -> Self
	{
		Self
		{
			streaming_socket_file_descriptor,
			input_output_yielder,
			byte_counter,
		}
	}
}

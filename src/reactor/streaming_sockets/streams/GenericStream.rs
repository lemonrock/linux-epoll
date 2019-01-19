// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


pub(crate) struct GenericStream<'a, SD: SocketData>
{
	streaming_socket_file_descriptor: &'a StreamingSocketFileDescriptor<SD>,
	input_output_yielder: InputOutputYielder<'a>,
	byte_counter: ByteCounter,
}

impl<'a, SD: SocketData> GenericStream<'a, SD>
{
	#[inline(always)]
	pub(crate) fn tls_handshake(&mut self, tls_session: &mut impl Session) -> Result<(), CompleteError>
	{
		tls_session.complete_handshaking(self.streaming_socket_file_descriptor, &mut self.input_output_yielder, &mut self.byte_counter)
	}

	#[inline(always)]
	fn tls_read(&mut self, read_into_buffer: &mut [u8], tls_session: &mut impl Session) -> Result<usize, CompleteError>
	{
		self.tls_session.stream_read(self.streaming_socket_file_descriptor, &mut self.input_output_yielder, &mut self.byte_counter, read_into_buffer).map_err(CompleteError::Tls)
	}

	#[inline(always)]
	fn tls_write(&mut self, write_from_buffer: &mut [u8], tls_session: &mut impl Session) -> Result<usize, CompleteError>
	{
		self.tls_session.stream_write(self.streaming_socket_file_descriptor, &mut self.input_output_yielder, &mut self.byte_counter, write_from_buffer)
	}
}

impl<'a, SD: SocketData> GenericStream<'a, SD>
{
	#[inline(always)]
	pub(crate) fn new(streaming_socket_file_descriptor: &'a StreamingSocketFileDescriptor<SD>, input_output_yielder: InputOutputYielder<'a>, byte_counter: ByteCounter) -> Self
	{
		Self
		{
			streaming_socket_file_descriptor,
			input_output_yielder,
			byte_counter,
		}
	}
}

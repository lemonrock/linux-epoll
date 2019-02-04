// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// An unencrypted (raw) stream.
///
/// Only really useful in 2019 for implementing a HTTP => HTTPS delegate, and, perhaps, to support AMQP, LDAP, SMTP, IMAP and POP3 'upgrades' to TLS (eg `STARTTLS`).
/// However, although all of these 'upgrade' approaches should be considered potentially insecure as of 2019.
/// RFC 8314 recommends not using `STARTTLS` for email protocols (`SMTP`, `IMAP` and `POP3`).
///
/// Vectored reads and vectored writes are not supported (although they could be) because they do not compose well when also using TLS streams; since most modern protocols are TLS-based, the a vectored read or write operation is much less useful than it once was.
///
/// Likewise, sendfile is not directly supported, although if (ever) Rustls gets support the Linux's kernel implementation of a TLS-encrypted sendfile, we may add support for it.
#[derive(Debug)]
pub struct UnencryptedStream<'yielder, SD: SocketData>(GenericStream<'yielder, SD>);

stream_read_write!(UnencryptedStream);

impl<'yielder, SD: SocketData> Stream for UnencryptedStream<'yielder, SD>
{
	type PostHandshakeInformation = ();

	#[inline(always)]
	fn post_handshake_information(&self) -> Self::PostHandshakeInformation
	{
		()
	}

	#[inline(always)]
	fn read_data(&mut self, read_into_buffer: &mut [u8]) -> Result<usize, CompleteError>
	{
		loop
		{
			let bytes_read = match self.0.streaming_socket_file_descriptor.receive_from(read_into_buffer)
			{
				Err(io_error) => read_loop_or_await_or_error!(io_error, &mut self.0.input_output_yielder, SocketRead),

				Ok(bytes_read) => bytes_read,
			};

			self.0.byte_counter.bytes_read(bytes_read);
			return Ok(bytes_read)
		}
	}

	#[inline(always)]
	fn write_data(&mut self, write_from_buffer: &[u8]) -> Result<usize, CompleteError>
	{
		loop
		{
			let bytes_written = match self.0.streaming_socket_file_descriptor.send_to(write_from_buffer)
			{
				Err(io_error) => write_loop_or_await_or_error!(io_error, &mut self.0.input_output_yielder, SocketWrite),

				Ok(bytes_written) => bytes_written,
			};

			self.0.byte_counter.bytes_read(bytes_written);
			return Ok(bytes_written)
		}
	}

	#[inline(always)]
	fn flush_written_data(&mut self) -> Result<(), CompleteError>
	{
		Ok(())
	}

	#[inline(always)]
	fn finish(self) -> Result<(), CompleteError>
	{
		Ok(())
	}
}

impl<'yielder, SD: SocketData> UnencryptedStream<'yielder, SD>
{
	#[inline(always)]
	pub(crate) fn new(generic_stream: GenericStream<'yielder, SD>) -> Self
	{
		Self(generic_stream)
	}
}

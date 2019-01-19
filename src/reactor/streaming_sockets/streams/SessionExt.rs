// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


trait SessionExt: Session
{
	/// Logic required to complete handshaking before progressing with a connection.
	fn complete_handshaking<SD: SocketData>(&mut self, streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<SD>, yielder: &mut InputOutputYielder, byte_counter: &mut ByteCounter) -> Result<(), CompleteError>
	{
		while self.is_handshaking()
		{
			let is_end_of_file = self.complete_input_output(streaming_socket_file_descriptor, yielder, byte_counter, true)?;

			if unlikely!(is_end_of_file)
			{
				return Err(CompleteError::from(TlsInputOutputError::EndOfFileWhilstHandshaking))
			}
		}

		Ok(())
	}

	/// Logic required for an implementation of `io::Read.read()`.
	///
	/// Can legitimately return 0 bytes and ***NOT*** be end-of-file.
	fn stream_read<SD: SocketData>(&mut self, streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<SD>, yielder: &mut InputOutputYielder, byte_counter: &mut ByteCounter, buf: &mut [u8]) -> Result<usize, TlsInputOutputError>
	{
		self.complete_prior_input_output::<SD>(streaming_socket_file_descriptor, yielder, byte_counter)?;

		const IsEndOfFile: bool = true;

		// We call `process_input_output_after_handshaking()` in a loop since a single call may read only a partial packet from the underlying transport; a full packet is needed to get more plaintext.
		while self.wants_read() && self.process_input_output_after_handshaking::<SD>(streaming_socket_file_descriptor, yielder, byte_counter)? != IsEndOfFile
		{}

		match self.read(buf)
		{
			Err(error) => match error.kind()
			{
				ErrorKind::ConnectionAborted => Err(CompleteError::from(TlsInputOutputError::CloseNotifyAlertReceived)),
				_ => panic!("Unexpected error `{:?}` from Session.read()", error),
			}

			Ok(bytes_read) => Ok(bytes_read)
		}
	}

	/// Logic required for an implementation of `io::Write.write()`.
	fn stream_write<SD: SocketData>(&mut self, streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<SD>, yielder: &mut InputOutputYielder, byte_counter: &mut ByteCounter, buf: &[u8]) -> Result<usize, CompleteError>
	{
		self.complete_prior_input_output::<SD>(streaming_socket_file_descriptor, yielder, byte_counter)?;

		let len = self.write(buf).expect("Internal implementation in self (ServerSession) imp (ServerSessionImpl) common (SessionCommon) send_some_plaintext() does not return errors");

		// Try to write the underlying transport here, but don't let
		// any errors mask the fact we've consumed `len` bytes.
		// Callers will learn of permanent errors on the next call.
		let _ = self.process_input_output_after_handshaking::<SD>(streaming_socket_file_descriptor, yielder, byte_counter);

		Ok(len)
	}

	/// Logic required to close a TLS stream by sending a close notify fatal alert.
	#[inline(always)]
	fn stream_close(&mut self) -> Result<(), CompleteError>
	{
		self.complete_prior_input_output::<SD>(streaming_socket_file_descriptor, yielder, byte_counter)?;

		self.send_close_notify();

		self.process_input_output_after_handshaking::<SD>(streaming_socket_file_descriptor, yielder, byte_counter)
	}

	/// Logic required for an implementation of `io::Write.flush()`.
	fn stream_flush<SD: SocketData>(&mut self, streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<SD>, yielder: &mut InputOutputYielder, byte_counter: &mut ByteCounter) -> Result<(), CompleteError>
	{
		let is_end_of_file = self.complete_prior_input_output::<SD>(streaming_socket_file_descriptor, yielder, byte_counter)?;
		if is_end_of_file
		{
			return Ok(())
		}

		self.flush().expect("Internal implementation in self (ServerSession) flush_plaintext() does not return errors");

		if self.wants_write()
		{
			self.process_input_output_after_handshaking::<SD>(streaming_socket_file_descriptor, yielder, byte_counter)
		}
		else
		{
			Ok(())
		}
	}

	#[doc(hidden)]
	#[inline(always)]
	fn complete_prior_input_output<SD: SocketData>(&mut self, streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<SD>, yielder: &mut InputOutputYielder, byte_counter: &mut ByteCounter) -> Result<bool, CompleteError>
	{
		if self.wants_write()
		{
			self.process_input_output_after_handshaking::<SD>(streaming_socket_file_descriptor, yielder, byte_counter)
		}
		else
		{
			Ok(())
		}
	}

	#[doc(hidden)]
	#[inline(always)]
    fn process_input_output_after_handshaking<SD: SocketData>(&mut self, streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<SD>, yielder: &mut InputOutputYielder, byte_counter: &mut ByteCounter) -> Result<bool, CompleteError>
    {
		self.complete_input_output::<SD>(streaming_socket_file_descriptor, yielder, byte_counter)
	}

	#[doc(hidden)]
	#[inline(always)]
	fn complete_input_output<SD: SocketData>(&mut self, streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<SD>, yielder: &mut InputOutputYielder, byte_counter: &mut ByteCounter) -> Result<bool, CompleteError>
	{
		use self::TlsInputOutputError::*;

		// rustls always wants to write if its output buffer is not empty.
		while self.wants_write()
		{
			loop
			{
				match self.write_tls_vectored(streaming_socket_file_descriptor)
				{
					Err(io_error) => write_loop_or_await_or_error!(io_error, yielder, SocketVectoredWrite),

					#[cfg(debug_assertions)] Ok(0) => panic!("Writes should always write more than one byte"),

					Ok(bytes_written) =>
					{
						byte_counter.bytes_written(bytes_written);
						break
					}
				}
			}
		}

		// rustls always wants to read more data all the time, except when there is unprocessed (readable) plaintext, ie its plaintext buffer is not empty.
		//
		// During handshaking, there is never unprocessed (readable) plaintext, and so `self.wants_read()` is always true.
		let bytes_read = if self.wants_read()
		{
			loop
			{
				let bytes_read = match self.read_tls(streaming_socket_file_descriptor)
				{
					Err(io_error) => read_loop_or_await_or_error!(io_error, yielder, SocketRead),

					Ok(bytes_read) => bytes_read,
				};

				if let Err(tls_error) = self.process_new_packets()
				{
					// In case we have a TLS alert message to send describing this error we do a final write.
					loop
					{
						match self.write_tls_vectored(streaming_socket_file_descriptor)
						{
							Err(io_error) => write_loop_or_await_or_error!(io_error, yielder, SocketVectoredWrite),

							#[cfg(debug_assertions)] Ok(0) => panic!("Writes should always write more than one byte"),

							Ok(bytes_written) =>
							{
								byte_counter.bytes_written(bytes_written);
								break
							}
						}
					}

					return Err(CompleteError::from(ProcessNewPackets(error, io_error.err())));
				}

				break bytes_read
			}
		};

		byte_counter.bytes_read(bytes_read);
		let is_end_of_file = bytes_read == 0;
		Ok(is_end_of_file)
	}

	#[doc(hidden)]
	#[inline(always)]
	fn write_tls_vectored<SD: SocketData>(&mut self, streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<SD>) -> io::Result<usize>
	{
		struct WriteVAdaptor<'a, SD: 'a>(&'a StreamingSocketFileDescriptor<SD>);

		impl WriteV for X
		{
			#[inline(always)]
			fn writev(&mut self, vbytes: &[&[u8]]) -> Result<usize>
			{
				self.write_vectored(vbytes)
			}
		}

		self.writev_tls(WriteVAdapter(streaming_socket_file_descriptor))
	}
}

impl SessionExt for ClientSession
{
}

impl SessionExt for ServerSession
{
}

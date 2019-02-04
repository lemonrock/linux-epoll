// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


macro_rules! stream_read_write
{
	($name: ident) =>
	{
		impl<'yielder, SD: SocketData> Read for $name<'yielder, SD>
		{
			/// Will ***never*** return `ErrorKind::Interupted` or `ErrorKind::WouldBlock`.
			///
			/// Will never return `Ok(0)` unless `buf.is_empty()`.
			///
			/// Smuggles `CompleteError::Killed` as `ErrorKind::ConnectionAborted`.
			/// Unwraps `CompleteError::Undifferentiated`, `CompleteError::SocketRead`, `CompleteError::SocketWrite`, `CompleteError::SocketReadVectored` and `CompleteError::SocketWriteVectored` as their underlying `io::Error`.
			/// Converts all other `CompleteError` variants to `ErrorKind::Other`.
			#[inline(always)]
			fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
			{
				CompleteError::convert_to_io_error(self.read_data(buf))
			}

			#[inline(always)]
			unsafe fn initializer(&self) -> Initializer
			{
				Initializer::nop()
			}
		}

		impl<'yielder, SD: SocketData> Write for $name<'yielder, SD>
		{
			/// Will ***never*** return `ErrorKind::Interupted` or `ErrorKind::WouldBlock`.
			///
			/// Will never return `Ok(0)` unless `buf.is_empty()`.
			///
			/// Smuggles `CompleteError::Killed` as `ErrorKind::ConnectionAborted`.
			/// Unwraps `CompleteError::Undifferentiated`, `CompleteError::SocketRead`, `CompleteError::SocketWrite`, `CompleteError::SocketReadVectored` and `CompleteError::SocketWriteVectored` as their underlying `io::Error`.
			/// Converts all other `CompleteError` variants to `ErrorKind::Other`.
			#[inline(always)]
			fn write(&mut self, buf: &[u8]) -> io::Result<usize>
			{
				CompleteError::convert_to_io_error(self.write_data(buf))
			}

			/// Will ***never*** return `ErrorKind::Interupted` or `ErrorKind::WouldBlock`.
			///
			/// Will never return `Ok(0)` unless `buf.is_empty()`.
			///
			/// Smuggles `CompleteError::Killed` as `ErrorKind::ConnectionAborted`.
			/// Unwraps `CompleteError::Undifferentiated`, `CompleteError::SocketRead`, `CompleteError::SocketWrite`, `CompleteError::SocketReadVectored` and `CompleteError::SocketWriteVectored` as their underlying `io::Error`.
			/// Converts all other `CompleteError` variants to `ErrorKind::Other`.
			#[inline(always)]
			fn flush(&mut self) -> io::Result<()>
			{
				CompleteError::convert_to_io_error(self.flush_written_data())
			}
		}
	}
}

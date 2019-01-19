// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Error occuring on completion of a coroutine.
pub enum CompleteError
{
	/// An `io::Error` was converted from some other cause (eg from using a third-party library that wraps implementations of `io::Read` and `io::Write`).
	Undifferentiated(io::Error),

	/// A socket read failed with an irrecoverable `io::Error` (not `Interupted` or `WouldBlock`).
	SocketRead(io::Error),

	/// A socket write failed with an irrecoverable `io::Error` (not `Interupted` or `WouldBlock`).
	SocketWrite(io::Error),

	/// A socket vectored read failed with an irrecoverable `io::Error` (not `Interupted` or `WouldBlock`).
	SocketVectoredRead(io::Error),

	/// A socket vectored write failed with an irrecoverable `io::Error` (not `Interupted` or `WouldBlock`).
	SocketVectoredWrite(io::Error),

	/// A socket read, socket write, socket vectored read or socket vectored write would have blocked; after waiting for input or output to become available with epoll (Event Poll), the connection closed with an error.
	ClosedWithError,

	/// A socket read, socket write, socket vectored read or socket vectored write would have blocked; after waiting for input or output to become available with epoll (Event Poll), the remote peer cleanly closed the connection.
	///
	/// This can also happen for TCP 'half-close' shutdowns (which are near useless on modern socket protocols that use TLS).
	RemotePeerClosedCleanly,

	/// The coroutine managing the socket was killed.
	///
	/// Typically this is because the parent that owns it is being dropped.
	Killed,

	/// An error relating to TLS occurred.
	Tls(TlsInputOutputError),
}

impl Display for CompleteError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for CompleteError
{
	#[inline(always)]
	fn source(&self) -> Option<&(error::Error + 'static)>
	{
		use self::CompleteError::*;

		match self
		{
			&Undifferentiated(ref error) => Some(error),

			&SocketVectoredRead(ref error) => Some(error),

			&SocketVectoredWrite(ref error) => Some(error),

			&SocketRead(ref error) => Some(error),

			&SocketWrite(ref error) => Some(error),

			&ClosedWithError => None,

			&RemotePeerClosedCleanly => None,

			&Killed => None,

			&Tls(ref error) => Some(error),
		}
	}
}

impl From<io::Error> for CompleteError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		CompleteError::Undifferentiated(error)
	}
}

impl From<TlsInputOutputError> for CompleteError
{
	#[inline(always)]
	fn from(error: TlsInputOutputError) -> Self
	{
		CompleteError::Tls(error)
	}
}

impl CompleteError
{
	#[inline(always)]
	pub(crate) fn convert_to_io_error<T>(result: Result<T, CompleteError>) -> Result<T, io::Error>
	{
		use self::CompleteError::*;
		use self::ErrorKind::*;

		match result
		{
			Ok(()) => Ok(()),

			Err(complete_error) => Err
			(
				match complete_error
				{
					Killed => io::Error::from(ConnectionAborted),

					Undifferentiated(io_error) | SocketRead(io_error) | SocketWrite(io_error) | SocketReadVectored(io_error) | SocketWriteVectored(io_error) => io_error,

					_ => io::Error::from(Other),
				}
			)
		}
	}
}

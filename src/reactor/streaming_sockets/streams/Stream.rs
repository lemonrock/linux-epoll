// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A trait to make common the differences between TLS server streams, TLS client streams and unencrypted streams.
///
/// Also offers wrapper implementations of `io::Read` and `io::Write`, but it advisable to use `read_data()` and `write_data()` in preference.
pub trait Stream<'a>: Read + Write
{
	/// Type of the information available to the stream after handshaking has completed.
	type PostHandshakeInformation: Sized;

	/// Information available to the stream after handshaking has completed.
	///
	/// Constructing this information is slightly expensive and may involve malloc, so it is advisable to only call this method once.
	///
	/// Since a stream is only made available after handshaking has been successful, this method always succeeds.
	///
	/// For a unencrypted stream, nothing useful is available as no handshaking occurs, although if support for `STARTTLS` in LDAP, SMTP, POP3 and IMAP is added it may be slightly useful.
	#[inline(always)]
	fn post_handshake_information(&'a self) -> Self::PostHandshakeInformation;

	/// Read data.
	///
	/// Appears to the user of this API to be blocking, but in practice it uses a coroutine.
	fn read_data(&mut self, read_into_buffer: &mut [u8]) -> Result<usize, CompleteError>;

	/// Write data.
	///
	/// Appears to the user of this API to be blocking, but in practice it uses a coroutine.
	fn write_data(&mut self, write_from_buffer: &[u8]) -> Result<usize, CompleteError>;

	/// Flush written data.
	///
	/// Not particularly useful, and there is no need to use this before calling `read_data()`, `write_data()` or `finish()`, all of which are self-flushing.
	fn flush_written_data(&mut self) -> Result<(), CompleteError>;

	/// Used to indicate that the user has finished with the stream.
	///
	/// Unencrypted streams will do nothing in this method; TLS streams will try to send a CloseNotify alert.
	fn finish(self) -> Result<(), CompleteError>;
}

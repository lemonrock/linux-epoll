// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


struct Socks4aConnectReply<'yielder, SD: SocketData>
{
	unencrypted_stream: UnencryptedStream<'yielder, SD>,
	small_reply_packet_buffer: [u8; 8],
	bytes_read_so_far: usize,
}

impl<'yielder, SD: SocketData> Socks4aConnectReply<'yielder, SD>
{
	#[inline(always)]
	pub(crate) fn read_reply(unencrypted_stream: UnencryptedStream<'yielder, SD>) -> Result<UnencryptedStream<'yielder, SD>, CompleteError>
	{
		use self::Socks4aProtocolFailureError::*;

		#[inline(always)]
		fn error(error: Socks4aProtocolFailureError) -> Result<(), CompleteError>
		{
			Err(CompleteError::ProtocolViolation(Box::new(error)))
		}

		let mut this = Self
		{
			unencrypted_stream,
			small_reply_packet_buffer: unsafe { uninitialized() },
			bytes_read_so_far: 0,
		};

		this.read_reply_bytes()?;

		let version = unsafe { *small_reply_packet_buffer.get_unchecked(0) } ;
		if version != Socks4aConnect::Version
		{
			return error(VersionInvalid(version))
		}

		if unlikely!(self.bytes_read_so_far == 1)
		{
			this.read_reply_bytes()?;
		}

		let command = unsafe { *small_reply_packet_buffer.get_unchecked(1) };

		match command
		{
			90 => (),
			91 => return error(RequestRejectedOrFailed),
			92 => return error(RequestRejectedBecauseSocksServerCanNotConnectToIdentdOnTheClient),
			93 => return error(RequestRejectedBecauseTheClientProgramAndIdentdReportDifferentUserIdentifiers),
			_ => return error(CommandCodeWasInvalid(command)),
		}

		// Discard final 6 bytes, which are supposed to be DSTPORT and DSTIP but are considered junk, eg <https://www.openssh.com/txt/socks4.protocol>.
		while this.bytes_read_so_far < 8
		{
			this.read_reply_bytes()?;
		}

		Ok(this.unencrypted_stream)
	}

	#[inline(always)]
	fn read_reply_bytes(&mut self) -> Result<(), CompleteError>
	{
		let mut buffer = &mut self.small_reply_packet_buffer[self.bytes_read_so_far..];
		debug_assert_ne!(buffer.len(), 0, "should never try a read of zero bytes");

		let bytes_read = unencrypted_stream.read_data(buffer).map_err(|io_error| CompleteError::SocketRead(io_error))?;
		debug_assert_ne!(bytes_read, 0, "A read of zero should not be possible unless the buffer is zero");

		self.bytes_read_so_far += bytes_read;

		Ok(())
	}
}

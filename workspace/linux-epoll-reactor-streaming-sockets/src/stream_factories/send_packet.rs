// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn send_packet<'yielder, SD: SocketData>(unencrypted_stream: UnencryptedStream<'yielder, SD>, packet: &[u8]) -> Result<UnencryptedStream<'yielder, SD>, CompleteError>
{
	let mut bytes_already_written = 0;
	let mut bytes_remaining_to_write = packet.len();

	while
	{
		let bytes_written = unencrypted_stream.write_data(&packet[bytes_already_written .. ]).map_err(|io_error| CompleteError::SocketRead(io_error))?;
		bytes_remaining_to_write -= bytes_written;
		bytes_already_written += bytes_written;
		bytes_remaining_to_write != 0
	}
	{
	}

	Ok(unencrypted_stream)
}

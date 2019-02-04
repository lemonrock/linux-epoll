// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// An error in TLS handling.
///
/// All are irrecoverable.
#[derive(Debug)]
pub enum TlsInputOutputError
{
	/// Error occurred whilst processing newly received TLS packets (messages).
	ProcessNewPackets(TLSError),

	/// End-of-file occurred whilst handshaking.
	EndOfFileWhilstHandshaking,

	/// When reading from a socket, a CloseNotify fatal alart was received.
	BufferReadCloseNotifyAlertReceived,
}

impl Display for TlsInputOutputError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for TlsInputOutputError
{
	#[inline(always)]
	fn source(&self) -> Option<&(error::Error + 'static)>
	{
		use self::TlsInputOutputError::*;

		match self
		{
			&ProcessNewPackets(ref error) => Some(error),

			&EndOfFileWhilstHandshaking => None,

			&BufferReadCloseNotifyAlertReceived => None,
		}
	}
}

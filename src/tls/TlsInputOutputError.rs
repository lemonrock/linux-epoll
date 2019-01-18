// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) enum TlsInputOutputError
{
	SocketVectoredWrite(io::Error),

	SocketRead(io::Error),

	Killed,

	ProcessNewPackets(TLSError, Option<io::Error>),

	EndOfFileWhilstHandshaking,

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
			&SocketVectoredWrite(ref error) => Some(error),

			&SocketRead(ref error) => Some(error),

			&Killed => None,

			&ProcessNewPackets(ref error, ..) => Some(error),

			&EndOfFileWhilstHandshaking => None,

			&BufferReadCloseNotifyAlertReceived => None,
		}
	}
}

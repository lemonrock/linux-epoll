// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Error occuring on completion of a coroutine.
pub enum CompleteError
{
	SocketVectoredWrite(io::Error),

	SocketRead(io::Error),

	Killed,

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
			&SocketVectoredWrite(ref error) => Some(error),

			&SocketRead(ref error) => Some(error),

			&Killed => None,

			&Tls(ref error) => Some(error),
		}
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

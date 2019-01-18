// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An error that can occur during registration with epoll.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventPollRegistrationError
{
	/// Error on creation of a file descriptor.
	Creation(CreationError),

	/// Could not allocate (out of memory in some way) from an `Arena`.
	Allocation(ArenaAllocationError),

	/// Could not internally add a file descriptor to an epoll instance.
	Add(EPollAddError),

	/// Could not create a file descriptor to register with an epoll instance.
	NewSocketServerListener(NewSocketServerListenerError),
}

impl Display for EventPollRegistrationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<EventPollRegistrationError as Debug>::fmt(self, f)
	}
}

impl error::Error for EventPollRegistrationError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(error::Error + 'static)>
	{
		use self::EventPollRegistrationError::*;

		match self
		{
			&Creation(ref error) => Some(error),

			&Allocation(ref error) => Some(error),

			&Add(ref error) => Some(error),

			&NewSocketServerListener(ref error) => Some(error),
		}
	}
}

impl From<CreationError> for EventPollRegistrationError
{
	#[inline(always)]
	fn from(error: CreationError) -> Self
	{
		EventPollRegistrationError::Creation(error)
	}
}

impl From<ArenaAllocationError> for EventPollRegistrationError
{
	#[inline(always)]
	fn from(error: ArenaAllocationError) -> Self
	{
		EventPollRegistrationError::Allocation(error)
	}
}

impl From<EPollAddError> for EventPollRegistrationError
{
	#[inline(always)]
	fn from(error: EPollAddError) -> Self
	{
		EventPollRegistrationError::Add(error)
	}
}

impl From<NewSocketServerListenerError> for EventPollRegistrationError
{
	#[inline(always)]
	fn from(error: NewSocketServerListenerError) -> Self
	{
		EventPollRegistrationError::NewSocketServerListener(error)
	}
}

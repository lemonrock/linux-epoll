// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A dummy structure suitable when not using a particular kind of file descriptor.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unused;

impl AsRawFd for Unused
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		panic!("Should not be called")
	}
}

impl FromRawFd for Unused
{
	#[inline(always)]
	unsafe fn from_raw_fd(_raw_file_descriptor: RawFd) -> Self
	{
		Self
	}
}

impl Reactor for Unused
{
	type FileDescriptor = Self;

	const FileDescriptorKind: FileDescriptorKind = FileDescriptorKind::CharacterDevice;

	type RegistrationData = ();

	#[inline(always)]
	fn our_arena(arenas: &impl Arenas) -> &Arena<Self>
	{
		panic!("Does not have an arena");
	}

	#[inline(always)]
	fn do_initial_input_and_output_and_register_with_epoll_if_necesssary(_event_poll: &EventPoll<impl Arenas>, _registration_data: Self::RegistrationData) -> Result<(), EventPollRegistrationError>
	{
		panic!("Can not be registered")
	}

	#[inline(always)]
	fn react(&mut self, _file_descriptor: &Self::FileDescriptor, _event_flags: EPollEventFlags, _terminate: &impl Terminate) -> Result<bool, String>
	{
		panic!("Can not react")
	}
}

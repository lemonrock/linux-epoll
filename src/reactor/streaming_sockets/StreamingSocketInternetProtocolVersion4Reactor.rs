// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// This object wraps streaming sockets.
#[derive(Debug)]
struct StreamingSocketInternetProtocolVersion4Reactor<'a, S: Sized + Deref<Target=Stack>, SF: StreamFactory<'a, sockaddr_in>, SU: StreamUser<'a, sockaddr_in>>
{
	inner: StreamingSocketCommon<'a, S, SF, SU, sockaddr_in>,
}

impl<'a, S: Sized + Deref<Target=Stack>, SF: StreamFactory<'a, sockaddr_in>, SU: StreamUser<'a, sockaddr_in>> Reactor for StreamingSocketInternetProtocolVersion4Reactor<'a, S, SF, SU>
{
	type FileDescriptor = StreamingSocketInternetProtocolVersion4FileDescriptor;

	const FileDescriptorKind: FileDescriptorKind = FileDescriptorKind::StreamingSocketInternetProtocolVersion4;

	type RegistrationData = (StreamingSocketFileDescriptor<sockaddr_in>, &'a SF, SF::AdditionalArguments, &'a SU);

	#[inline(always)]
	fn our_arena(arenas: &impl Arenas) -> &Arena<Self>
	{
		arenas.streaming_socket_internet_protocol_version_4()
	}

	#[inline(always)]
	fn do_initial_input_and_output_and_register_with_epoll_if_necesssary(event_poll: &EventPoll<impl Arenas>, registration_data: Self::RegistrationData) -> Result<(), EventPollRegistrationError>
	{
		StreamingSocketCommon::<SF, SU, sockaddr_in>::do_initial_input_and_output_and_register_with_epoll_if_necesssary(event_poll, registration_data)
	}

	#[inline(always)]
	fn react(&mut self, event_poll: &EventPoll<impl Arenas>, file_descriptor: &Self::FileDescriptor, event_flags: EPollEventFlags, terminate: &impl Terminate) -> Result<bool, String>
	{
		self.inner.react(event_poll, file_descriptor, event_flags, terminate)
	}
}

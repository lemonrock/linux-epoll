// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


macro_rules! streaming_socket_reactor
{
	($reactor_name: ident, $sockaddr_type: ty, $file_descriptor_name: ty, $file_descriptor_kind: ident, $lower_case_kind: ident) =>
	{
		/// This object wraps streaming sockets.
		#[derive(Debug)]
		struct $reactor_name<'a, SF: 'a + StreamFactory<'a, $sockaddr_type>, SU: 'a + StreamUser<'a, SF::S>>
		{
			inner: StreamingSocketCommon<'a, SF, SU, $sockaddr_type>,
		}

		impl<'a, SF: 'a + StreamFactory<'a, $sockaddr_type>, SU: 'a + StreamUser<'a, SF::S>> Reactor for $reactor_name<'a, SF, SU>
		{
			type FileDescriptor = $file_descriptor_name;

			const FileDescriptorKind: FileDescriptorKind = FileDescriptorKind::$file_descriptor_kind;

			type RegistrationData = (StreamingSocketFileDescriptor<$sockaddr_type>, &'a SF, SF::AdditionalArguments, &'a SU);

			#[inline(always)]
			fn our_arena(arenas: &impl Arenas) -> &Arena<Self>
			{
				arenas.$lower_case_kind()
			}

			#[inline(always)]
			fn do_initial_input_and_output_and_register_with_epoll_if_necesssary(event_poll: &EventPoll<impl Arenas>, registration_data: Self::RegistrationData) -> Result<(), EventPollRegistrationError>
			{
				StreamingSocketCommon::<SF, SU, $sockaddr_type>::do_initial_input_and_output_and_register_with_epoll_if_necesssary::<Self>(event_poll, registration_data)
			}

			#[inline(always)]
			fn react(&mut self, event_poll: &EventPoll<impl Arenas>, file_descriptor: &Self::FileDescriptor, event_flags: EPollEventFlags, terminate: &impl Terminate) -> Result<bool, String>
			{
				self.inner.react(event_poll, file_descriptor, event_flags, terminate)
			}
		}

		impl<'a, SF: 'a + StreamFactory<'a, $sockaddr_type>, SU: 'a + StreamUser<'a, SF::S>> StreamingSocketReactor<'a, SF, SU, $sockaddr_type> for $reactor_name<'a, SF, SU>
		{
		}
	}
}



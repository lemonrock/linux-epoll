// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


macro_rules! streaming_socket_reactor
{
	($reactor_name: ident, $sockaddr_type: ty, $file_descriptor_name: ty, $title_case: ident, $lower_case_kind: ident, $title_case_arena: ident) =>
	{
		/// This object wraps streaming sockets.
		#[derive(Debug)]
		struct $reactor_name<SF: StreamFactory<$sockaddr_type>, SU: StreamUser<SF::S>>
		{
			common: StreamingSocketCommon<SF, SU, $sockaddr_type>,
		}

		impl<SF: StreamFactory<$sockaddr_type>, SU: StreamUser<SF::S>, AS: Arenas<$title_case=Self, $title_case_arena=A>, A: Arena<Self, AS>> Reactor<AS, A> for $reactor_name<SF, SU>
		{
			type FileDescriptor = $file_descriptor_name;

			const FileDescriptorKind: FileDescriptorKind = FileDescriptorKind::$title_case;

			type RegistrationData = (StreamingSocketFileDescriptor<$sockaddr_type>, &SF, SF::AdditionalArguments, Rc<SU>);

			#[inline(always)]
			fn our_arena(arenas: &AS) -> &A
			{
				arenas.$lower_case_kind()
			}

			#[inline(always)]
			fn do_initial_input_and_output_and_register_with_epoll_if_necesssary(event_poll: &EventPoll<AS>, registration_data: Self::RegistrationData) -> Result<(), EventPollRegistrationError>
			{
				StreamingSocketCommon::<SF, SU, $sockaddr_type>::do_initial_input_and_output_and_register_with_epoll_if_necesssary::<Self, AS, A>(event_poll, registration_data)
			}

			#[inline(always)]
			fn react(&mut self, event_poll: &EventPoll<AS>, file_descriptor: &Self::FileDescriptor, event_flags: EPollEventFlags, terminate: &impl Terminate) -> Result<bool, String>
			{
				self.common.react(event_poll, file_descriptor, event_flags, terminate)
			}
		}

		impl<SF: StreamFactory<$sockaddr_type>, SU: StreamUser<SF::S>, AS: Arenas<$title_case=Self, $title_case_arena=A>, A: Arena<Self, AS>> StreamingSocketReactor<SF, SU, $sockaddr_type, AS, A> for $reactor_name<SF, SU>
		{
			#[inline(always)]
			fn initialize(&mut self, common: StreamingSocketCommon<SF, SU, $sockaddr_type>)
			{
				unsafe { write(&mut self.common, common) }
			}
		}
	}
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


macro_rules! streaming_server_listener_reactor
{
	($reactor_name: ident, $sockaddr_type: ty, $file_descriptor_name: ty, $title_case: ident, $rust_socket_type: ty, $lower_case_kind: ident, $title_case_arena: ident) =>
	{
		/// This object wraps streaming server listener sockets of variety `$title_case`.
		///
		/// Construction is done via `do_initial_input_and_output_and_register_with_epoll_if_necesssary()` and *MUST* occur after the current thread has had its affinity forced to the current CPU.
		#[derive(Debug)]
		pub struct $reactor_name<AC: AccessControl<$sockaddr_type>>
		{
			common: StreamingServerListenerSocketCommon<$sockaddr_type, AC>,
		}

		impl<AC: AccessControl<$sockaddr_type>, AS: Arenas<$title_case=Self, $title_case_arena=A>, A: Arena<Self, AS>> Reactor<AS, A> for $reactor_name<AC>
		{
			type FileDescriptor = $file_descriptor_name;

			const FileDescriptorKind: FileDescriptorKind = FileDescriptorKind::$title_case;

			type RegistrationData = (Arc<StreamingServerListenerSocketSettings>, $rust_socket_type, AC, FileDescriptorDistributor<$sockaddr_type>);

			#[inline(always)]
			fn our_arena(arenas: &AS) -> &A
			{
				arenas.$lower_case_kind()
			}

			#[inline(always)]
			fn do_initial_input_and_output_and_register_with_epoll_if_necesssary(event_poll: &EventPoll<AS>, registration_data: Self::RegistrationData) -> Result<(), EventPollRegistrationError>
			{
				let (settings, socket_address, access_control, file_descriptor_distributor) = registration_data;

				let streaming_server_listener_socket_file_descriptor = StreamingServerListenerSocketCommon::<$sockaddr_type, AC>::new_streaming_socket_file_descriptor(&settings, socket_address)?;

				StreamingServerListenerSocketCommon::<$sockaddr_type, AC>::do_initial_input_and_output_and_register_with_epoll_if_necesssary::<Self, AS, A>(event_poll, streaming_server_listener_socket_file_descriptor, access_control, file_descriptor_distributor)
			}

			#[inline(always)]
			fn react(&mut self, event_poll: &EventPoll<AS>, file_descriptor: &Self::FileDescriptor, event_flags: EPollEventFlags, terminate: &impl Terminate) -> Result<bool, String>
			{
				self.common.react(event_poll, file_descriptor, event_flags, terminate)
			}
		}

		impl<AC: AccessControl<$sockaddr_type>, AS: Arenas<$title_case=Self, $title_case_arena=A>, A: Arena<Self, AS>> StreamingServerListenerSocketReactor<$sockaddr_type, AC, AS, A> for $reactor_name<AC>
		{
			#[inline(always)]
			fn initialize(&mut self, common: StreamingServerListenerSocketCommon<$sockaddr_type, AC>)
			{
				unsafe { write(&mut self.common, common) }
			}
		}
	}
}

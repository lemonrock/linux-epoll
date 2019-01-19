// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


macro_rules! streaming_server_listener_reactor
{
	($reactor_name: ident, $sockaddr_type: ty, $file_descriptor_name: ty, $file_descriptor_kind: ident, $rust_socket_type: ty, $lower_case_kind: ident) =>
	{
		/// This object wraps streaming server listener sockets of variety `$file_descriptor_kind`.
		///
		/// Construction is done via `do_initial_input_and_output_and_register_with_epoll_if_necesssary()` and *MUST* occur after the current thread has had its affinity forced to the current CPU.
		#[derive(Debug)]
		pub struct $reactor_name<A: AccessControl<$sockaddr_type>>
		{
			common: StreamingServerListenerSocketCommon<$sockaddr_type, A>,
		}

		impl<A: AccessControl<$sockaddr_type>> Reactor for $reactor_name<A>
		{
			type FileDescriptor = $file_descriptor_name;

			const FileDescriptorKind: FileDescriptorKind = FileDescriptorKind::$file_descriptor_kind;

			type RegistrationData = (StreamingServerListenerSocketSettings, $rust_socket_type, A, FileDescriptorDistributor<$sockaddr_type>);

			#[inline(always)]
			fn our_arena(arenas: &impl Arenas) -> &Arena<Self>
			{
				arenas.$lower_case_kind()
			}

			#[inline(always)]
			fn do_initial_input_and_output_and_register_with_epoll_if_necesssary(event_poll: &EventPoll<impl Arenas>, registration_data: Self::RegistrationData) -> Result<(), EventPollRegistrationError>
			{
				let (settings, socket_address, access_control, file_descriptor_distributor) = registration_data;

				let streaming_server_listener_socket_file_descriptor = StreamingServerListenerSocketCommon::<$sockaddr_type, A>::new_streaming_socket_file_descriptor(settings, socket_address)?;

				StreamingServerListenerSocketCommon::<$sockaddr_type, A>::do_initial_input_and_output_and_register_with_epoll_if_necesssary(event_poll, streaming_server_listener_socket_file_descriptor, access_control, file_descriptor_distributor)
			}

			#[inline(always)]
			fn react(&mut self, event_poll: &EventPoll<impl Arenas>, file_descriptor: &Self::FileDescriptor, event_flags: EPollEventFlags, terminate: &impl Terminate) -> Result<bool, String>
			{
				self.common.react(event_poll, file_descriptor, event_flags, terminate)
			}
		}

		impl<A: AccessControl<$sockaddr_type>> StreamingServerListenerReactor<$sockaddr_type, A> for $reactor_name<A>
		{
			#[inline(always)]
			fn initialize(&mut self, common: StreamingServerListenerSocketCommon<$sockaddr_type, A>)
			{
				unsafe { write(&mut self.common, common) }
			}
		}
	}
}

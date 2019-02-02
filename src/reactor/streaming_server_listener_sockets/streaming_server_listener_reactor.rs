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

		impl<AC: AccessControl<$sockaddr_type>> Reactor for $reactor_name<AC>
		{
			type FileDescriptor = $file_descriptor_name;

			type RegistrationData = (Arc<StreamingServerListenerSocketSettings>, $rust_socket_type, AC, FileDescriptorDistributor<$sockaddr_type>, QueuePerThreadQueuesPublisher<(), String>, CompressedTypeIdentifier, u8);

			#[inline(always)]
			fn do_initial_input_and_output_and_register_with_epoll_if_necesssary<A: Arena<Self>, T: Terminate>(event_poll: &EventPoll<T>, arena: &A, reactor_compressed_type_identifier: CompressedTypeIdentifier, registration_data: Self::RegistrationData) -> Result<(), EventPollRegistrationError>
			{
				let (settings, socket_address, access_control, file_descriptor_distributor) = registration_data;

				let streaming_server_listener_socket_file_descriptor = StreamingServerListenerSocketCommon::<$sockaddr_type, AC>::new_streaming_socket_file_descriptor(&settings, socket_address)?;

				StreamingServerListenerSocketCommon::<$sockaddr_type, AC>::do_initial_input_and_output_and_register_with_epoll_if_necesssary::<Self, A, T>(event_poll, arena, reactor_compressed_type_identifier, streaming_server_listener_socket_file_descriptor, access_control, file_descriptor_distributor)
			}

			#[inline(always)]
			fn react(&mut self, event_flags: EPollEventFlags, terminate: &impl Terminate) -> Result<bool, String>
			{
				self.common.react(event_flags, terminate)
			}
		}

		impl<AC: AccessControl<$sockaddr_type>> StreamingServerListenerSocketReactor<$sockaddr_type, AC> for $reactor_name<AC>
		{
			#[inline(always)]
			fn initialize(&mut self, common: StreamingServerListenerSocketCommon<$sockaddr_type, AC>)
			{
				unsafe { write(&mut self.common, common) }
			}
		}
	}
}

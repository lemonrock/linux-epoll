// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug)]
struct StreamingServerListenerSocketCommon<SD: SocketData, AC: AccessControl<SD>>
{
	streaming_server_listener_socket_file_descriptor: StreamingServerListenerSocketFileDescriptor<SD>,
	access_control: AC,
	publisher: QueuePerThreadQueuesPublisher<(), String>,
	accepted_streaming_socket_message_compressed_type_identifier: CompressedTypeIdentifier,
	streaming_socket_service_identifier: u8,
}

impl<SD: SocketData, AC: AccessControl<SD>> StreamingServerListenerSocketCommon<SD, AC>
{
	#[inline(always)]
	fn do_initial_input_and_output_and_register_with_epoll_if_necesssary<SSLSR: StreamingServerListenerSocketReactor<SD, AC, A>, A: Arena<SSLSR>>(event_poll: &EventPoll, arena: &A, reactor_compressed_type_identifier: CompressedTypeIdentifier, streaming_server_listener_socket_file_descriptor: SSLSR::FileDescriptor, access_control: AC, publisher: QueuePerThreadQueuesPublisher<(), String>, accepted_streaming_socket_message_compressed_type_identifier: CompressedTypeIdentifier, streaming_socket_service_identifier: u8) -> Result<(), EventPollRegistrationError>
	{
		event_poll.register::<SSLSR, A, _>(arena, reactor_compressed_type_identifier, streaming_server_listener_socket_file_descriptor, EPollAddFlags::EdgeTriggeredInputExclusive, |uninitialized_reactor, streaming_server_listener_socket_file_descriptor|
		{
			uninitialized_reactor.initialize
			(
				Self
				{
					streaming_server_listener_socket_file_descriptor,
					access_control,
					publisher,
					accepted_streaming_socket_message_compressed_type_identifier,
					streaming_socket_service_identifier,
				}
			);
			Ok(())
		})
	}

	#[inline(always)]
	fn react(&mut self, event_flags: EPollEventFlags, terminate: &impl Terminate) -> Result<bool, String>
	{
		debug_assert_eq!(event_flags, EPollEventFlags::Input, "flags contained a flag other than `Input`");

		while terminate.should_continue()
		{
			use self::SocketAcceptError::*;

			match file_descriptor.accept()
			{
				Ok(AcceptedConnection { streaming_socket_file_descriptor, peer_address }) => if likely!(self.is_remote_peer_allowed(peer_address, &streaming_socket_file_descriptor))
				{
					let logical_core_identifier = streaming_socket_file_descriptor.logical_core_identifier();
					self.publisher.publish_message::<AcceptedStreamingSocket<SD>>(logical_core_identifier, self.accepted_streaming_socket_message_compressed_type_identifier, AcceptedStreamingSocketMessage::<SD>::initialize);
				},

				Err(error) => match error
				{
					Again => break,

					PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded | SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded | KernelWouldBeOutOfMemory => continue,

					ConnectionFailed(_connection_failed_reason @ _) => continue,

					Interrupted => continue,
				},
			}
		}

		Ok(false)
	}

	#[inline(always)]
	fn is_remote_peer_allowed(&self, remote_peer_address: SD, streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<SD>) -> bool
	{
		self.access_control.is_remote_peer_allowed(remote_peer_address, streaming_socket_file_descriptor)
	}
}

impl<AC: AccessControl<sockaddr_in>> StreamingServerListenerSocketCommon<sockaddr_in, AC>
{
	#[inline(always)]
	fn new_streaming_socket_file_descriptor(settings: &StreamingServerListenerSocketSettings, socket_address: SocketAddrV4) -> Result<StreamingServerListenerSocketFileDescriptor<sockaddr_in>, NewSocketServerListenerError>
	{
		SocketFileDescriptor::<sockaddr_in>::new_transmission_control_protocol_over_internet_protocol_version_4_server_listener
		(
			socket_address,
			settings.send_buffer_size_in_bytes,
			settings.receive_buffer_size_in_bytes,
			settings.idles_before_keep_alive_seconds,
			settings.keep_alive_interval_seconds,
			settings.maximum_keep_alive_probes,
			settings.linger_seconds,
			settings.linger_in_FIN_WAIT2_seconds,
			settings.maximum_SYN_transmits,
			settings.back_log,
			LogicalCores::current_logical_core(),
		)
	}
}

impl<AC: AccessControl<sockaddr_in6>> StreamingServerListenerSocketCommon<sockaddr_in6, AC>
{
	#[inline(always)]
	fn new_streaming_socket_file_descriptor(settings: &StreamingServerListenerSocketSettings, socket_address: SocketAddrV6) -> Result<StreamingServerListenerSocketFileDescriptor<sockaddr_in6>, NewSocketServerListenerError>
	{
		SocketFileDescriptor::<sockaddr_in6>::new_transmission_control_protocol_over_internet_protocol_version_6_server_listener
		(
			socket_address,
			settings.send_buffer_size_in_bytes,
			settings.receive_buffer_size_in_bytes,
			settings.idles_before_keep_alive_seconds,
			settings.keep_alive_interval_seconds,
			settings.maximum_keep_alive_probes,
			settings.linger_seconds,
			settings.linger_in_FIN_WAIT2_seconds,
			settings.maximum_SYN_transmits,
			settings.back_log,
			LogicalCores::current_logical_core(),
		)
	}
}

impl<AC: AccessControl<sockaddr_un>> StreamingServerListenerSocketCommon<sockaddr_un, AC>
{
	#[inline(always)]
	fn new_streaming_socket_file_descriptor(settings: &StreamingServerListenerSocketSettings, socket_address: UnixDomainSocketAddress) -> Result<StreamingServerListenerSocketFileDescriptor<sockaddr_un>, NewSocketServerListenerError>
	{
		SocketFileDescriptor::<sockaddr_un>::new_streaming_unix_domain_socket_server_listener
		(
			&socket_address.0,
			settings.send_buffer_size_in_bytes,
			settings.back_log,
			LogicalCores::current_logical_core(),
		)
	}
}

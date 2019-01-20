// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug)]
struct StreamingServerListenerSocketCommon<SD: SocketData, A: AccessControl<SD>>
{
	access_control: A,
	file_descriptor_distributor: FileDescriptorDistributor<SD>,
}

impl<SD: SocketData, A: AccessControl<SD>> StreamingServerListenerSocketCommon<SD, A>
{
	#[inline(always)]
	fn do_initial_input_and_output_and_register_with_epoll_if_necesssary<SSLR: StreamingServerListenerReactor<SD, A>>(event_poll: &EventPoll<impl Arenas>, streaming_server_listener_socket_file_descriptor: SSLR::FileDescriptor, access_control: A, file_descriptor_distributor: FileDescriptorDistributor<SD>) -> Result<(), EventPollRegistrationError>
	{
		const AddFlags: EPollAddFlags = EPollAddFlags::EdgeTriggeredInput | EPollAddFlags::Exclusive;

		event_poll.register::<SSLR>(streaming_server_listener_socket_file_descriptor, AddFlags, |uninitialized_reactor|
		{
			uninitialized_reactor.initialize
			(
				Self
				{
					access_control,
					file_descriptor_distributor,
				}
			);
			Ok(())
		})
	}

	#[inline(always)]
	fn react(&mut self, event_poll: &EventPoll<impl Arenas>, file_descriptor: &StreamingServerListenerSocketFileDescriptor<SD>, event_flags: EPollEventFlags, terminate: &impl Terminate) -> Result<bool, String>
	{
		debug_assert_eq!(event_flags, EPollEventFlags::Input, "flags contained a flag other than `Input`");

		while terminate.should_continue()
		{
			use self::SocketAcceptError::*;

			match file_descriptor.accept()
			{
				Ok(AcceptedConnection { streaming_socket_file_descriptor, peer_address }) => if likely!(self.is_remote_peer_allowed(peer_address, &streaming_socket_file_descriptor))
				{
					self.file_descriptor_distributor.assign(streaming_socket_file_descriptor)
				},

				Err(error) => match error
				{
					Again => return self.finish(),

					PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded | SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded | KernelWouldBeOutOfMemory => continue,

					ConnectionFailed(_connection_failed_reason @ _) => continue,

					Interrupted => continue,
				},
			}
		}

		self.finish()
	}

	#[inline(always)]
	fn is_remote_peer_allowed(&self, remote_peer_address: SD, streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<SD>) -> bool
	{
		self.access_control.is_remote_peer_allowed(remote_peer_address, streaming_socket_file_descriptor)
	}

	#[inline(always)]
	fn finish(&mut self) -> Result<bool, String>
	{
		self.file_descriptor_distributor.distribute();
		Ok(false)
	}
}

impl<A: AccessControl<sockaddr_in>> StreamingServerListenerSocketCommon<sockaddr_in, A>
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
			current_logical_cpu(),
		)
	}
}

impl<A: AccessControl<sockaddr_in6>> StreamingServerListenerSocketCommon<sockaddr_in6, A>
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
			current_logical_cpu(),
		)
	}
}

impl<A: AccessControl<sockaddr_un>> StreamingServerListenerSocketCommon<sockaddr_un, A>
{
	#[inline(always)]
	fn new_streaming_socket_file_descriptor(settings: &StreamingServerListenerSocketSettings, socket_address: UnixDomainSocketAddress) -> Result<StreamingServerListenerSocketFileDescriptor<sockaddr_un>, NewSocketServerListenerError>
	{
		SocketFileDescriptor::<sockaddr_un>::new_streaming_unix_domain_socket_server_listener
		(
			&socket_address.0,
			settings.send_buffer_size_in_bytes,
			settings.back_log,
			current_logical_cpu(),
		)
	}
}

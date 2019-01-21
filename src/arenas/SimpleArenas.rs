// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


macro_rules! simple_arenas_struct
{
	($($lower_case: ident, $title_case: ident, $title_case_arena: ident, $file_descriptor: ty,)*) =>
	{
		/// Holds a SimpleArena for each item of `FileDescriptorKind`.
		#[derive(Debug)]
		pub struct SimpleArenas
		<
			$(
				$title_case: Reactor<Self, SimpleArena<$title_case, Self>, FileDescriptor=$file_descriptor>,
			)*
		>
		{
			$(
				$lower_case: SimpleArena<$title_case, Self>,
			)*
		}

		impl
		<
			$(
				$title_case: Reactor<Self, SimpleArena<$title_case, Self>, FileDescriptor=$file_descriptor>,
			)*
		>
		Arenas for SimpleArenas
		<
			$(
				$title_case,
			)*
		>
		{
			$(
				type $title_case = $title_case;

				type $title_case_arena = SimpleArena<Self::$title_case, Self>;

				#[inline(always)]
				fn $lower_case(&self) -> &Self::$title_case_arena
				{
					&self.$lower_case
				}
			)*
		}


		impl
		<
			$(
				$title_case: Reactor<Self, SimpleArena<$title_case, Self>, FileDescriptor=$file_descriptor>,
			)*
		>
		SimpleArenas
		<
			$(
				$title_case,
			)*
		>
		{
			/// Creates a new instance.
			#[inline(always)]
			pub fn new
			(
				$(
					$lower_case: SimpleArena<$title_case, Self>,
				)*
			) -> Self
			{
				Self
				{
					$(
						$lower_case,
					)*
				}
			}
		}
	}
}

simple_arenas_struct!
(
	character_device, CharacterDevice, CharacterDeviceArena, CharacterDeviceFileDescriptor,

	event_poll, EventPoll, EventPollArena, EPollFileDescriptor,

	event, Event, EventArena, EventFileDescriptor,

	fanotify, FANotify, FANotifyArena, FanotifyFileDescriptor,

	inotify, INotify, INotifyArena, InotifyFileDescriptor,

	receive_pipe, ReceivePipe, ReceivePipeArena, ReceivePipeFileDescriptor,

	send_pipe, SendPipe, SendPipeArena, SendPipeFileDescriptor,

	receive_posix_message_queue, ReceivePosixMessageQueue, ReceivePosixMessageQueueArena, ReceivePosixMessageQueueFileDescriptor,

	send_posix_message_queue, SendPosixMessageQueue, SendPosixMessageQueueArena, SendPosixMessageQueueFileDescriptor,

	send_and_receive_posix_message_queue, SendAndReceivePosixMessageQueue, SendAndReceivePosixMessageQueueArena, SendAndReceivePosixMessageQueueFileDescriptor,

	signal, Signal, SignalArena, SignalFileDescriptor,

	terminal, Terminal, TerminalArena, TerminalFileDescriptor,

	timer, Timer, TimerArena, TimerFileDescriptor,

	datagram_client_socket_internet_protocol_version_4, DatagramClientSocketInternetProtocolVersion4, DatagramClientSocketInternetProtocolVersion4Arena, DatagramClientSocketInternetProtocolVersion4FileDescriptor,

	datagram_client_socket_internet_protocol_version_6, DatagramClientSocketInternetProtocolVersion6, DatagramClientSocketInternetProtocolVersion6Arena, DatagramClientSocketInternetProtocolVersion6FileDescriptor,

	datagram_client_socket_unix_domain, DatagramClientSocketUnixDomain, DatagramClientSocketUnixDomainArena, DatagramClientSocketUnixDomainFileDescriptor,

	datagram_server_listener_socket_internet_protocol_version_4, DatagramServerListenerSocketInternetProtocolVersion4, DatagramServerListenerSocketInternetProtocolVersion4Arena, DatagramServerListenerSocketInternetProtocolVersion4FileDescriptor,

	datagram_server_listener_socket_internet_protocol_version_6, DatagramServerListenerSocketInternetProtocolVersion6, DatagramServerListenerSocketInternetProtocolVersion6Arena, DatagramServerListenerSocketInternetProtocolVersion6FileDescriptor,

	datagram_server_listener_socket_unix_domain, DatagramServerListenerSocketUnixDomain, DatagramServerListenerSocketUnixDomainArena, DatagramServerListenerSocketUnixDomainFileDescriptor,

	streaming_socket_internet_protocol_version_4, StreamingSocketInternetProtocolVersion4, StreamingSocketInternetProtocolVersion4Arena, StreamingSocketInternetProtocolVersion4FileDescriptor,

	streaming_socket_internet_protocol_version_6, StreamingSocketInternetProtocolVersion6, StreamingSocketInternetProtocolVersion6Arena, StreamingSocketInternetProtocolVersion6FileDescriptor,

	streaming_socket_unix_domain, StreamingSocketUnixDomain, StreamingSocketUnixDomainArena, StreamingSocketUnixDomainFileDescriptor,

	streaming_server_listener_socket_internet_protocol_version_4, StreamingServerListenerSocketInternetProtocolVersion4, StreamingServerListenerSocketInternetProtocolVersion4Arena, StreamingServerListenerSocketInternetProtocolVersion4FileDescriptor,

	streaming_server_listener_socket_internet_protocol_version_6, StreamingServerListenerSocketInternetProtocolVersion6, StreamingServerListenerSocketInternetProtocolVersion6Arena, StreamingServerListenerSocketInternetProtocolVersion6FileDescriptor,

	streaming_server_listener_socket_unix_domain, StreamingServerListenerSocketUnixDomain, StreamingServerListenerSocketUnixDomainArena, StreamingServerListenerSocketUnixDomainFileDescriptor,
);

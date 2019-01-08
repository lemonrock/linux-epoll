// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


macro_rules! arena
{
	($lower_case: ident, $title_case: ident, $title_case_arena: ident) =>
	{
		/// Type (possibly an enum) of data held in an arena of `$title_case`s.
		type $title_case: Sized;

		/// Type of `$title_case` arena.
		type $title_case_arena: Arena<Self::$title_case>;

		/// Obtains the arena for `$title_case`s.
		fn $lower_case(&self) -> &Self::$title_case_arena;
	}
}

/// Represents the types of each Arena.
pub trait Arenas
{
	arena!(character_device, CharacterDevice, CharacterDeviceArena);

	arena!(event_poll, EventPoll, EventPollArena);

	arena!(event, Event, EventArena);

	arena!(fanotify, FANotify, FANotifyArena);

	arena!(inotify, INotify, INotifyArena);

	arena!(receive_pipe, ReceivePipe, ReceivePipeArena);

	arena!(send_pipe, SendPipe, SendPipeArena);

	arena!(receive_posix_message_queue, ReceivePosixMessageQueuePipe, ReceivePosixMessageQueueArena);

	arena!(send_posix_message_queue, SendPosixMessageQueuePipe, SendPosixMessageQueueArena);

	arena!(send_and_receive_posix_message_queue, SendAndReceivePosixMessageQueuePipe, SendAndReceivePosixMessageQueueArena);

	arena!(signal, Signal, SignalArena);

	arena!(terminal, Terminal, TerminalArena);

	arena!(timer, Timer, TimerArena);

	arena!(datagram_client_socket_internet_protocol_version_4, DatagramClientSocketInternetProtocolVersion4, DatagramClientSocketInternetProtocolVersion4Arena);

	arena!(datagram_client_socket_internet_protocol_version_6, DatagramClientSocketInternetProtocolVersion6, DatagramClientSocketInternetProtocolVersion6Arena);

	arena!(datagram_client_socket_internet_protocol_unix_domain, DatagramClientSocketUnixDomain, DatagramClientSocketUnixDomainArena);

	arena!(datagram_server_listener_socket_internet_protocol_version_4, DatagramServerListenerSocketInternetProtocolVersion4, DatagramServerListenerSocketInternetProtocolVersion4Arena);

	arena!(datagram_server_listener_socket_internet_protocol_version_6, DatagramServerListenerSocketInternetProtocolVersion6, DatagramServerListenerSocketInternetProtocolVersion6Arena);

	arena!(datagram_server_listener_socket_internet_protocol_unix_domain, DatagramServerListenerSocketUnixDomain, DatagramServerListenerSocketUnixDomainArena);

	arena!(streaming_socket_internet_protocol_version_4, StreamingSocketInternetProtocolVersion4, StreamingSocketInternetProtocolVersion4Arena);

	arena!(streaming_socket_internet_protocol_version_6, StreamingSocketInternetProtocolVersion6, StreamingSocketInternetProtocolVersion6Arena);

	arena!(streaming_socket_internet_protocol_unix_domain, StreamingSocketUnixDomain, StreamingSocketUnixDomainArena);

	arena!(streaming_server_listener_socket_internet_protocol_version_4, StreamingServerListenerSocketInternetProtocolVersion4, StreamingServerListenerSocketInternetProtocolVersion4Arena);

	arena!(streaming_server_listener_socket_internet_protocol_version_6, StreamingServerListenerSocketInternetProtocolVersion6, StreamingServerListenerSocketInternetProtocolVersion6Arena);

	arena!(streaming_server_listener_socket_internet_protocol_unix_domain, StreamingServerListenerSocketUnixDomain, StreamingServerListenerSocketUnixDomainArena);
}

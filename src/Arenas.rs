// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


macro_rules! arena
{
	($lower_case: ident, $title_case: ident, $title_case_arena: ident, $file_descriptor: ty) =>
	{
		/// Type (possibly an enum) of data held in an arena of `$title_case`s.
		type $title_case: UsesFileDescriptor<FileDescriptor=$file_descriptor>;

		/// Type of `$title_case` arena.
		type $title_case_arena: Arena<Self::$title_case>;

		/// Obtains the arena for `$title_case`s.
		fn $lower_case(&self) -> &Self::$title_case_arena;
	}
}

/// Represents the types of each Arena.
pub trait Arenas
{
	arena!(character_device, CharacterDevice, CharacterDeviceArena, CharacterDeviceFileDescriptor);

	arena!(event_poll, EventPoll, EventPollArena, EPollFileDescriptor);

	arena!(event, Event, EventArena, EventFileDescriptor);

	arena!(fanotify, FANotify, FANotifyArena, FanotifyFileDescriptor);

	arena!(inotify, INotify, INotifyArena, InotifyFileDescriptor);

	arena!(receive_pipe, ReceivePipe, ReceivePipeArena, ReceivePipeFileDescriptor);

	arena!(send_pipe, SendPipe, SendPipeArena, SendPipeFileDescriptor);

	arena!(receive_posix_message_queue, ReceivePosixMessageQueue, ReceivePosixMessageQueueArena, ReceivePosixMessageQueueFileDescriptor);

	arena!(send_posix_message_queue, SendPosixMessageQueue, SendPosixMessageQueueArena, SendPosixMessageQueueFileDescriptor);

	arena!(send_and_receive_posix_message_queue, SendAndReceivePosixMessageQueue, SendAndReceivePosixMessageQueueArena, SendAndReceivePosixMessageQueueFileDescriptor);

	arena!(signal, Signal, SignalArena, SignalFileDescriptor);

	arena!(terminal, Terminal, TerminalArena, TerminalFileDescriptor);

	arena!(timer, Timer, TimerArena, TimerFileDescriptor);

	arena!(datagram_client_socket_internet_protocol_version_4, DatagramClientSocketInternetProtocolVersion4, DatagramClientSocketInternetProtocolVersion4Arena, DatagramClientSocketInternetProtocolVersion4FileDescriptor);

	arena!(datagram_client_socket_internet_protocol_version_6, DatagramClientSocketInternetProtocolVersion6, DatagramClientSocketInternetProtocolVersion6Arena, DatagramClientSocketInternetProtocolVersion6FileDescriptor);

	arena!(datagram_client_socket_unix_domain, DatagramClientSocketUnixDomain, DatagramClientSocketUnixDomainArena, DatagramClientSocketUnixDomainFileDescriptor);

	arena!(datagram_server_listener_socket_internet_protocol_version_4, DatagramServerListenerSocketInternetProtocolVersion4, DatagramServerListenerSocketInternetProtocolVersion4Arena, DatagramServerListenerSocketInternetProtocolVersion4FileDescriptor);

	arena!(datagram_server_listener_socket_internet_protocol_version_6, DatagramServerListenerSocketInternetProtocolVersion6, DatagramServerListenerSocketInternetProtocolVersion6Arena, DatagramServerListenerSocketInternetProtocolVersion6FileDescriptor);

	arena!(datagram_server_listener_socket_unix_domain, DatagramServerListenerSocketUnixDomain, DatagramServerListenerSocketUnixDomainArena, DatagramServerListenerSocketUnixDomainFileDescriptor);

	arena!(streaming_socket_internet_protocol_version_4, StreamingSocketInternetProtocolVersion4, StreamingSocketInternetProtocolVersion4Arena, StreamingSocketInternetProtocolVersion4FileDescriptor);

	arena!(streaming_socket_internet_protocol_version_6, StreamingSocketInternetProtocolVersion6, StreamingSocketInternetProtocolVersion6Arena, StreamingSocketInternetProtocolVersion6FileDescriptor);

	arena!(streaming_socket_unix_domain, StreamingSocketUnixDomain, StreamingSocketUnixDomainArena, StreamingSocketUnixDomainFileDescriptor);

	arena!(streaming_server_listener_socket_internet_protocol_version_4, StreamingServerListenerSocketInternetProtocolVersion4, StreamingServerListenerSocketInternetProtocolVersion4Arena, StreamingServerListenerSocketInternetProtocolVersion4FileDescriptor);

	arena!(streaming_server_listener_socket_internet_protocol_version_6, StreamingServerListenerSocketInternetProtocolVersion6, StreamingServerListenerSocketInternetProtocolVersion6Arena, StreamingServerListenerSocketInternetProtocolVersion6FileDescriptor);

	arena!(streaming_server_listener_socket_unix_domain, StreamingServerListenerSocketUnixDomain, StreamingServerListenerSocketUnixDomainArena, StreamingServerListenerSocketUnixDomainFileDescriptor);
}

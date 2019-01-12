// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Holds an arena for each item of `FileDescriptorKind`.
#[derive(Debug)]
pub struct SimpleArenas
<
	CharacterDevice: UsesFileDescriptor<FileDescriptor=CharacterDeviceFileDescriptor>,
	EventPoll: UsesFileDescriptor<FileDescriptor=EPollFileDescriptor>,
	Event: UsesFileDescriptor<FileDescriptor=EventFileDescriptor>,
	FANotify: UsesFileDescriptor<FileDescriptor=FanotifyFileDescriptor>,
	INotify: UsesFileDescriptor<FileDescriptor=InotifyFileDescriptor>,
	ReceivePipe: UsesFileDescriptor<FileDescriptor=ReceivePipeFileDescriptor>,
	SendPipe: UsesFileDescriptor<FileDescriptor=SendPipeFileDescriptor>,
	ReceivePosixMessageQueue: UsesFileDescriptor<FileDescriptor=ReceivePosixMessageQueueFileDescriptor>,
	SendPosixMessageQueue: UsesFileDescriptor<FileDescriptor=SendPosixMessageQueueFileDescriptor>,
	SendAndReceivePosixMessageQueue: UsesFileDescriptor<FileDescriptor=SendAndReceivePosixMessageQueueFileDescriptor>,
	Signal: UsesFileDescriptor<FileDescriptor=SignalFileDescriptor>,
	Terminal: UsesFileDescriptor<FileDescriptor=TerminalFileDescriptor>,
	Timer: UsesFileDescriptor<FileDescriptor=TimerFileDescriptor>,
	DatagramClientSocketInternetProtocolVersion4: UsesFileDescriptor<FileDescriptor=DatagramClientSocketInternetProtocolVersion4FileDescriptor>,
	DatagramClientSocketInternetProtocolVersion6: UsesFileDescriptor<FileDescriptor=DatagramClientSocketInternetProtocolVersion6FileDescriptor>,
	DatagramClientSocketUnixDomain: UsesFileDescriptor<FileDescriptor=DatagramClientSocketUnixDomainFileDescriptor>,
	DatagramServerListenerSocketInternetProtocolVersion4: UsesFileDescriptor<FileDescriptor=DatagramServerListenerSocketInternetProtocolVersion4FileDescriptor>,
	DatagramServerListenerSocketInternetProtocolVersion6: UsesFileDescriptor<FileDescriptor=DatagramServerListenerSocketInternetProtocolVersion6FileDescriptor>,
	DatagramServerListenerSocketUnixDomain: UsesFileDescriptor<FileDescriptor=DatagramServerListenerSocketUnixDomainFileDescriptor>,
	StreamingSocketInternetProtocolVersion4: UsesFileDescriptor<FileDescriptor=StreamingSocketInternetProtocolVersion4FileDescriptor>,
	StreamingSocketInternetProtocolVersion6: UsesFileDescriptor<FileDescriptor=StreamingSocketInternetProtocolVersion6FileDescriptor>,
	StreamingSocketUnixDomain: UsesFileDescriptor<FileDescriptor=StreamingSocketUnixDomainFileDescriptor>,
	StreamingServerListenerSocketInternetProtocolVersion4: UsesFileDescriptor<FileDescriptor=StreamingServerListenerSocketInternetProtocolVersion4FileDescriptor>,
	StreamingServerListenerSocketInternetProtocolVersion6: UsesFileDescriptor<FileDescriptor=StreamingServerListenerSocketInternetProtocolVersion6FileDescriptor>,
	StreamingServerListenerSocketUnixDomain: UsesFileDescriptor<FileDescriptor=StreamingServerListenerSocketUnixDomainFileDescriptor>,
>
{
	character_device: SimpleArena<CharacterDevice>,

	event_poll: SimpleArena<EventPoll>,

	event: SimpleArena<Event>,

	fanotify: SimpleArena<FANotify>,

	inotify: SimpleArena<INotify>,

	receive_pipe: SimpleArena<ReceivePipe>,

	send_pipe: SimpleArena<SendPipe>,

	receive_posix_message_queue: SimpleArena<ReceivePosixMessageQueue>,

	send_posix_message_queue: SimpleArena<SendPosixMessageQueue>,

	send_and_receive_posix_message_queue: SimpleArena<SendAndReceivePosixMessageQueue>,

	signal: SimpleArena<Signal>,

	terminal: SimpleArena<Terminal>,

	timer: SimpleArena<Timer>,

	datagram_client_socket_internet_protocol_version_4: SimpleArena<DatagramClientSocketInternetProtocolVersion4>,

	datagram_client_socket_internet_protocol_version_6: SimpleArena<DatagramClientSocketInternetProtocolVersion6>,

	datagram_client_socket_unix_domain: SimpleArena<DatagramClientSocketUnixDomain>,

	datagram_server_listener_socket_internet_protocol_version_4: SimpleArena<DatagramServerListenerSocketInternetProtocolVersion4>,

	datagram_server_listener_socket_internet_protocol_version_6: SimpleArena<DatagramServerListenerSocketInternetProtocolVersion6>,

	datagram_server_listener_socket_unix_domain: SimpleArena<DatagramServerListenerSocketUnixDomain>,
	
	streaming_socket_internet_protocol_version_4: SimpleArena<StreamingSocketInternetProtocolVersion4>,

	streaming_socket_internet_protocol_version_6: SimpleArena<StreamingSocketInternetProtocolVersion6>,

	streaming_socket_unix_domain: SimpleArena<StreamingSocketUnixDomain>,
	
	streaming_server_listener_socket_internet_protocol_version_4: SimpleArena<StreamingServerListenerSocketInternetProtocolVersion4>,

	streaming_server_listener_socket_internet_protocol_version_6: SimpleArena<StreamingServerListenerSocketInternetProtocolVersion6>,

	streaming_server_listener_socket_unix_domain : SimpleArena<StreamingServerListenerSocketUnixDomain>,
}

macro_rules! arena_impl
{
	($lower_case: ident, $title_case: tt, $title_case_arena: tt) =>
	{
		type $title_case = $title_case;

		type $title_case_arena = SimpleArena<Self::$title_case>;

		#[inline(always)]
		fn $lower_case(&self) -> &Self::$title_case_arena
		{
			&self.$lower_case
		}
	}
}

impl
<
	CharacterDevice: UsesFileDescriptor<FileDescriptor=CharacterDeviceFileDescriptor>,
	EventPoll: UsesFileDescriptor<FileDescriptor=EPollFileDescriptor>,
	Event: UsesFileDescriptor<FileDescriptor=EventFileDescriptor>,
	FANotify: UsesFileDescriptor<FileDescriptor=FanotifyFileDescriptor>,
	INotify: UsesFileDescriptor<FileDescriptor=InotifyFileDescriptor>,
	ReceivePipe: UsesFileDescriptor<FileDescriptor=ReceivePipeFileDescriptor>,
	SendPipe: UsesFileDescriptor<FileDescriptor=SendPipeFileDescriptor>,
	ReceivePosixMessageQueue: UsesFileDescriptor<FileDescriptor=ReceivePosixMessageQueueFileDescriptor>,
	SendPosixMessageQueue: UsesFileDescriptor<FileDescriptor=SendPosixMessageQueueFileDescriptor>,
	SendAndReceivePosixMessageQueue: UsesFileDescriptor<FileDescriptor=SendAndReceivePosixMessageQueueFileDescriptor>,
	Signal: UsesFileDescriptor<FileDescriptor=SignalFileDescriptor>,
	Terminal: UsesFileDescriptor<FileDescriptor=TerminalFileDescriptor>,
	Timer: UsesFileDescriptor<FileDescriptor=TimerFileDescriptor>,
	DatagramClientSocketInternetProtocolVersion4: UsesFileDescriptor<FileDescriptor=DatagramClientSocketInternetProtocolVersion4FileDescriptor>,
	DatagramClientSocketInternetProtocolVersion6: UsesFileDescriptor<FileDescriptor=DatagramClientSocketInternetProtocolVersion6FileDescriptor>,
	DatagramClientSocketUnixDomain: UsesFileDescriptor<FileDescriptor=DatagramClientSocketUnixDomainFileDescriptor>,
	DatagramServerListenerSocketInternetProtocolVersion4: UsesFileDescriptor<FileDescriptor=DatagramServerListenerSocketInternetProtocolVersion4FileDescriptor>,
	DatagramServerListenerSocketInternetProtocolVersion6: UsesFileDescriptor<FileDescriptor=DatagramServerListenerSocketInternetProtocolVersion6FileDescriptor>,
	DatagramServerListenerSocketUnixDomain: UsesFileDescriptor<FileDescriptor=DatagramServerListenerSocketUnixDomainFileDescriptor>,
	StreamingSocketInternetProtocolVersion4: UsesFileDescriptor<FileDescriptor=StreamingSocketInternetProtocolVersion4FileDescriptor>,
	StreamingSocketInternetProtocolVersion6: UsesFileDescriptor<FileDescriptor=StreamingSocketInternetProtocolVersion6FileDescriptor>,
	StreamingSocketUnixDomain: UsesFileDescriptor<FileDescriptor=StreamingSocketUnixDomainFileDescriptor>,
	StreamingServerListenerSocketInternetProtocolVersion4: UsesFileDescriptor<FileDescriptor=StreamingServerListenerSocketInternetProtocolVersion4FileDescriptor>,
	StreamingServerListenerSocketInternetProtocolVersion6: UsesFileDescriptor<FileDescriptor=StreamingServerListenerSocketInternetProtocolVersion6FileDescriptor>,
	StreamingServerListenerSocketUnixDomain: UsesFileDescriptor<FileDescriptor=StreamingServerListenerSocketUnixDomainFileDescriptor>,
>
	Arenas for SimpleArenas
<
	CharacterDevice,
	EventPoll,
	Event,
	FANotify,
	INotify,
	ReceivePipe,
	SendPipe,
	ReceivePosixMessageQueue,
	SendPosixMessageQueue,
	SendAndReceivePosixMessageQueue,
	Signal,
	Terminal,
	Timer,
	DatagramClientSocketInternetProtocolVersion4,
	DatagramClientSocketInternetProtocolVersion6,
	DatagramClientSocketUnixDomain,
	DatagramServerListenerSocketInternetProtocolVersion4,
	DatagramServerListenerSocketInternetProtocolVersion6,
	DatagramServerListenerSocketUnixDomain,
	StreamingSocketInternetProtocolVersion4,
	StreamingSocketInternetProtocolVersion6,
	StreamingSocketUnixDomain,
	StreamingServerListenerSocketInternetProtocolVersion4,
	StreamingServerListenerSocketInternetProtocolVersion6,
	StreamingServerListenerSocketUnixDomain,
>
{
	arena_impl!(character_device, CharacterDevice, CharacterDeviceArena);

	arena_impl!(event_poll, EventPoll, EventPollArena);

	arena_impl!(event, Event, EventArena);

	arena_impl!(fanotify, FANotify, FANotifyArena);

	arena_impl!(inotify, INotify, INotifyArena);

	arena_impl!(receive_pipe, ReceivePipe, ReceivePipeArena);

	arena_impl!(send_pipe, SendPipe, SendPipeArena);

	arena_impl!(receive_posix_message_queue, ReceivePosixMessageQueue, ReceivePosixMessageQueueArena);

	arena_impl!(send_posix_message_queue, SendPosixMessageQueue, SendPosixMessageQueueArena);

	arena_impl!(send_and_receive_posix_message_queue, SendAndReceivePosixMessageQueue, SendAndReceivePosixMessageQueueArena);

	arena_impl!(signal, Signal, SignalArena);

	arena_impl!(terminal, Terminal, TerminalArena);

	arena_impl!(timer, Timer, TimerArena);

	arena_impl!(datagram_client_socket_internet_protocol_version_4, DatagramClientSocketInternetProtocolVersion4, DatagramClientSocketInternetProtocolVersion4Arena);

	arena_impl!(datagram_client_socket_internet_protocol_version_6, DatagramClientSocketInternetProtocolVersion6, DatagramClientSocketInternetProtocolVersion6Arena);

	arena_impl!(datagram_client_socket_unix_domain, DatagramClientSocketUnixDomain, DatagramClientSocketUnixDomainArena);

	arena_impl!(datagram_server_listener_socket_internet_protocol_version_4, DatagramServerListenerSocketInternetProtocolVersion4, DatagramServerListenerSocketInternetProtocolVersion4Arena);

	arena_impl!(datagram_server_listener_socket_internet_protocol_version_6, DatagramServerListenerSocketInternetProtocolVersion6, DatagramServerListenerSocketInternetProtocolVersion6Arena);

	arena_impl!(datagram_server_listener_socket_unix_domain, DatagramServerListenerSocketUnixDomain, DatagramServerListenerSocketUnixDomainArena);

	arena_impl!(streaming_socket_internet_protocol_version_4, StreamingSocketInternetProtocolVersion4, StreamingSocketInternetProtocolVersion4Arena);

	arena_impl!(streaming_socket_internet_protocol_version_6, StreamingSocketInternetProtocolVersion6, StreamingSocketInternetProtocolVersion6Arena);

	arena_impl!(streaming_socket_unix_domain, StreamingSocketUnixDomain, StreamingSocketUnixDomainArena);

	arena_impl!(streaming_server_listener_socket_internet_protocol_version_4, StreamingServerListenerSocketInternetProtocolVersion4, StreamingServerListenerSocketInternetProtocolVersion4Arena);

	arena_impl!(streaming_server_listener_socket_internet_protocol_version_6, StreamingServerListenerSocketInternetProtocolVersion6, StreamingServerListenerSocketInternetProtocolVersion6Arena);

	arena_impl!(streaming_server_listener_socket_unix_domain, StreamingServerListenerSocketUnixDomain, StreamingServerListenerSocketUnixDomainArena);
}

impl
<
	CharacterDevice: UsesFileDescriptor<FileDescriptor=CharacterDeviceFileDescriptor>,
	EventPoll: UsesFileDescriptor<FileDescriptor=EPollFileDescriptor>,
	Event: UsesFileDescriptor<FileDescriptor=EventFileDescriptor>,
	FANotify: UsesFileDescriptor<FileDescriptor=FanotifyFileDescriptor>,
	INotify: UsesFileDescriptor<FileDescriptor=InotifyFileDescriptor>,
	ReceivePipe: UsesFileDescriptor<FileDescriptor=ReceivePipeFileDescriptor>,
	SendPipe: UsesFileDescriptor<FileDescriptor=SendPipeFileDescriptor>,
	ReceivePosixMessageQueue: UsesFileDescriptor<FileDescriptor=ReceivePosixMessageQueueFileDescriptor>,
	SendPosixMessageQueue: UsesFileDescriptor<FileDescriptor=SendPosixMessageQueueFileDescriptor>,
	SendAndReceivePosixMessageQueue: UsesFileDescriptor<FileDescriptor=SendAndReceivePosixMessageQueueFileDescriptor>,
	Signal: UsesFileDescriptor<FileDescriptor=SignalFileDescriptor>,
	Terminal: UsesFileDescriptor<FileDescriptor=TerminalFileDescriptor>,
	Timer: UsesFileDescriptor<FileDescriptor=TimerFileDescriptor>,
	DatagramClientSocketInternetProtocolVersion4: UsesFileDescriptor<FileDescriptor=DatagramClientSocketInternetProtocolVersion4FileDescriptor>,
	DatagramClientSocketInternetProtocolVersion6: UsesFileDescriptor<FileDescriptor=DatagramClientSocketInternetProtocolVersion6FileDescriptor>,
	DatagramClientSocketUnixDomain: UsesFileDescriptor<FileDescriptor=DatagramClientSocketUnixDomainFileDescriptor>,
	DatagramServerListenerSocketInternetProtocolVersion4: UsesFileDescriptor<FileDescriptor=DatagramServerListenerSocketInternetProtocolVersion4FileDescriptor>,
	DatagramServerListenerSocketInternetProtocolVersion6: UsesFileDescriptor<FileDescriptor=DatagramServerListenerSocketInternetProtocolVersion6FileDescriptor>,
	DatagramServerListenerSocketUnixDomain: UsesFileDescriptor<FileDescriptor=DatagramServerListenerSocketUnixDomainFileDescriptor>,
	StreamingSocketInternetProtocolVersion4: UsesFileDescriptor<FileDescriptor=StreamingSocketInternetProtocolVersion4FileDescriptor>,
	StreamingSocketInternetProtocolVersion6: UsesFileDescriptor<FileDescriptor=StreamingSocketInternetProtocolVersion6FileDescriptor>,
	StreamingSocketUnixDomain: UsesFileDescriptor<FileDescriptor=StreamingSocketUnixDomainFileDescriptor>,
	StreamingServerListenerSocketInternetProtocolVersion4: UsesFileDescriptor<FileDescriptor=StreamingServerListenerSocketInternetProtocolVersion4FileDescriptor>,
	StreamingServerListenerSocketInternetProtocolVersion6: UsesFileDescriptor<FileDescriptor=StreamingServerListenerSocketInternetProtocolVersion6FileDescriptor>,
	StreamingServerListenerSocketUnixDomain: UsesFileDescriptor<FileDescriptor=StreamingServerListenerSocketUnixDomainFileDescriptor>,
>
	SimpleArenas
<
	CharacterDevice,
	EventPoll,
	Event,
	FANotify,
	INotify,
	ReceivePipe,
	SendPipe,
	ReceivePosixMessageQueue,
	SendPosixMessageQueue,
	SendAndReceivePosixMessageQueue,
	Signal,
	Terminal,
	Timer,
	DatagramClientSocketInternetProtocolVersion4,
	DatagramClientSocketInternetProtocolVersion6,
	DatagramClientSocketUnixDomain,
	DatagramServerListenerSocketInternetProtocolVersion4,
	DatagramServerListenerSocketInternetProtocolVersion6,
	DatagramServerListenerSocketUnixDomain,
	StreamingSocketInternetProtocolVersion4,
	StreamingSocketInternetProtocolVersion6,
	StreamingSocketUnixDomain,
	StreamingServerListenerSocketInternetProtocolVersion4,
	StreamingServerListenerSocketInternetProtocolVersion6,
	StreamingServerListenerSocketUnixDomain,
>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new
	(
		character_device: SimpleArena<CharacterDevice>,
		event_poll: SimpleArena<EventPoll>,
		event: SimpleArena<Event>,
		fanotify: SimpleArena<FANotify>,
		inotify: SimpleArena<INotify>,
		receive_pipe: SimpleArena<ReceivePipe>,
		send_pipe: SimpleArena<SendPipe>,
		receive_posix_message_queue: SimpleArena<ReceivePosixMessageQueue>,
		send_posix_message_queue: SimpleArena<SendPosixMessageQueue>,
		send_and_receive_posix_message_queue: SimpleArena<SendAndReceivePosixMessageQueue>,
		signal: SimpleArena<Signal>,
		terminal: SimpleArena<Terminal>,
		timer: SimpleArena<Timer>,
		datagram_client_socket_internet_protocol_version_4: SimpleArena<DatagramClientSocketInternetProtocolVersion4>,
		datagram_client_socket_internet_protocol_version_6: SimpleArena<DatagramClientSocketInternetProtocolVersion6>,
		datagram_client_socket_unix_domain: SimpleArena<DatagramClientSocketUnixDomain>,
		datagram_server_listener_socket_internet_protocol_version_4: SimpleArena<DatagramServerListenerSocketInternetProtocolVersion4>,
		datagram_server_listener_socket_internet_protocol_version_6: SimpleArena<DatagramServerListenerSocketInternetProtocolVersion6>,
		datagram_server_listener_socket_unix_domain: SimpleArena<DatagramServerListenerSocketUnixDomain>,
		streaming_socket_internet_protocol_version_4: SimpleArena<StreamingSocketInternetProtocolVersion4>,
		streaming_socket_internet_protocol_version_6: SimpleArena<StreamingSocketInternetProtocolVersion6>,
		streaming_socket_unix_domain: SimpleArena<StreamingSocketUnixDomain>,
		streaming_server_listener_socket_internet_protocol_version_4: SimpleArena<StreamingServerListenerSocketInternetProtocolVersion4>,
		streaming_server_listener_socket_internet_protocol_version_6: SimpleArena<StreamingServerListenerSocketInternetProtocolVersion6>,
		streaming_server_listener_socket_unix_domain: SimpleArena<StreamingServerListenerSocketUnixDomain>,
	) -> Self
	{
		Self
		{
			character_device,
			event_poll,
			event,
			fanotify,
			inotify,
			receive_pipe,
			send_pipe,
			receive_posix_message_queue,
			send_posix_message_queue,
			send_and_receive_posix_message_queue,
			signal,
			terminal,
			timer,
			datagram_client_socket_internet_protocol_version_4,
			datagram_client_socket_internet_protocol_version_6,
			datagram_client_socket_unix_domain,
			datagram_server_listener_socket_internet_protocol_version_4,
			datagram_server_listener_socket_internet_protocol_version_6,
			datagram_server_listener_socket_unix_domain,
			streaming_socket_internet_protocol_version_4,
			streaming_socket_internet_protocol_version_6,
			streaming_socket_unix_domain,
			streaming_server_listener_socket_internet_protocol_version_4,
			streaming_server_listener_socket_internet_protocol_version_6,
			streaming_server_listener_socket_unix_domain,
		}
	}
}

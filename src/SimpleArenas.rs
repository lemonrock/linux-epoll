// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.

/// Holds an arena for each item of `FileDescriptorKind`.
#[derive(Debug)]
pub struct SimpleArenas<CharacterDevice: UsesFileDescriptor, EventPoll: UsesFileDescriptor, Event: UsesFileDescriptor, FANotify: UsesFileDescriptor, INotify: UsesFileDescriptor, ReceivePipe: UsesFileDescriptor, SendPipe: UsesFileDescriptor, ReceivePosixMessageQueue: UsesFileDescriptor, SendPosixMessageQueue: UsesFileDescriptor, SendAndReceivePosixMessageQueue: UsesFileDescriptor, Signal: UsesFileDescriptor, Terminal: UsesFileDescriptor, Timer: UsesFileDescriptor, DatagramClientSocket: UsesFileDescriptor, DatagramServerListenerSocket: UsesFileDescriptor, StreamingSocket: UsesFileDescriptor, StreamingServerListenerSocket: UsesFileDescriptor>
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




	datagram_server_listener_socket: SimpleArena<DatagramServerListenerSocket>,

	streaming_socket: SimpleArena<StreamingSocket>,

	streaming_server_listener_socket: SimpleArena<StreamingServerListenerSocket>,
}

impl<CharacterDevice: UsesFileDescriptor, EventPoll: UsesFileDescriptor, Event: UsesFileDescriptor, FANotify: UsesFileDescriptor, INotify: UsesFileDescriptor, ReceivePipe: UsesFileDescriptor, SendPipe: UsesFileDescriptor, ReceivePosixMessageQueue: UsesFileDescriptor, SendPosixMessageQueue: UsesFileDescriptor, SendAndReceivePosixMessageQueue: UsesFileDescriptor, Signal: UsesFileDescriptor, Terminal: UsesFileDescriptor, Timer: UsesFileDescriptor, DatagramClientSocket: UsesFileDescriptor, DatagramServerListenerSocket: UsesFileDescriptor, StreamingSocket: UsesFileDescriptor, StreamingServerListenerSocket: UsesFileDescriptor> Arenas for SimpleArenas<CharacterDevice, EventPoll, Event, FANotify, INotify, ReceivePipe, SendPipe, ReceivePosixMessageQueue, SendPosixMessageQueue, SendAndReceivePosixMessageQueue, Signal, Terminal, Timer, DatagramClientSocket, DatagramServerListenerSocket, StreamingSocket, StreamingServerListenerSocket>
{
	type CharacterDevice = CharacterDevice;
	
	type CharacterDeviceArena = SimpleArena<CharacterDevice>;

	#[inline(always)]
	fn character_device(&self) -> &Self::CharacterDeviceArena
	{
		&self.character_device
	}

	type EventPoll = EventPoll;

	type EventPollArena = SimpleArena<Self::EventPoll>;

	#[inline(always)]
	fn event_poll(&self) -> &Self::EventPollArena
	{
		&self.event_poll
	}

	type Event = Event;

	type EventArena = SimpleArena<Self::Event>;

	#[inline(always)]
	fn event(&self) -> &Self::EventArena
	{
		&self.event
	}

	type FANotify = FANotify;

	type FANotifyArena = SimpleArena<Self::FANotify>;

	#[inline(always)]
	fn fanotify(&self) -> &Self::FANotifyArena
	{
		&self.fanotify
	}

	type INotify = INotify;

	type INotifyArena = SimpleArena<Self::INotify>;

	#[inline(always)]
	fn inotify(&self) -> &Self::INotifyArena
	{
		&self.inotify
	}

	type ReceivePipe = ReceivePipe;

	type ReceivePipeArena = SimpleArena<Self::ReceivePipe>;

	#[inline(always)]
	fn receive_pipe(&self) -> &Self::ReceivePipeArena
	{
		&self.receive_pipe
	}

	type SendPipe = SendPipe;

	type SendPipeArena = SimpleArena<Self::SendPipe>;

	#[inline(always)]
	fn send_pipe(&self) -> &Self::SendPipeArena
	{
		&self.send_pipe
	}

	type ReceivePosixMessageQueue = ReceivePosixMessageQueue;

	type ReceivePosixMessageQueueArena = SimpleArena<Self::ReceivePosixMessageQueue>;

	#[inline(always)]
	fn receive_posix_message_queue(&self) -> &Self::ReceivePosixMessageQueueArena
	{
		&self.receive_posix_message_queue
	}

	type SendPosixMessageQueue = SendPosixMessageQueue;

	type SendPosixMessageQueueArena = SimpleArena<Self::SendPosixMessageQueue>;

	#[inline(always)]
	fn send_posix_message_queue(&self) -> &Self::SendPosixMessageQueueArena
	{
		&self.send_posix_message_queue
	}

	type SendAndReceivePosixMessageQueue = SendAndReceivePosixMessageQueue;

	type SendAndReceivePosixMessageQueueArena = SimpleArena<Self::SendAndReceivePosixMessageQueue>;

	#[inline(always)]
	fn send_and_receive_posix_message_queue(&self) -> &Self::SendAndReceivePosixMessageQueueArena
	{
		&self.send_and_receive_posix_message_queue
	}

	type Signal = Signal;

	type SignalArena = SimpleArena<Self::Signal>;

	#[inline(always)]
	fn signal(&self) -> &Self::SignalArena
	{
		&self.signal
	}

	type Terminal = Terminal;

	type TerminalArena = SimpleArena<Self::Terminal>;

	#[inline(always)]
	fn terminal(&self) -> &Self::TerminalArena
	{
		&self.terminal
	}

	type Timer = Timer;

	type TimerArena = SimpleArena<Self::Timer>;

	#[inline(always)]
	fn timer(&self) -> &Self::TimerArena
	{
		&self.timer
	}

	type DatagramClientSocket = DatagramClientSocket;

	type DatagramClientSocketArena = SimpleArena<Self::DatagramClientSocket>;

	#[inline(always)]
	fn datagram_client_socket(&self) -> &Self::DatagramClientSocketArena
	{
		&self.datagram_client_socket
	}

	type DatagramServerListenerSocket = DatagramServerListenerSocket;

	type DatagramServerListenerSocketArena = SimpleArena<Self::DatagramServerListenerSocket>;

	#[inline(always)]
	fn datagram_server_listener_socket(&self) -> &Self::DatagramServerListenerSocketArena
	{
		&self.datagram_server_listener_socket
	}

	type StreamingSocket = StreamingSocket;

	type StreamingSocketArena = SimpleArena<Self::StreamingSocket>;

	#[inline(always)]
	fn streaming_socket(&self) -> &Self::StreamingSocketArena
	{
		&self.streaming_socket
	}

	type StreamingServerListenerSocket = StreamingServerListenerSocket;

	type StreamingServerListenerSocketArena = SimpleArena<Self::StreamingServerListenerSocket>;

	#[inline(always)]
	fn streaming_server_listener_socket(&self) -> &Self::StreamingServerListenerSocketArena
	{
		&self.streaming_server_listener_socket
	}
}

impl<CharacterDevice: UsesFileDescriptor, EventPoll: UsesFileDescriptor, Event: UsesFileDescriptor, FANotify: UsesFileDescriptor, INotify: UsesFileDescriptor, ReceivePipe: UsesFileDescriptor, SendPipe: UsesFileDescriptor, ReceivePosixMessageQueue: UsesFileDescriptor, SendPosixMessageQueue: UsesFileDescriptor, SendAndReceivePosixMessageQueue: UsesFileDescriptor, Signal: UsesFileDescriptor, Terminal: UsesFileDescriptor, Timer: UsesFileDescriptor, DatagramClientSocket: UsesFileDescriptor, DatagramServerListenerSocket: UsesFileDescriptor, StreamingSocket: UsesFileDescriptor, StreamingServerListenerSocket: UsesFileDescriptor> SimpleArenas<CharacterDevice, EventPoll, Event, FANotify, INotify, ReceivePipe, SendPipe, ReceivePosixMessageQueue, SendPosixMessageQueue, SendAndReceivePosixMessageQueue, Signal, Terminal, Timer, DatagramClientSocket, DatagramServerListenerSocket, StreamingSocket, StreamingServerListenerSocket>
{
	/// Creates a new instance.
	#[inline(always)]
	pub const fn new
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
		datagram_client_socket: SimpleArena<DatagramClientSocket>,
		datagram_server_listener_socket: SimpleArena<DatagramServerListenerSocket>,
		streaming_socket: SimpleArena<StreamingSocket>,
		streaming_server_listener_socket: SimpleArena<StreamingServerListenerSocket>
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
			datagram_client_socket,
			datagram_server_listener_socket,
			streaming_socket,
			streaming_server_listener_socket,
		}
	}
}

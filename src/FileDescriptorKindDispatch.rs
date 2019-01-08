// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


macro_rules! file_descriptor_kind_dispatch_function
{
	($lower_case: ident, $title_case: ident, $file_descriptor: ty) =>
	{
		/// Invoked when `FileDescriptorKind` is `FileDescriptorKind::$title_case`.
		///
		/// Returns a tuple of `(close_file_descriptor, R)`.
		fn $lower_case<$title_case>(&self, file_descriptor: &$file_descriptor, instance: &mut $title_case, arguments: Self::Arguments) -> (bool, Self::R);
	}
}

/// Used to store different kinds of file descriptors without usind dynamic dispatch.
pub trait FileDescriptorKindDispatch
{
	/// Result type.
	type R;

	/// Arguments to pass.
	type Arguments;

	file_descriptor_kind_dispatch_function!(character_device, CharacterDevice, CharacterDeviceFileDescriptor);

	file_descriptor_kind_dispatch_function!(event_poll, EventPoll, EPollFileDescriptor);

	file_descriptor_kind_dispatch_function!(event, Event, EventFileDescriptor);

	file_descriptor_kind_dispatch_function!(fanotify, FANotify, FanotifyFileDescriptor);

	file_descriptor_kind_dispatch_function!(inotify, Inotify, InotifyFileDescriptor);

	file_descriptor_kind_dispatch_function!(receive_pipe, ReceivePipe, ReceivePipeFileDescriptor);

	file_descriptor_kind_dispatch_function!(send_pipe, SendPipe, SendPipeFileDescriptor);

	file_descriptor_kind_dispatch_function!(receive_posix_message_queue, ReceivePosixMessageQueue, ReceivePosixMessageQueueFileDescriptor);

	file_descriptor_kind_dispatch_function!(send_posix_message_queue, SendPosixMessageQueue, SendPosixMessageQueueFileDescriptor);

	file_descriptor_kind_dispatch_function!(send_and_posix_message_queue, SendAndReceivePosixMessageQueue, SendAndReceivePosixMessageQueueFileDescriptor);

	file_descriptor_kind_dispatch_function!(signal, Signal, SignalFileDescriptor);

	file_descriptor_kind_dispatch_function!(terminal, Terminal, TerminalFileDescriptor);

	file_descriptor_kind_dispatch_function!(timer, Timer, TimerFileDescriptor);

	file_descriptor_kind_dispatch_function!(datagram_client_socket_internet_protocol_version_4, DatagramClientSocketInternetProtocolVersion4, DatagramClientSocketFileDescriptor<sockaddr_in>);

	file_descriptor_kind_dispatch_function!(datagram_client_socket_internet_protocol_version_6, DatagramClientSocketInternetProtocolVersion6, DatagramClientSocketFileDescriptor<sockaddr_in6>);

	file_descriptor_kind_dispatch_function!(datagram_client_socket_internet_protocol_unix_domain, DatagramClientSocketUnixDomain, DatagramClientSocketFileDescriptor<sockaddr_un>);

	file_descriptor_kind_dispatch_function!(datagram_server_listener_socket_internet_protocol_version_4, DatagramServerListenerSocketInternetProtocolVersion4, DatagramServerListenerSocketFileDescriptor<sockaddr_in>);

	file_descriptor_kind_dispatch_function!(datagram_server_listener_socket_internet_protocol_version_6, DatagramServerListenerSocketInternetProtocolVersion6, DatagramServerListenerSocketFileDescriptor<sockaddr_in6>);

	file_descriptor_kind_dispatch_function!(datagram_server_listener_socket_internet_protocol_unix_domain, DatagramServerListenerSocketUnixDomain, DatagramServerListenerSocketFileDescriptor<sockaddr_un>);

	file_descriptor_kind_dispatch_function!(streaming_socket_internet_protocol_version_4, StreamingSocketInternetProtocolVersion4, StreamingSocketFileDescriptor<sockaddr_in>);

	file_descriptor_kind_dispatch_function!(streaming_socket_internet_protocol_version_6, StreamingSocketInternetProtocolVersion6, StreamingSocketFileDescriptor<sockaddr_in6>);

	file_descriptor_kind_dispatch_function!(streaming_socket_internet_protocol_unix_domain, StreamingSocketUnixDomain, StreamingSocketFileDescriptor<sockaddr_un>);

	file_descriptor_kind_dispatch_function!(streaming_server_listener_socket_internet_protocol_version_4, StreamingServerListenerSocketInternetProtocolVersion4, StreamingServerListenerSocketFileDescriptor<sockaddr_in>);

	file_descriptor_kind_dispatch_function!(streaming_server_listener_socket_internet_protocol_version_6, StreamingServerListenerSocketInternetProtocolVersion6, StreamingServerListenerSocketFileDescriptor<sockaddr_in6>);

	file_descriptor_kind_dispatch_function!(streaming_server_listener_socket_internet_protocol_unix_domain, StreamingServerListenerSocketUnixDomain, StreamingServerListenerSocketFileDescriptor<sockaddr_un>);
}

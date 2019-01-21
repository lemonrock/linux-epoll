// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


macro_rules! file_descriptor_kind_dispatch
{
	(@ $arenas: expr, $event_poll_token: ident, $raw_file_descriptor: ident, $dispatch: ident, $dispatch_arguments: expr, $($title_case: ident => $lower_case: ident,)*) =>
	{
		{
			let arena_index = FileDescriptorKind::arena_index($event_poll_token);

			use self::FileDescriptorKind::*;

			match FileDescriptorKind::file_descriptor_kind($event_poll_token)
			{
				$(
					$title_case =>
					{
						let arena = $arenas.$lower_case();

						let (reactor, file_descriptor) = arena.get(arena_index, $raw_file_descriptor);

						$dispatch($dispatch_arguments, arena, arena_index, reactor, file_descriptor)
					}
				)*
			}
		}
	};

	// $dispatch is invoked as `$dispatch($dispatch_arguments, &arena, arena_index, &mut reactor, file_descriptor)`.
	($arenas: expr, $event_poll_token: ident, $raw_file_descriptor: ident, $dispatch: ident, $dispatch_arguments: expr) =>
	{
		file_descriptor_kind_dispatch!
		(
			@ $arenas, $event_poll_token, $raw_file_descriptor, $dispatch, $dispatch_arguments,

			CharacterDevice => character_device,

			EventPoll => event_poll,

			Event => event,

			FANotify => fanotify,

			INotify => inotify,

			ReceivePipe => receive_pipe,

			SendPipe => send_pipe,

			ReceivePosixMessageQueue => receive_posix_message_queue,

			SendPosixMessageQueue => send_posix_message_queue,

			SendAndReceivePosixMessageQueue => send_and_receive_posix_message_queue,

			Signal => signal,

			Terminal => terminal,

			Timer => timer,

			DatagramClientSocketInternetProtocolVersion4 => datagram_client_socket_internet_protocol_version_4,

			DatagramClientSocketInternetProtocolVersion6 => datagram_client_socket_internet_protocol_version_6,

			DatagramClientSocketUnixDomain => datagram_client_socket_unix_domain,

			DatagramServerListenerSocketInternetProtocolVersion4 => datagram_server_listener_socket_internet_protocol_version_4,

			DatagramServerListenerSocketInternetProtocolVersion6 => datagram_server_listener_socket_internet_protocol_version_6,

			DatagramServerListenerSocketUnixDomain => datagram_server_listener_socket_unix_domain,

			StreamingSocketInternetProtocolVersion4 => streaming_socket_internet_protocol_version_4,

			StreamingSocketInternetProtocolVersion6 => streaming_socket_internet_protocol_version_6,

			StreamingSocketUnixDomain => streaming_socket_unix_domain,

			StreamingServerListenerSocketInternetProtocolVersion4 => streaming_server_listener_socket_internet_protocol_version_4,

			StreamingServerListenerSocketInternetProtocolVersion6 => streaming_server_listener_socket_internet_protocol_version_6,

			StreamingServerListenerSocketUnixDomain => streaming_server_listener_socket_unix_domain,
		)
	};
}

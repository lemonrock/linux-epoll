// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Used to distinguish the kind of file descriptor stored in an epoll token.
///
/// Currently restricted to a maximum of 5 bits.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum FileDescriptorKind
{
	CharacterDevice = 0,

	EventPoll = 1,

	Event = 2,

	FANotify = 3,

	INotify = 4,

	ReceivePipe = 5,

	SendPipe = 6,

	ReceivePosixMessageQueue = 7,

	SendPosixMessageQueue = 8,

	SendAndReceivePosixMessageQueue = 9,

	Signal = 10,

	Terminal = 11,

	Timer = 12,

	DatagramClientSocketInternetProtocolVersion4 = 13,

	DatagramClientSocketInternetProtocolVersion6 = 14,

	DatagramClientSocketUnixDomain = 15,

	DatagramServerListenerSocketInternetProtocolVersion4 = 16,

	DatagramServerListenerSocketInternetProtocolVersion6 = 17,

	DatagramServerListenerSocketUnixDomain = 18,

	StreamingSocketInternetProtocolVersion4 = 19,

	StreamingSocketInternetProtocolVersion6 = 20,

	StreamingSocketUnixDomain = 21,

	StreamingServerListenerSocketInternetProtocolVersion4 = 22,

	StreamingServerListenerSocketInternetProtocolVersion6 = 23,

	StreamingServerListenerSocketUnixDomain = 24,
}

impl FileDescriptorKind
{
	const EventPollTokenSize: u64 = Self::size_in_bits::<u64>();

	const FileDescriptorKindSize: u64 = 5;

	const FileDescriptorKindShift: u64 = Self::EventPollTokenSize - Self::FileDescriptorKindSize;

	const FileDescriptorKindMask: u64 = Self::bitmask(Self::FileDescriptorKindSize, Self::FileDescriptorKindShift);

	const RawFileDescriptorSize: u64 = Self::size_in_bits::<RawFd>();

	const RawFileDescriptorShift: u64 = Self::FileDescriptorKindShift - Self::RawFileDescriptorSize;

	const RawFileDescriptorMask: u64 = Self::bitmask(Self::RawFileDescriptorSize, Self::RawFileDescriptorShift);

	const ArenaIndexMask: u64 = !(Self::FileDescriptorKindMask | Self::RawFileDescriptorMask);

	#[inline(always)]
	const fn size_in_bits<T: Sized>() -> u64
	{
		(size_of::<T>() as u64) * 8
	}

	#[inline(always)]
	const fn bitmask(number_of_bits: u64, shift: u64) -> u64
	{
		#[inline(always)]
		const fn set_bits(number_of_bits: u64) -> u64
		{
			(1 << number_of_bits) - 1
		}

		set_bits(number_of_bits) << shift
	}

	/// Extracts the file descriptor kind from an event poll token.
	#[inline(always)]
	pub(crate) fn file_descriptor_kind(event_poll_token: EventPollToken) -> Self
	{
		let value = ((event_poll_token & Self::FileDescriptorKindMask) >> Self::FileDescriptorKindShift) as u8;
		unsafe { transmute(value) }
	}

	/// Extracts the file descriptor kind from an event poll token.
	#[inline(always)]
	pub(crate) fn raw_file_descriptor(event_poll_token: EventPollToken) -> RawFd
	{
		((event_poll_token & Self::RawFileDescriptorMask) >> Self::RawFileDescriptorShift) as RawFd
	}

	/// Extracts the arena index from an event poll token.
	#[inline(always)]
	pub(crate) fn arena_index(event_poll_token: EventPollToken) -> ArenaIndex
	{
		(event_poll_token & Self::ArenaIndexMask) as ArenaIndex
	}

	/// Extracts the file descriptor kind from an event poll token.
	#[inline(always)]
	pub(crate) fn new_event_poll_token(self, raw_file_descriptor: &impl AsRawFd, arena_index: ArenaIndex) -> EventPollToken
	{
		EventPollToken(((self as u8 as u64) << Self::FileDescriptorKindShift) | ((raw_file_descriptor.as_raw_fd() as u64) << Self::RawFileDescriptorShift) | (arena_index as u64))
	}

	/// Result is an error if the associated file descriptor has already been closed; this can happen due to spurious epoll events (eg receiving read and write as separate events).
	#[inline(always)]
	pub fn react<A: Arenas>(event_poll_token: EventPollToken, spurious_event_suppression_of_already_closed_file_descriptors: &mut HashSet<RawFd>, arenas: &A, event_flags: EPollEventFlags, terminate: &impl Terminate) -> Result<(), String>
	{
		macro_rules! dispatch
		{
			($event_poll_token: ident, $spurious_event_suppression_of_already_closed_file_descriptors: ident, $arenas: ident, $event_flags: ident, $terminate: ident, $($title_case: tt => $lower_case: tt,)*) =>
			{
				{
					let raw_file_descriptor = Self::raw_file_descriptor($event_poll_token);
					if $spurious_event_suppression_of_already_closed_file_descriptors.contains(&raw_file_descriptor)
					{
						return Ok(())
					}

					let arena_index = Self::arena_index($event_poll_token);

					use self::FileDescriptorKind::*;

					match Self::file_descriptor_kind($event_poll_token)
					{
						$(
							$title_case =>
							{
								let arena = $arenas.$lower_case();

								let (instance, file_descriptor) = arena.get(arena_index, raw_file_descriptor);

								match instance.react(&file_descriptor, $event_flags, $terminate)
								{
									Err(reason) => Err(reason),
									Ok(close_file_descriptor) =>
									{
										if unlikely!(close_file_descriptor)
										{
											drop(file_descriptor);
											let first_insertion = $spurious_event_suppression_of_already_closed_file_descriptors.insert(raw_file_descriptor);
											debug_assert!(first_insertion, "Spurious event somehow not captured and double-close of file descriptor occurred");
											arena.reclaim(arena_index);
										}
										else
										{
											forget(file_descriptor);
										}
										Ok(())
									}
								}
							}
						)*
					}
				}
			}
		}

		dispatch!
		{
			event_poll_token, spurious_event_suppression_of_already_closed_file_descriptors, arenas, event_flags, terminate,

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
		}
	}
}

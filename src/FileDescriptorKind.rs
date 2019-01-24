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
	/// Result is an error if the associated file descriptor has already been closed; this can happen due to spurious epoll events (eg receiving read and write as separate events).
	#[inline(always)]
	pub fn react<AS: Arenas>(event_poll: &EventPoll<AS>, event_poll_token: EventPollToken, spurious_event_suppression_of_already_closed_file_descriptors: &mut HashSet<RawFd>, arenas: &AS, event_flags: EPollEventFlags, terminate: &impl Terminate) -> Result<(), String>
	{
		#[inline(always)]
		fn dispatch<AS: Arenas, R: Reactor<AS, A>, A: Arena<R, AS>>((event_poll, spurious_event_suppression_of_already_closed_file_descriptors, event_flags, terminate): (&EventPoll<AS>, &mut HashSet<RawFd>, EPollEventFlags, &impl Terminate), arena: &A, arena_index: ArenaIndex, reactor: &mut R, file_descriptor: R::FileDescriptor) -> Result<(), String>
		{
			match reactor.react(event_poll, &file_descriptor, event_flags, terminate)
			{
				Err(reason) => Err(reason),
				Ok(close_file_descriptor) =>
				{
					if unlikely!(close_file_descriptor)
					{
						let first_insertion = spurious_event_suppression_of_already_closed_file_descriptors.insert(file_descriptor.as_raw_fd());
						debug_assert!(first_insertion, "Spurious event somehow not captured and double-close of file descriptor occurred");

						event_poll.deregister_and_close(file_descriptor);
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

		let raw_file_descriptor = event_poll_token.raw_file_descriptor();

		file_descriptor_kind_dispatch!(arenas, event_poll_token, raw_file_descriptor, dispatch, (event_poll, spurious_event_suppression_of_already_closed_file_descriptors, event_flags, terminate))
	}
}

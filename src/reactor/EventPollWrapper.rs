// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// ?
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventPollWrapper<A: Arenas>
{
	epoll_file_descriptor: EPollFileDescriptor,
	arenas: A,
}

impl<A: Arenas> EventPollWrapper<A>
{
	/// Creates a new instance.
	///
	/// Only one instance per thread is normally required.
	#[inline(always)]
	pub fn new(arenas: A) -> Result<Self, CreationError>
	{
		Ok
		(
			Self
			{
				epoll_file_descriptor: EPollFileDescriptor::new()?,
				arenas,
			}
		)
	}

	/// Register a new file descriptor and allocate space for its management data in an arena.
	#[inline(always)]
	pub fn register<R: Reactor>(&self, arena: &impl Arena<R>, file_descriptor: R::FileDescriptor, initializer: impl FnOnce(&mut R) -> Result<(), EventPollRegistrationError>) -> Result<(), EventPollRegistrationError>
	{
		let (mut non_null, arena_index) = arena.allocate()?;
		let event_poll_token = R::FileDescriptorKind.new_event_poll_token(&file_descriptor, arena_index);

		match self.epoll_file_descriptor.add(file_descriptor.as_raw_fd(), EPollAddFlags::EdgeTriggeredInput, event_poll_token.0)
		{
			Err(error) =>
			{
				arena.reclaim(arena_index);
				Err(EventPollRegistrationError::from(error))
			}

			Ok(()) =>
			{
				forget(file_descriptor);
				initializer(unsafe { non_null.as_mut() })
			}
		}
	}

	/// Event loop; loops until terminate is set or a serious error occurs.
	#[inline(always)]
	pub fn event_loop<'a>(&self, terminate: &impl Terminate, time_out_milliseconds: u16) -> Result<(), String>
	{
		const MaximumEvents: usize = 1024;

		let time_out = EPollTimeOut::in_n_milliseconds(time_out_milliseconds);
		let mut events: [epoll_event; MaximumEvents] = unsafe { uninitialized() };
		let mut spurious_event_suppression_of_already_closed_file_descriptors = HashSet::with_capacity(MaximumEvents);

		while terminate.should_continue()
		{
			let ready_events = match self.epoll_file_descriptor.wait(&mut events, time_out)
			{
				Ok(ready_events) => ready_events,

				Err(EPollWaitError::Interrupted) => continue,
			};

			for ready_event in ready_events
			{
				let event_poll_token = EventPollToken(ready_event.token());
				let result = FileDescriptorKind::react(event_poll_token, &mut spurious_event_suppression_of_already_closed_file_descriptors, &self.arenas, ready_event.flags(), terminate);
				if let Err(reason) = result
				{
					terminate.begin_termination();
					return Err(reason)
				}
			}

			spurious_event_suppression_of_already_closed_file_descriptors.clear();
		}

		Ok(())
	}
}

// TODO Share a file descriptor across threads
// SO_REUSEPORT with SO_INCOMING_CPU
// EPOLLEXCLUSIVE
// setsockopt(http->fd, SOL_SOCKET, SO_REUSEPORT, &val, sizeof(val));

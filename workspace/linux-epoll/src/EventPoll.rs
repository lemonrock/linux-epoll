// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


const MaximumEvents: usize = 1024;

/// Wraps event polling.
#[derive(Debug)]
pub(crate) struct EventPoll
{
	arenas: Arenas,
	epoll_file_descriptor: EPollFileDescriptor,
	time_out: EPollTimeOut,
	spurious_event_suppression_of_already_closed_file_descriptors: UnsafeCell<HashSet<EventPollToken>>,
}

impl ArenasRegistrar for EventPoll
{
	#[inline(always)]
	fn register_arena<A: Arena<R> + 'static, R: Reactor + 'static, T: Terminate>(&mut self, arena: A) -> CompressedTypeIdentifier
	{
		self.arenas.register_arena::<A, R, T>(arena)
	}
}

impl ReactorsRegistrar for EventPoll
{
	#[inline(always)]
	unsafe fn add_a_new_reactor_efficiently<A: Arena<R> + 'static, R: Reactor + 'static>(&self, reactor_compressed_type_identifier: CompressedTypeIdentifier, registration_data: R::RegistrationData) -> Result<(), EventPollRegistrationError>
	{
		let arena = self.arenas.get_arena::<A, R>(reactor_compressed_type_identifier);

		R::do_initial_input_and_output_and_register_with_epoll_if_necesssary::<A, Self>(self, arena, reactor_compressed_type_identifier, registration_data)
	}

	#[inline(always)]
	fn add_a_new_reactor_slightly_slowly<A: Arena<R> + 'static, R: Reactor + 'static>(&self, registration_data: R::RegistrationData) -> Result<(), EventPollRegistrationError>
	{
		let (arena, reactor_compressed_type_identifier) = self.arenas.get_arena_and_reactor_compressed_type_identifier::<A, R>();

		R::do_initial_input_and_output_and_register_with_epoll_if_necesssary::<A, Self>(self, arena, reactor_compressed_type_identifier, registration_data)
	}
}

impl EventPollRegister for EventPoll
{
	#[inline(always)]
	fn register<A: Arena<R>, R: Reactor, F: FnOnce(&mut R, R::FileDescriptor) -> Result<(), EventPollRegistrationError>>(&self, arena: &A, reactor_compressed_type_identifier: CompressedTypeIdentifier, file_descriptor: R::FileDescriptor, add_flags: EPollAddFlags, initializer: F) -> Result<(), EventPollRegistrationError>
	{
		let (mut non_null, arena_index) = arena.allocate()?;
		let event_poll_token = EventPollToken::new(reactor_compressed_type_identifier, arena_index);

		match self.epoll_file_descriptor.add(file_descriptor.as_raw_fd(), add_flags, event_poll_token.0)
		{
			Err(error) =>
			{
				arena.reclaim(arena_index);
				Err(EventPollRegistrationError::from(error))
			}

			Ok(()) =>
			{
				let uninitialized_reactor = unsafe { non_null.as_mut() };
				initializer(uninitialized_reactor, file_descriptor)
			}
		}
	}
}

impl EventPoll
{
	/// Creates a new instance.
	///
	/// Only one instance per thread is normally required.
	#[inline(always)]
	pub(crate) fn new(arenas: Arenas, time_out_milliseconds: u16) -> Result<Self, CreationError>
	{
		Ok
		(
			Self
			{
				arenas,
				epoll_file_descriptor: EPollFileDescriptor::new()?,
				time_out: EPollTimeOut::in_n_milliseconds(time_out_milliseconds),
				spurious_event_suppression_of_already_closed_file_descriptors: UnsafeCell::new(HashSet::with_capacity(MaximumEvents))
			}
		)
	}

	/// One iteration of an event loop.
	///
	/// If interrupted by a signal then re-waits on epoll unless terminate has become true.
	#[inline(always)]
	pub(crate) fn event_loop_iteration<T: Terminate>(&self, terminate: &T) -> Result<(), String>
	{
		let mut events: [epoll_event; MaximumEvents] = unsafe { uninitialized() };

		self.spurious_event_suppression_of_already_closed_file_descriptors().clear();

		let ready_events = loop
		{
			match self.epoll_file_descriptor.wait(&mut events, self.time_out)
			{
				Ok(ready_events) => break ready_events,

				Err(EPollWaitError::Interrupted) => if likely!(terminate.should_continue())
				{
					continue
				}
				else
				{
					return Ok(())
				},
			}
		};

		for ready_event in ready_events
		{
			let event_poll_token = EventPollToken(ready_event.token());

			if unlikely!(self.spurious_event_suppression_of_already_closed_file_descriptors().contains(&event_poll_token))
			{
				continue
			}

			let result = self.react::<T>(event_poll_token, ready_event.flags(), terminate);
			if let Err(reason) = result
			{
				terminate.begin_termination();
				return Err(reason)
			}
		}

		Ok(())
	}

	#[inline(always)]
	fn spurious_event_suppression_of_already_closed_file_descriptors(&self) -> &mut HashSet<EventPollToken>
	{
		unsafe { &mut * self.spurious_event_suppression_of_already_closed_file_descriptors.get() }
	}

	#[inline(always)]
	fn react<T: Terminate>(&self, event_poll_token: EventPollToken, event_flags: EPollEventFlags, terminate: &T) -> Result<(), String>
	{
		let reactor_compressed_type_identifier = event_poll_token.reactor_compressed_type_identifier();
		let (unsized_arena, react_function_pointer) = self.arenas.get_unsized_arena_and_react_function_pointer(reactor_compressed_type_identifier);
		react_function_pointer(self, unsized_arena, event_poll_token, event_flags, unsafe { NonNull::new_unchecked(terminate as *const _ as *mut _) })
	}

	#[inline(always)]
	pub(crate) fn react_function_pointer<A: Arena<R>, R: Reactor, T: Terminate>(&self, arena: NonNull<A>, event_poll_token: EventPollToken, event_flags: EPollEventFlags, terminate: NonNull<T>) -> Result<(), String>
	{
		let arena = unsafe { & * arena.as_ptr() };
		let terminate = unsafe { & * terminate.as_ptr() };

		let arena_index = event_poll_token.arena_index();

		let reactor = arena.get(arena_index);

		match reactor.react(event_flags, terminate)
		{
			Err(reason) => Err(reason),

			Ok(dispose) =>
			{
				if unlikely!(dispose)
				{
					let first_insertion = self.spurious_event_suppression_of_already_closed_file_descriptors().insert(event_poll_token);
					debug_assert!(first_insertion, "Spurious event somehow not captured and double-close of file descriptor occurred");

					arena.reclaim(arena_index);
				}
				Ok(())
			}
		}
	}
}

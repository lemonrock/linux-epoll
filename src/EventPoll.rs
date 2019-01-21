// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Wraps event polling.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventPoll<AS: Arenas>
{
	arenas: ManuallyDrop<AS>,
	epoll_file_descriptor: ManuallyDrop<EPollFileDescriptor>,
}

impl<AS: Arenas> Drop for EventPoll<AS>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		// Arenas may hold Reactors which may hold Coroutines (eg StreamingSocketCommon::coroutine()) which themselves hold references or copies of file descriptors that are also held implicitly as tokens registered with the `epoll_file_descriptor`.
		//
		// One way out of this problem would be to use `libc::dup()`, but that requires a syscall.
		unsafe
		{
			ManuallyDrop(&mut self.arenas)
		}

		if let Ok((_header, epoll_information_items)) = self.epoll_file_descriptor.information()
		{
			for epoll_information_item in epoll_information_items
			{
				let raw_file_descriptor = epoll_information_item.target_file_descriptor;

				/// We can not easily find the true wrapping new type of this file descriptor.
				///
				/// (The actual process would be to parse `EPollInformationItem.token`).
				struct GenericFileDescriptor(RawFd);

				impl Drop for GenericFileDescriptor
				{
					#[inline(always)]
					fn drop(&mut self)
					{
						self.0.close()
					}
				}

				impl AsRawFd for GenericFileDescriptor
				{
					#[inline(always)]
					fn as_raw_fd(&self) -> RawFd
					{
						self.0
					}
				}

				self.deregister_and_close(GenericFileDescriptor(epoll_information_item.target_file_descriptor));
			}
		}

		unsafe
		{
			ManuallyDrop(&mut self.epoll_file_descriptor)
		}
	}
}

impl<AS: Arenas> EventPoll<AS>
{
	/// Creates a new instance.
	///
	/// Only one instance per thread is normally required.
	#[inline(always)]
	pub fn new(arenas: AS) -> Result<Self, CreationError>
	{
		Ok
		(
			Self
			{
				arenas: ManuallyDrop::new(arenas),
				epoll_file_descriptor: ManuallyDrop::new(EPollFileDescriptor::new()?),
			}
		)
	}

	/// Add a new reactor.
	pub fn add<R: Reactor<AS, A>, A: Arena<R, AS>>(&self, registration_data: R::RegistrationData) -> Result<(), EventPollRegistrationError>
	{
		R::do_initial_input_and_output_and_register_with_epoll_if_necesssary(self, registration_data)
	}

	/// Register a new file descriptor and allocate space for its management data in an arena.
	#[inline(always)]
	pub(crate) fn register<R: Reactor<AS, A>, A: Arena<R, AS>, F: FnOnce(&mut R) -> Result<(), EventPollRegistrationError>>(&self, file_descriptor: R::FileDescriptor, add_flags: EPollAddFlags, initializer: F) -> Result<(), EventPollRegistrationError>
	{
		let arena = R::our_arena(&self.arenas);

		let (mut non_null, arena_index) = arena.allocate()?;
		let event_poll_token = R::FileDescriptorKind.new_event_poll_token(&file_descriptor, arena_index);

		match self.epoll_file_descriptor.add(file_descriptor.as_raw_fd(), add_flags, event_poll_token.0)
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
				let result = FileDescriptorKind::react(self, event_poll_token, &mut spurious_event_suppression_of_already_closed_file_descriptors, &self.arenas, ready_event.flags(), terminate);
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

	#[inline(always)]
	pub(crate) fn deregister_and_close(&self, file_descriptor: impl AsRawFd)
	{
		if cfg!(not(feature = "assume-file-descriptors-are-never-duplicated"))
		{
			let raw_file_descriptor = file_descriptor.as_raw_fd();
			self.epoll_file_descriptor.delete(raw_file_descriptor);
		}
		drop(file_descriptor);
	}
}

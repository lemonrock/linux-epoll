// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// A reactor 'reacts' to events becoming ready from an epoll instance.
pub trait Reactor: Sized
{
	/// An associated file descriptor type.
	type FileDescriptor: AsRawFd;

	/// Data to pass to `do_initial_input_and_output_and_register_with_epoll_if_necesssary()`.
	type RegistrationData: Sized;

	/// Register with epoll.
	fn do_initial_input_and_output_and_register_with_epoll_if_necesssary<A: Arena<Self>, EPR: EventPollRegister>(event_poll_register: &EPR, arena: &A, reactor_compressed_type_identifier: CompressedTypeIdentifier, registration_data: Self::RegistrationData) -> Result<(), EventPollRegistrationError>;

	/// React to events becoming ready.
	///
	/// If `Ok(true)` is returned then the file descriptor is de-registered and closed; if `Ok(false)` is returned then it isn't.
	/// If an `Err` is returned then all activity is cut short; any dequeued events not yet 'reacted' to are discarded.
	fn react(&mut self, event_flags: EPollEventFlags, terminate: &impl Terminate) -> Result<bool, String>;
}

// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// A reactor 'reacts' to events becoming ready from an epoll instance.
pub trait Reactor: Sized
{
	/// An associated file descriptor type.
	type FileDescriptor: AsRawFd;

	/// File descriptor kind.
	const FileDescriptorKind: FileDescriptorKind;

	/// Data to pass to `register_with_epoll()`.
	type RegistrationData: Sized;

	/// Register with epoll.
	fn register_with_epoll(event_poll: &EventPollWrapper<impl Arenas>, arena: &impl Arena<Self>, registration_data: Self::RegistrationData) -> Result<(), EventPollRegistrationError>;

	/// React to events becoming ready.
	///
	/// If an error is returned then all activity is cut short; any dequeued events not yet 'reacted' to are discarded.
	fn react(&mut self, file_descriptor: &Self::FileDescriptor, event_flags: EPollEventFlags, terminate: &impl Terminate) -> Result<bool, String>;
}

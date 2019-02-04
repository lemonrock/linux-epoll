// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Adaptor for something that implements the trait `ReactorsRegistrar`.
#[derive(Debug, Copy, Clone)]
pub struct AdaptedReactorsRegistrar(NonNull<EventPoll>);

impl ReactorsRegistrar for AdaptedReactorsRegistrar
{
	#[inline(always)]
	unsafe fn add_a_new_reactor_efficiently<A: Arena<R> + 'static, R: Reactor + 'static>(&self, reactor_compressed_type_identifier: CompressedTypeIdentifier, registration_data: R::RegistrationData) -> Result<(), EventPollRegistrationError>
	{
		self.event_poll().add_a_new_reactor_efficiently::<A, R>(reactor_compressed_type_identifier, registration_data)
	}

	#[inline(always)]
	fn add_a_new_reactor_slightly_slowly<A: Arena<R> + 'static, R: Reactor + 'static>(&self, registration_data: R::RegistrationData) -> Result<(), EventPollRegistrationError>
	{
		self.event_poll().add_a_new_reactor_slightly_slowly::<A, R>(registration_data)
	}
}

impl AdaptedReactorsRegistrar
{
	/// This is a raw pointer as if we pollute the struct definition with a lifetime (eg `AdaptedReactorsRegistrar<'a>`, the same lifetime ends up in the trait definition `Registration`, and this is false; no such lifetime actually exists.
	#[inline(always)]
	fn event_poll(&self) -> &EventPoll
	{
		unsafe { & * self.0.as_ptr() }
	}

	#[inline(always)]
	pub(crate) fn wrap(event_poll: &EventPoll) -> Self
	{
		Self(unsafe { NonNull::new_unchecked(event_poll as *const _ as *mut _) })
	}
}

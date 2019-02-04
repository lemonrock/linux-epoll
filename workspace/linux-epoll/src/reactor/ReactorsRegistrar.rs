// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Registrar for reactors.
pub trait ReactorsRegistrar
{
	/// Adds a new reactor, efficiently.
	///
	/// The arena used by the reactor **MUST** have been previously added to the `Arenas` through the `ArenasRegistrar`.
	///
	/// Very unsafe as no checks are made that `reactor_compressed_type_identifier` is actually for `R`.
	unsafe fn add_a_new_reactor_efficiently<A: Arena<R> + 'static, R: Reactor + 'static>(&self, reactor_compressed_type_identifier: CompressedTypeIdentifier, registration_data: R::RegistrationData) -> Result<(), EventPollRegistrationError>;

	/// Adds a new reactor, slightly slowly.
	///
	/// The arena used by the reactor **MUST** have been previously added to the `Arenas` through the `ArenasRegistrar`.
	fn add_a_new_reactor_slightly_slowly<A: Arena<R> + 'static, R: Reactor + 'static>(&self, registration_data: R::RegistrationData) -> Result<(), EventPollRegistrationError>;
}

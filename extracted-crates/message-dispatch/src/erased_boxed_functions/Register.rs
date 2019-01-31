// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Allows one to register functions to handle objects.
pub trait Register<R>
{
	/// Registers a message handler and returns a `CompressedTypeIdentifier` to refer to it.
	///
	/// `CompressedTypeIdentifier` are monotonically increasing from 0 (inclusive), so they can be predicted.
	///
	/// Registration should only occur from the thread that will also be consuming (dequeuing) messages from this queue, unless the registered message handler is thread context aware (which is unlikely to be the case and difficult to make work).
	///
	/// Regardless, there can only be one consuming (reading) thread at a time for the queue; this is not enforced.
	///
	/// Panics if the message handler has already been registered (only if `debug_assertions` are configured).
	///
	/// Panics if there is not space for more message handlers (only 256 message handlers are allowed) (only if `debug_assertions` are configured).
	fn enter_into_the_register<Function: FnMut(&mut Receiver) -> R + 'static, Receiver: 'static>(&mut self, function: Function) -> CompressedTypeIdentifier;
}

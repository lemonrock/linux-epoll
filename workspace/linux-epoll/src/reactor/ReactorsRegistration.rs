// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Used to pass a constructor across threads.
pub trait ReactorsRegistration
{
	/// Register any reactors with the `reactors_registrar` when called.
	///
	/// Will be called once per thread.
	fn register_any_reactors<AR: ReactorsRegistrar>(&self, reactors_registrar: &AR);
}

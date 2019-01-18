// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Abstracts the need to terminate a loop or application and to begin and check termination.
pub trait Terminate: Send + Sync
{
	/// Begin termination.
	fn begin_termination(&self);

	/// Begin termination (due to a panic).
	fn begin_termination_due_to_panic(&self, panic_info: &PanicInfo);

	/// Should finish.
	fn should_finish(&self) -> bool;

	/// Should continue (opposite of `should_finish()`).
	#[inline(always)]
	fn should_continue(&self) -> bool
	{
		!self.should_finish()
	}
}

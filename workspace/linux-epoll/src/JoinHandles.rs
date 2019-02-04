// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


pub(crate) struct JoinHandles<'terminate, T: 'terminate + Terminate>
{
	join_handles: Vec<JoinHandle<()>>,
	terminate: &'terminate T,
}

impl<'terminate, T: 'terminate + Terminate> Drop for JoinHandles<'terminate, T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		for join_handle in self.join_handles.drain(..)
		{
			if let Err(panic_information) = join_handle.join()
			{
				self.terminate.begin_termination_due_to_irrecoverable_error(&panic_information)
			}
		}
	}
}

impl<'terminate, T: 'terminate + Terminate> JoinHandles<'terminate, T>
{
	#[inline(always)]
	pub(crate) fn new(logical_cores: &LogicalCores, terminate: &'terminate T) -> Self
	{
		Self
		{
			join_handles: Vec::with_capacity(logical_cores.len()),
			terminate,
		}
	}

	#[inline(always)]
	pub(crate) fn push(&mut self, join_handle: JoinHandle<()>)
	{
		self.join_handles.push(join_handle)
	}
}

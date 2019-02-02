// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Process configuration.
#[derive(Default, Debug)]
#[derive(Deserialize)]
#[serde(default)]
pub struct ProcessConfiguration
{
	/// Common configuration.
	pub process_common_configuration: ProcessCommonConfiguration,

	/// Per-thread stack size.
	pub per_thread_stack_size: usize,

	/// Scheduler to use per thread.
	pub per_thread_scheduler: Scheduler,
}

impl Default for ProcessConfiguration
{
	fn default() -> Self
	{
		Self
		{
			process_common_configuration: ProcessCommonConfiguration::default(),
			per_thread_stack_size: 2^16,
			per_thread_scheduler: Scheduler::RealTimeFirstInFirstOut(RealTimeSchedulerPriority::_99),
		}
	}
}

impl ProcessConfiguration
{
	/// Is the process running interactively (ie not as a daemon).
	#[inline(always)]
	pub fn running_interactively(&self) -> bool
	{
		self.process_common_configuration.running_interactively()
	}
}

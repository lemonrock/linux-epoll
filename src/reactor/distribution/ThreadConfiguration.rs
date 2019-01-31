// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.






use ::std::thread::Builder;




pub struct ThreadConfiguration<T: Terminate>
{
	name: String,
	stack_size: usize,
	terminate: Arc<T>,
	queue_per_threads_publisher: QueuePerThreadQueuesPublisher,
}

impl<T: Terminate> ThreadConfiguration<T>
{
	pub fn main_thread(logical_cores: &LogicalCores)
	{
		logical_cores.set_current_process_affinity();

		// Which core do we want to run the main thread on? It's not particularly important, but we might want to use one core as a 'general purpose' rather than share, say, core 0.

		let per_logical_core_data = logical_cores.populate_per_logical_core_data(constructor);
	}

	/// Spawns a new thread.
	pub fn spawn(&self, logical_core_identifier: LogicalCoreIdentifier)
	{
		let terminate = self.terminate.clone();
		let queue_per_threads_publisher = self.queue_per_threads_publisher.clone();

		let name = format!("LogicalCore-{:?}", logical_core_identifier);
		let result = Builder::new().stack_size(self.stack_size).name(name).spawn(move ||
		{
			LogicalCores::set_current_thread_affinity();
			while terminate.should_continue()
			{

				pub fn new<MH: Fn(LogicalCoreIdentifier) -> MutableTypeErasedBoxedFunctionCompressedMap<Result<(), E>> + Send + 'static + Copy>(publisher: &QueuePerThreadQueuesPublisher, terminate: &Arc<T>, message_handlers: MH)
			}
		});

		// result contains a joinhandle and any potential errors.
	}
}

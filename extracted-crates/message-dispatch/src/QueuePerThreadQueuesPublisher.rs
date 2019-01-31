// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Publishes to a queue used by a particular thread.
///
/// Assumes a thread-per-logical core model.
#[derive(Debug, Clone)]
pub struct QueuePerThreadQueuesPublisher<MessageHandlerArguments: Debug + Copy, E: Debug>
{
	queues: Arc<PerLogicalCoreData<Arc<Queue<MessageHandlerArguments, E>>>>,
}

unsafe impl<MessageHandlerArguments: Debug + Copy, E: Debug> Send for QueuePerThreadQueuesPublisher<MessageHandlerArguments, E>
{
}

unsafe impl<MessageHandlerArguments: Debug + Copy, E: Debug> Sync for QueuePerThreadQueuesPublisher<MessageHandlerArguments, E>
{
}

impl<MessageHandlerArguments: Debug + Copy, E: Debug> QueuePerThreadQueuesPublisher<MessageHandlerArguments, E>
{
	/// Allocate a new instance.
	#[inline(always)]
	pub fn allocate(logical_cores: &LogicalCores, queue_size_in_bytes: usize) -> Self
	{
		Self
		{
			queues: Queue::queues(logical_cores, queue_size_in_bytes)
		}
	}

	/// Publish a message to be received by the queue for `logical_core_identifier`.
	///
	/// Assumes a thread-per-logical core model.
	///
	/// If there is no registered queue, publishes to the queue which is assumed to exist for the current thread.
	#[inline(always)]
	pub fn publish_message<MessageContents>(&self, logical_core_identifier: LogicalCoreIdentifier, compressed_type_identifier: CompressedTypeIdentifier, message_contents_constructor: impl FnOnce(NonNull<MessageContents>))
	{
		let queue = self.queues.get_or_current(logical_core_identifier);
		queue.enqueue(compressed_type_identifier, message_contents_constructor)
	}

	#[inline(always)]
	fn get_queue(&self, logical_core_identifier: LogicalCoreIdentifier) -> Arc<Queue<MessageHandlerArguments, E>>
	{
		self.queues.get(logical_core_identifier).unwrap().clone()
	}
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A subscriber to a queue.
#[derive(Debug)]
pub struct PerThreadQueueSubscriber<T: Terminate, MessageHandlerArguments: Debug + Copy, E: Debug>
{
	queue: Arc<Queue<MessageHandlerArguments, E>>,
	terminate: Arc<T>,
}

impl<T: Terminate, MessageHandlerArguments: Debug + Copy, E: Debug> PerThreadQueueSubscriber<T, MessageHandlerArguments, E>
{
	/// Creates a new instance for the current logical core.
	///
	/// Thus must only be run on the thread that is doing subscribing.
	#[inline(always)]
	pub fn new<MHR: MessageHandlersRegistration<MessageHandlerArguments=MessageHandlerArguments, E=E>>(queue_per_threads_publisher: QueuePerThreadQueuesPublisher<MessageHandlerArguments, E>, terminate: Arc<T>, message_handlers_registration: &MHR, message_handlers_registration_arguments: &MHR::Arguments) -> Self
	{
		let logical_core_identifier = LogicalCores::current_logical_core();

		let queue = queue_per_threads_publisher.get_queue(logical_core_identifier);
		message_handlers_registration.register_all_message_handlers(queue.message_handlers(), message_handlers_registration_arguments);

		Self
		{
			queue,
			terminate,
		}
	}

	/// Receives and handles messages; short-circuits if `terminate` becomes true or a message handler returns an error `E`.
	#[inline(always)]
	pub fn receive_and_handle_messages(&self, message_handler_arguments: MessageHandlerArguments) -> Result<(), E>
	{
		self.queue.dequeue(self.terminate.deref(), message_handler_arguments)
	}
}

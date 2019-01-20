// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A file descriptor consumer.
#[derive(Debug)]
pub struct FileDescriptorConsumer<SD: SocketData>
{
	consumer: RingBufferConsumer<StreamingSocketFileDescriptor<SD>>,
}

impl<SD: SocketData> FileDescriptorConsumer<SD>
{
	#[inline(always)]
	fn new(consumer: RingBufferConsumer<StreamingSocketFileDescriptor<SD>>) -> Self
	{
		Self
		{
			consumer
		}
	}

	/// Consume.
	#[inline(always)]
	pub fn consume<A: Arenas>(&self, event_poll: &EventPoll<A>)
	{
		let slice_guard = self.consumer.consume();

		for streaming_socket_file_descriptor in slice_guard
		{
//			let registration_data: SomeReactor::RegistrationData = XXXX;
//			match event_poll.add::<SomeReactor>(registration_data)
//			{
//				Err(_event_poll_registration_error) => (),
//				Ok(()) => (),
//			}

			unimplemented!("TODO");

			// TODO: some sort of error log... capture dropped file descriptors and other nasty errors.

			// Now need to do something with the file descriptor
		}
	}
}

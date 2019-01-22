// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Consumes file descriptors for streaming sockets sent from other logical cores and then instantiates a Reactor for them.
#[derive(Debug)]
pub struct FileDescriptorConsumer<SSR: StreamingSocketReactor<SF, SU, SD, AS, A>, SF: StreamFactory<SD>, SU: StreamUser<SF::S>, SD: SocketData, AS: Arenas, A: Arena<SSR, AS>>
{
	consumer: RingBufferConsumer<StreamingSocketFileDescriptor<SD>>,
	stream_factory: Rc<SF>,
	stream_user: Rc<SU>,
	marker: PhantomData<(SSR, AS, A)>,
}

impl<SSR: StreamingSocketReactor<SF, SU, SD, AS, A>, SF: StreamFactory<SD>, SU: StreamUser<SF::S>, SD: SocketData, AS: Arenas, A: Arena<SSR, AS>> FileDescriptorConsumer<SSR, SF, SU, SD, AS, A>
{
	#[inline(always)]
	pub(crate) fn new(consumer: RingBufferConsumer<StreamingSocketFileDescriptor<SD>>, (stream_factory, stream_user): (SF, SU)) -> Self
	{
		Self
		{
			consumer,
			stream_factory: Rc::new(stream_factory),
			stream_user: Rc::new(stream_user),
			marker: PhantomData,
		}
	}

	/// Consume.
	#[inline(always)]
	pub fn consume(&self, event_poll: &EventPoll<AS>)
	{
		let slice_guard = self.consumer.consume();

		for streaming_socket_file_descriptor in slice_guard
		{
//			let registration_data: (StreamingSocketFileDescriptor<SD>, Rc<SF>, SF::AdditionalArguments, Rc<SU>) = (streaming_socket_file_descriptor, self.stream_factory.clone(), additional_data, self.stream_user.clone());
//			SSR::do_initial_input_and_output_and_register_with_epoll_if_necesssary(event_poll, registration_data);
		}

		unimplemented!("TODO: Fix the problem with additional_data - where does it get passed?");


		// additional_data is rather hard - we can not distribute client-initiated connections easily without it; we need to pass it via the producer.
		// it is used for TLS client stream factory (for tls client session construction).

		// likewise, we have a problem with client and server initiated connections - they will need different stream_user.
		// likewise, we need to be able to choose a different stream_user for HTTP, HTTPS etc on different server listener ports as we provide different services.
		// and that is before the impact of ALPN.

		// we can punt a lot of this into stream user - it would need an integer key to make its decisions with.
		// but that is memory poor for the arena.

		//
	}
}

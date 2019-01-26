// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


type MessageOfVariableSize = u8;

/// A message consumer.
#[derive(Debug)]
pub struct MessageConsumer
{
	// might even be able to hint how-many-bytes.
	messages_available_hint: Arc<AtomicUsize>,
	message_handlers: MutableTypeErasedBoxedFunctionCompressedMap<Result<(), String>>
}

/// A factory that creates `MessageConsumer`s once assigned to a particular thread.
#[derive(Debug)]
pub struct MessageConsumerConstructor
{
	messages_available_hint: Arc<AtomicUsize>,
	#[cfg(debug_assertions)] logical_core_identifier: LogicalCoreIdentifier,
}

impl MessageConsumerConstructor
{
	/// Must only be run on the thread on the specific logical core that this consumer will execute on.
	#[inline(always)]
	pub fn create_on_thread(mut self, message_handlers: MutableTypeErasedBoxedFunctionCompressedMap<Result<(), String>>) -> MessageConsumer
	{
		debug_assert_eq!(self.logical_core_identifier, current_logical_core(), "Can only be executed on a thread running on logical core `{:?}`", self.logical_core_identifier);

		MessageConsumer
		{
			messages_available_hint: self.messages_available_hint,
			message_handlers,
		}
	}
}

/// An artifical structure for constructing the necessary message queues.
pub struct MessageQueues;

impl MessageQueues
{
	/// Creates the data structures needed to assign to logical core threads.
	pub fn new_for_all_cores(ring_buffer_capacity: usize, message_buffer_capacity: usize, logical_cores: &LogicalCores) -> PerLogicalCoreData<(MessagePublisher, MessageConsumerConstructor)>
	{
		debug_assert_ne!(ring_buffer_capacity, 0, "ring_buffer_capacity must not be zero");
		debug_assert_ne!(message_buffer_capacity, 0, "message_buffer_capacity must not be zero");
		debug_assert!(ring_buffer_capacity >= message_buffer_capacity, "ring_buffer_capacity must be the same or greater than message_buffer_capacity");

		let number_of_logical_cores = logical_cores.len();

		let mut consumers = PerLogicalCoreData::empty(logical_cores);
		let mut publishers_for_a_consumer = PerLogicalCoreData::empty(logical_cores);

		for logical_core_identifier_reference in logical_cores.iter()
		{
			let logical_core_identifier = (*logical_core_identifier_reference);

			let (consumer, producers) = RingBuffer::new(ring_buffer_capacity, number_of_logical_cores);
			consumers.set(logical_core_identifier, consumer);
			publishers_for_a_consumer.set(logical_core_identifier, producers);
		}

		consumers.map(|logical_core_identifier, consumer|
		{
			let messages_available_hint = Arc::new(AtomicUsize::new(0));

			(
				MessagePublisher
				{
					distribute_to: PerLogicalCoreData::new(logical_cores, |logical_core_identifier|
					{
						let producer = publishers_for_a_consumer.get_mut(logical_core_identifier).unwrap().pop().unwrap();
						let message_buffer = Vec::with_capacity(message_buffer_capacity);
						(producer, message_buffer)
					}),
					messages_available_hint: messages_available_hint.clone(),
				},
				MessageConsumerConstructor
				{
					messages_available_hint,
					#[cfg(debug_assertions)] logical_core_identifier,
				},
			)
		})
	}
}

/// Distributes using a message publisher streaming socket file descriptors.
///
/// Distribution is based on the logical CPU core that can best serve the streaming socket.
#[derive(Debug)]
pub struct StreamingSocketFileDescriptorMessageDistributor<SD: SocketData, ConnectionInitiationData: Send + Sync>
{
	message_publisher: MessagePublisher,
	compressed_type_identifier_for_StreamingSocketFileDescriptorMessageContents: CompressedTypeIdentifier,
	marker: PhantomData<StreamingSocketFileDescriptorMessageContents<SD, ConnectionInitiationData>>,
}

impl<SD: SocketData, ConnectionInitiationData: Send + Sync> StreamingSocketFileDescriptorMessageDistributor<SD, ConnectionInitiationData>
{
	/// Assigns.
	#[inline(always)]
	pub fn buffer(&mut self, terminate: &impl Terminate, streaming_socket_file_descriptor: StreamingSocketFileDescriptor<SD>, connection_initiation_data: ConnectionInitiationData)
	{
		let logical_core_identifier = streaming_socket_file_descriptor.logical_core_identifier();
		self.message_publisher.assign::<StreamingSocketFileDescriptorMessageContents<SD, ConnectionInitiationData>>(logical_core_identifier, terminate, self.compressed_type_identifier_for_StreamingSocketFileDescriptorMessageContents, |raw_pointer|
		{
			unsafe
			{
				let this: &mut StreamingSocketFileDescriptorMessageContents<SD, ConnectionInitiationData> = raw_pointer.as_mut();
				write(this.file_descriptor, streaming_socket_file_descriptor);
				write(this.connection_initiation_data, connection_initiation_data);
			}
		})
	}

	/// Publishes.
	///
	//TODO: Any file descriptors not distributed are closed.
	//TODO: Tertminate.
	#[inline(always)]
	pub fn publish(&mut self)
	{

	}
}

/// Can be for either a server or a client.
#[derive(Debug)]
pub struct StreamingSocketFileDescriptorMessageContents<SD: SocketData, ConnectionInitiationData: Send + Sync>
{
	file_descriptor: StreamingSocketFileDescriptor<SD>,
	connection_initiation_data: ConnectionInitiationData,
}


/// A message publisher.
#[derive(Debug)]
pub struct MessagePublisher
{
	distribute_to: PerLogicalCoreData<(RingBufferProducer<MessageOfVariableSize>, Vec<MessageOfVariableSize>)>,
	messages_available_hint: Arc<AtomicUsize>,
}

impl MessagePublisher
{
	// TODO: Do we want to eliminate the buffer, so eliminating a potential large memcpy and a large slice to acquire?
	// Or we can push the buffer to the wrapper of message publisher; this allows a type-safe drop if on terminate.
	// Or we just acquire and push - small slice, one memcpy / writes, potentially more overhead for consumer.
	// Another option - allocated, say, 256Kb; only enqueue a (relative pointer, size) tuple (can be usize). Enqueuers do a CAS to get memory. Or even a D-CAS ()
	/// Buffers messages.
	#[inline(always)]
	pub fn buffer<MessageContents>(&mut self, logical_core_identifier: usize, terminate: &impl Terminate, compressed_type_identifier: CompressedTypeIdentifier, message_contents_constructor: impl FnOnce(NonNull<MessageContents>))
	{
		#[inline(always)]
		fn space_available(buffer: &[u8]) -> usize
		{
			buffer.capacity() - buffer.len()
		}

		// The the `logical_core_identifier` might not have associated data because it was obtained using the `SO_INCOMING_CPU` socket option which can return an index for a CPU not assigned to this process.
		let (ring_buffer_producer, ref mut message_buffer) = self.distribute_to.get_mut_or(logical_core_identifier, || current_logical_core());

		let current_length = message_buffer.len();
		let space_starts_at = unsafe { message_buffer.as_mut_ptr().add(current_length) };
		let buffer = unsafe { from_raw_parts_mut(space_available(message_buffer)) };

		while terminate.should_continue()
		{
			match Message::enqueue::<MessageContents>(buffer, compressed_type_identifier, message_contents_constructor)
			{
				Ok(total_message_size_including_message_header) =>
				{
					unsafe { message_buffer.set_len(current_length + total_message_size_including_message_header) };
					return
				},

				Err(total_message_size_including_message_header) =>
				{
					debug_assert!(total_message_size_including_message_header <= message_buffer.capacity(), "Message is too big to ever fit in the pre-allocated data");

					while unlikely!(space_available(message_buffer) < total_message_size_including_message_header && terminate.should_continue())
					{
						ring_buffer_producer.repeatedly_acquire_and_try_to_populate(message_buffer)
					}
				}
			}
		}
	}

	/// Publishes.
	///
	// TODO: PROBLEM: Not publishing all messages, or terminating early, can leave data in the buffer which ISN'T dropped.
	#[inline(always)]
	pub fn publish(&mut self)
	{
		for data in self.distribute_to.iter_mut()
		{
			if let Some((producer, ref mut message_buffer)) = data
			{
				while terminate.should_continue()
				{
					producer.repeatedly_acquire_and_try_to_populate(message_buffer);
				}
			}
		}
	}
}

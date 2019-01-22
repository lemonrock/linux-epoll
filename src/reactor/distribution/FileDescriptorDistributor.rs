// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A file descriptor distributor.
#[derive(Debug)]
pub struct FileDescriptorDistributor<SD: SocketData>
{
	distribute_to: PerLogicalCoreData<(RingBufferProducer<StreamingSocketFileDescriptor<SD>>, Vec<StreamingSocketFileDescriptor<SD>>)>,
}

impl<SD: SocketData> FileDescriptorDistributor<SD>
{
	/// Creates the data structures needed to assign to logical core threads.
	///
	/// `factory` takes a `logical_core_index` and returns a new instance of a `StreamFactory` and a `StreamUser`.
	/// It is invoked on the *current thread* - be aware if relying on thread-local structure initialization that this won't work properly.
	pub fn new_for_all_cores<SSR: StreamingSocketReactor<SF, SU, SD, AS, A>, SF: StreamFactory<SD>, SU: StreamUser<SF::S>, AS: Arenas, A: Arena<SSR, AS>>(ring_buffer_capacity: usize, maximum_number_of_file_descriptors_pending_distribution_per_logical_core: usize, logical_cores: &LogicalCores, mut factory: impl FnMut(u16) -> (SF, SU)) -> PerLogicalCoreData<(FileDescriptorConsumer<SSR, SF, SU, SD, AS, A>, Self)>
	{
		debug_assert_ne!(ring_buffer_capacity, 0, "ring_buffer_capacity must not be zero");

		let number_of_logical_cores = logical_cores.len();

		let mut consumers = PerLogicalCoreData::empty(logical_cores);
		let mut producers_for_a_consumer = PerLogicalCoreData::empty(logical_cores);

		for logical_core_identifier_reference in logical_cores.iter()
		{
			let logical_core_identifier = (*logical_core_identifier_reference) as u16;

			let (consumer, producers) = RingBuffer::new(ring_buffer_capacity, number_of_logical_cores);
			consumers.set(logical_core_identifier, consumer);
			producers_for_a_consumer.set(logical_core_identifier, producers);
		}

		consumers.map(|logical_core_index, consumer|
		{
			(
				FileDescriptorConsumer::new(consumer, factory(logical_core_index)),
				Self
				{
					distribute_to: PerLogicalCoreData::new(logical_cores, |logical_core_identifier|
					{
						let producer = producers_for_a_consumer.get_mut(logical_core_identifier).unwrap().pop().unwrap();
						let pre_allocated_block_of_file_descriptor_messages = Vec::with_capacity(maximum_number_of_file_descriptors_pending_distribution_per_logical_core);
						(producer, pre_allocated_block_of_file_descriptor_messages)
					})
				}
			)
		})
	}

	/// Assigns.
	#[inline(always)]
	pub fn assign(&mut self, streaming_socket_file_descriptor: StreamingSocketFileDescriptor<SD>)
	{
		let logical_core_identifier = streaming_socket_file_descriptor.logical_core_identifier();

		// The the `logical_core_identifier` might not have associated data because it was obtained using the `SO_INCOMING_CPU` socket option which can return an index for a CPU not assigned to this process.
		let (producer, ref mut file_descriptors) = self.distribute_to.get_mut_or(logical_core_identifier, || current_logical_cpu());

		while unlikely!(file_descriptors.len() == file_descriptors.capacity())
		{
			producer.repeatedly_acquire_and_try_to_populate(file_descriptors)
		}

		file_descriptors.push(streaming_socket_file_descriptor)
	}

	/// Distributes.
	///
	/// Any file descriptors not distributed are closed.
	#[inline(always)]
	pub fn distribute(&mut self)
	{
		for data in self.distribute_to.iter_mut()
		{
			if let Some((producer, ref mut file_descriptors)) = data
			{
				producer.repeatedly_acquire_and_try_to_populate(file_descriptors);
				file_descriptors.clear()
			}
		}
	}
}

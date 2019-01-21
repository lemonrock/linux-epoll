// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A file descriptor distributor.
#[derive(Debug)]
pub struct FileDescriptorDistributor<SD: SocketData>
{
	producers: PerLogicalCoreData<(RingBufferProducer<StreamingSocketFileDescriptor<SD>>, Vec<StreamingSocketFileDescriptor<SD>>)>,
}

impl<SD: SocketData> FileDescriptorDistributor<SD>
{
	/// Creates thread data needed for all cores.
	pub fn new_for_all_cores(ring_buffer_capacity: usize, maximum_number_of_file_descriptors_pending_distribution_per_logical_core: usize, logical_cores: &LogicalCores) -> Vec<(Self, FileDescriptorConsumer<SD>)>
	{
		debug_assert_ne!(ring_buffer_capacity, 0, "ring_buffer_capacity must not be zero");

		let number_of_threads = logical_cores.len();

		let mut per_thread_data = Vec::with_capacity(number_of_threads);
		for _ in logical_cores.iter()
		{
			let (consumer, producers) = RingBuffer::new(ring_buffer_capacity, number_of_threads);
			let file_descriptor_distributor = Self::new(maximum_number_of_file_descriptors_pending_distribution_per_logical_core, logical_cores, producers);
			per_thread_data.push((file_descriptor_distributor, FileDescriptorConsumer::new(consumer)))
		};
		per_thread_data
	}

	#[inline(always)]
	fn new(maximum_number_of_file_descriptors_pending_distribution_per_logical_core: usize, logical_cores: &LogicalCores, mut producers: Vec<RingBufferProducer<StreamingSocketFileDescriptor<SD>>>) -> Self
	{
		Self
		{
			producers: PerLogicalCoreData::new(logical_cores, |_logical_core_identifier|
			{
				(producers.pop().unwrap(), Vec::with_capacity(maximum_number_of_file_descriptors_pending_distribution_per_logical_core))
			})
		}
	}

	/// Assigns.
	#[inline(always)]
	pub fn assign(&mut self, streaming_socket_file_descriptor: StreamingSocketFileDescriptor<SD>)
	{
		let logical_core_identifier = streaming_socket_file_descriptor.logical_core_identifier();

		// The the `logical_core_identifier` might not have associated data because it was obtained using the `SO_INCOMING_CPU` socket option which can return an index for a CPU not assigned to this process.
		let (producer, ref mut file_descriptors) = self.producers.get_mut_or(logical_core_identifier, || current_logical_cpu());

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
		for data in self.producers.iter_mut()
		{
			if let Some((producer, ref mut file_descriptors)) = data
			{
				producer.repeatedly_acquire_and_try_to_populate(file_descriptors);
				file_descriptors.clear()
			}
		}
	}
}

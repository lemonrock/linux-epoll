// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug)]
struct StreamingServerListenerSocketCommon<SD: SocketData, A: AccessControl<SD>>
{
	access_control: A,
	file_descriptor_distributor: FileDescriptorDistributor<SD>,
}

impl<A: AccessControl<sockaddr_in>> StreamingServerListenerSocketCommon<sockaddr_in, A>
{
	#[inline(always)]
	fn new_streaming_socket_file_descriptor(settings: &StreamingServerListenerSocketSettings, socket_address: SocketAddrV4) -> Result<StreamingServerListenerSocketFileDescriptor<sockaddr_in>, NewSocketServerListenerError>
	{
		StreamingServerListenerSocketFileDescriptor::new_transmission_control_protocol_over_internet_protocol_version_4_server_listener
		(
			socket_address,
			settings.send_buffer_size_in_bytes,
			settings.receive_buffer_size_in_bytes,
			settings.idles_before_keep_alive_seconds,
			settings.keep_alive_interval_seconds,
			settings.maximum_keep_alive_probes,
			settings.linger_seconds,
			settings.linger_in_FIN_WAIT2_seconds,
			settings.maximum_SYN_transmits,
			settings.back_log,
			current_logical_cpu(),
		)
	}
}

impl<A: AccessControl<sockaddr_in6>> StreamingServerListenerSocketCommon<sockaddr_in6, A>
{
	#[inline(always)]
	fn new_streaming_socket_file_descriptor(settings: &StreamingServerListenerSocketSettings, socket_address: SocketAddrV6) -> Result<StreamingServerListenerSocketFileDescriptor<sockaddr_in6>, NewSocketServerListenerError>
	{
		StreamingServerListenerSocketFileDescriptor::new_transmission_control_protocol_over_internet_protocol_version_6_server_listener
		(
			socket_address,
			settings.send_buffer_size_in_bytes,
			settings.receive_buffer_size_in_bytes,
			settings.idles_before_keep_alive_seconds,
			settings.keep_alive_interval_seconds,
			settings.maximum_keep_alive_probes,
			settings.linger_seconds,
			settings.linger_in_FIN_WAIT2_seconds,
			settings.maximum_SYN_transmits,
			settings.back_log,
			current_logical_cpu(),
		)
	}
}

impl<A: AccessControl<sockaddr_un>> StreamingServerListenerSocketCommon<sockaddr_un, A>
{
	#[inline(always)]
	fn new_streaming_socket_file_descriptor(settings: &StreamingServerListenerSocketSettings, socket_address: &UnixSocketAddress<impl AsRef<Path>>) -> Result<StreamingServerListenerSocketFileDescriptor<sockaddr_un>, NewSocketServerListenerError>
	{
		StreamingServerListenerSocketFileDescriptor::new_streaming_unix_domain_socket_server_listener
		(
			socket_address,
			settings.send_buffer_size_in_bytes,
			settings.back_log,
			current_logical_cpu(),
		)
	}
}

impl<SD: SocketData, A: AccessControl<SD>> StreamingServerListenerSocketCommon<SD, A>
{
	#[inline(always)]
	fn do_initial_input_and_output_and_register_with_epoll_if_necesssary<SSLR: StreamingServerListenerReactor>(event_poll: &EventPoll<impl Arenas>, streaming_server_listener_socket_file_descriptor: StreamingServerListenerSocketFileDescriptor<SD>, access_control: A, file_descriptor_distributor: FileDescriptorDistributor<SD>) -> Result<(), EventPollRegistrationError>
	{
		const AddFlags: EpollAddFlags = EPollAddFlags::EdgeTriggeredInput | EPollAddFlags::Exclusive;

		event_poll.register::<SSLR>(streaming_server_listener_socket_file_descriptor, AddFlags, |r|
		{
			r.initialize
			(
				Self
				{
					access_control,
					file_descriptor_distributor,
				}
			);
			Ok(())
		})
	}

	fn react(&mut self, event_poll: &EventPoll<impl Arenas>, file_descriptor: &Self::FileDescriptor, event_flags: EPollEventFlags, terminate: &impl Terminate) -> Result<bool, String>
	{
		debug_assert_eq!(event_flags, EPollEventFlags::Input, "flags contained a flag other than `Input`");

		while terminate.should_continue()
		{
			use self::SocketAcceptError::*;

			match file_descriptor.accept()
			{
				Ok(AcceptedConnection { streaming_socket_file_descriptor, peer_address }) => if likely!(self.is_remote_peer_allowed(peer_address))
				{
					self.file_descriptor_distributor.assign(streaming_socket_file_descriptor)

					self.dispatch_connection_to_appropriate_thread(streaming_socket_file_descriptor);
				},

				Err(error) => match error
				{
					PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded | SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded | KernelWouldBeOutOfMemory => continue,

					ConnectionFailed(_connection_failed_reason @ _) => continue,

					Interrupted => continue,

					Again => return Ok(false),
				},
			}
		}

		Ok(false)
	}

	#[inline(always)]
	fn is_remote_peer_allowed(&self, remote_peer_address: SD) -> bool
	{
		self.access_control.is_remote_peer_allowed(remote_peer_address)
	}

	#[inline(always)]
	fn dispatch_connection_to_appropriate_thread(&self, streaming_socket_file_descriptor: StreamingSocketFileDescriptor<SD>)
	{
		let cpu = streaming_socket_file_descriptor.logical_cpu_identifier();

		if let Ok(streaming_socket_handler) = self.streaming_socket_handler_factory.create()
		{
			// TODO
			xxxx

			StreamingSocketInternetProtocolVersion4Reactor::<SSHF::StreamingSocketHandler>::do_initial_input_and_output_and_register_with_epoll_if_necesssary(event_poll, (streaming_socket_handler, streaming_socket_file_descriptor))
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerLogicalCoreData<T>
{
	logical_cores_data: Box<[Option<T>]>,
	highest_logical_core_index: usize,
}

impl<T> Deref for PerLogicalCoreData<T>
{
	type Target = [Option<T>];

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.logical_cores_data
	}
}
impl<T> DerefMut for PerLogicalCoreData<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.logical_cores_data
	}
}

impl<T> PerLogicalCoreData<T>
{
	/// `t_initializer` is called for each defined logical core in `logical_cores`; it is passed the logical core's identifier.
	#[inline(always)]
	pub fn new(logical_cores: &LogicalCores, t_initializer: FnMut(usize)) -> Self
	{
		let number_of_logical_cores = logical_cores.len();
		assert_ne!(number_of_logical_cores, 0, "There are no logical cores specified");

		let mut logical_core_index = 0;
		Self
		{
			logical_cores_data:
			{
				// Since the highest logical core is not necessarily the same as the length, this could still be resized.
				let mut logical_cores_data = Vec::with_capacity(number_of_logical_cores);
				let mut current_logical_core = 0;
				for logical_core_index_reference in logical_cores.iter()
				{
					logical_core_index = (*logical_core_index_reference);

					while current_logical_core < logical_core_index
					{
						logical_cores_data.push(None);
						current_logical_core += 1;
					}
					debug_assert_eq!(current_logical_core, logical_core_index);
					logical_cores_data.push(Some(t_initializer(logical_core_index)));

					current_logical_core = logical_core_index + 1;
				}
				debug_assert_eq!(current_logical_core, logical_cores_data.len());

				logical_cores_data.into_boxed_slice()
			},

			highest_logical_core_index: logical_core_index,
		}
	}

	/// Gets the data for a particular logical core.
	///
	/// If the logical core does not exist (or does not have assigned data), returns None; this can happen on Linux if using the SO_INCOMING_CPU socket option, which can map to a CPU not assigned to the process.
	#[inline(always)]
	pub fn get(&self, logical_core_index: usize) -> Option<&T>
	{
		if unlikely!(logical_core_index >= self.logical_cores_data.len())
		{
			return None
		}
		unsafe { self.logical_cores_data.get_unchecked(logical_core_index).as_ref() }
	}

	/// Gets the mutable data for a particular logical core.
	///
	/// If the logical core does not exist (or does not have assigned data), returns None; this can happen on Linux if using the` SO_INCOMING_CPU` socket option, which can return an index for a CPU not assigned to the process.
	#[inline(always)]
	pub fn get_mut(&mut self, logical_core_index: usize) -> Option<&mut T>
	{
		if unlikely!(logical_core_index >= self.logical_cores_data.len())
		{
			return None
		}
		unsafe { self.logical_cores_data.get_unchecked_mut(logical_core_index).as_mut() }
	}
}

pub struct FileDescriptorDistributor<SD: SocketData>
{
	producers: PerLogicalCoreData<(RingBufferProducer<RawFd>, Vec<StreamingSocketFileDescriptor<SD>>)>,
}

impl<SD: SocketData> FileDescriptorDistributor<SD>
{
	#[inline(always)]
	pub fn new(logical_cores: &LogicalCores, maximum_number_of_file_descriptors_pending_distribution_per_logical_core: usize)
	{
		Self
		{
			producers: PerLogicalCoreData::new(logical_cores, |_logical_core_index|
			{
				Vec::with_capacity(maximum_number_of_file_descriptors_pending_distribution_per_logical_core)
			})
		}
	}

	/// Assigns.
	#[inline(always)]
	pub fn assign(&mut self, file_descriptor: StreamingSocketFileDescriptor<SD>)
	{
		let logical_core_index = file_descriptor.logical_core_index();

		let &mut (producer, file_descriptors) = match self.producers.get_mut(logical_core_index)
		{
			Some(data) => data,

			// The the `logical_core_index` might not have associated data because it was obtained using the `SO_INCOMING_CPU` socket option which can return an index for a CPU not assigned to this process.
			None => self.producers.get_mut(self.producers.highest_logical_core_index).unwrap(),
		};

		while unlikely!(file_descriptors.len() == file_descriptors.capacity())
		{
			producer.repeatedly_acquire_and_try_to_populate(file_descriptors)
		}

		file_descriptors.push(file_descriptor)
	}

	/// Distributes.
	///
	/// Any file descriptors not distributed are closed.
	#[inline(always)]
	pub fn distribute(&mut self)
	{
		for data in self.producers.iter_mut()
		{
			if let Some(&mut (producer, file_descriptors)) = data
			{
				producer.repeatedly_acquire_and_try_to_populate(file_descriptors);
				file_descriptors.clear()
			}
		}
	}
}

pub const MaximumLogicalCores: usize = 256;

pub struct EventPollWithDistributor<A: Arenas>
{
	event_poll: EventPoll<A>,
	internet_protocol_version_4: [Option<FileDescriptorDistributor<sockaddr_in>>; MaximumLogicalCores],
	internet_protocol_version_4_consumer: FileDescriptorConsumer<sockaddr_in>,
	internet_protocol_version_6: [Option<FileDescriptorDistributor<sockaddr_in6>>; MaximumLogicalCores],
	internet_protocol_version_6_consumer: FileDescriptorConsumer<sockaddr_in6>,
	unix_domain: [Option<FileDescriptorDistributor<sockaddr_un>>; MaximumLogicalCores],
	unix_domain_consumer: FileDescriptorConsumer<sockaddr_un>,
}

impl<A: Arenas> EventPollWithDistributor
{
	pub fn distribute()
	{

	}

	#[inline(always)]
	pub fn new_consumer(maximum_number_of_file_descriptors_being_distributed_at_once: usize, logical_cores: &LogicalCores) -> (FileDescriptorConsumer<SD>, RingBufferProducerIterator<SD>)
	{
		let number_of_producers = logical_cores.len();
		let (consumer, producer_iterator) = RingBuffer::new(capacity, number_of_producers);
		(
			FileDescriptorConsumer
			{
				consumer,
				marker: PhantomData,
			},
			producer_iterator,
		)
	}
}


pub struct FileDescriptorConsumer<SD: SocketData>
{
	consumer: RingBufferConsumer<RawFd>,
	marker: PhantomData<SD>,
}

impl<SD: SocketData> FileDescriptorConsumer<SD>
{
	#[inline(always)]
	pub fn consume(&self, user: FnMut(StreamingSocketFileDescriptor<SD>))
	{
		let guard = self.consumer.consume();
		let slice = guard.buffer_slice;
		for index in 0 .. slice.len()
		{
			user(StreamingSocketFileDescriptor::from_raw_fd(unsafe { slice.get_unchecked(index) }))
		}
	}
}



// Need a simple queue of stuff to transfer between threads. No need to wake up epolls with syscalls unless time outs are very long (unlikely).
// Could adapt a buffer based queue we started to use for DPDK as file descriptors are just bytes.

// https://github.com/lemonrock/lock-free-multi-producer-single-consumer-ring-buffer

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A queue of variably-sized messages of different types (eg structs, traits, etc), suitable for many-writer, single consumer usage.
///
/// Ideal for a thread control queue.
#[derive(Debug)]
pub struct Queue<E: Debug>
{
	magic_ring_buffer: MagicRingBuffer,
	message_handlers: UnsafeCell<MutableTypeErasedBoxedFunctionCompressedMap<Result<(), E>>>,
}

impl<E: Debug> Drop for Queue<E>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let message_handlers = self.message_handlers();
		while
		{
			let more_data_to_read = self.magic_ring_buffer.single_reader_read_some_data::<E, _>
			(
				|buffer|
				{
					Message::process_next_message_in_buffer::<Result<(), E>, _>
					(
						buffer,
						|compressed_type_identifier, receiver|
						{
							message_handlers.drop_in_place(compressed_type_identifier, receiver);
							Ok(())
						}
					)
				}
			).expect("Should never happen");

			more_data_to_read
		}
		{
		}
	}
}

impl<E: Debug> Enqueue for Queue<E>
{
	#[inline(always)]
	fn enqueue<MessageContents>(&self, compressed_type_identifier: CompressedTypeIdentifier, message_contents_constructor: impl FnOnce(NonNull<MessageContents>))
	{
		Message::enqueue(&self.magic_ring_buffer, compressed_type_identifier, message_contents_constructor)
	}
}

impl<E: Debug> Dequeue<E> for Queue<E>
{
	/// Dequeues messages.
	#[inline(always)]
	fn dequeue(&self, terminate: &impl Terminate) -> Result<(), E>
	{
		let message_handlers = self.message_handlers();
		while
		{
			let more_data_to_read = self.magic_ring_buffer.single_reader_read_some_data::<E, _>
			(
				|buffer|
				{
					Message::process_next_message_in_buffer::<Result<(), E>, _>
					(
						buffer,
						|compressed_type_identifier, receiver|
						{
							message_handlers.call_and_drop_in_place(compressed_type_identifier, receiver)
						}
					)
				}
			)?;

			more_data_to_read && terminate.should_continue()
		}
		{
		}

		Ok(())
	}
}

impl<E: Debug> Queue<E>
{
	/// Allocates a new `Queue`.
	#[inline(always)]
	pub fn allocate_from_dev_shm(file_extension: &str, queue_size_in_bytes: usize) -> Result<Arc<Self>, MirroredMemoryMapCreationError>
	{
		Ok
		(
			Arc::new
			(
				Self
				{
					magic_ring_buffer: MagicRingBuffer::allocate_mirrored_and_not_swappable_from_dev_shm(file_extension, queue_size_in_bytes)?,
					message_handlers: Default::default(),
				}
			)
		)
	}

	/// New set of per-thread queues.
	#[inline(always)]
	pub fn queues(logical_cores: &LogicalCores, queue_size_in_bytes: usize) -> Arc<PerLogicalCoreData<Arc<Queue<E>>>>
	{
		Arc::new
		(
			logical_cores.populate_per_logical_core_data(|_logical_core_identifier|
			{
				Queue::allocate_from_dev_shm("queue", queue_size_in_bytes).unwrap()
			})
		)
	}

	#[inline(always)]
	pub(crate) fn message_handlers(&self) -> &mut MutableTypeErasedBoxedFunctionCompressedMap<Result<(), E>>
	{
		unsafe { &mut * self.message_handlers.get() }
	}
}

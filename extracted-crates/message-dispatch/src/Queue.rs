// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A queue of variably-sized messages of different types (eg structs, traits, etc), suitable for many-writer, single consumer usage.
///
/// Ideal for a thread control queue.
#[derive(Debug)]
pub struct Queue(MagicRingBuffer);

impl Queue
{
	/// Allocates a new `Queue`.
	#[inline(always)]
	pub fn allocate_from_dev_shm(file_extension: &str, queue_size_in_bytes: usize) -> Result<Arc<Self>, MirroredMemoryMapCreationError>
	{
		Ok(Arc::new(Self(MagicRingBuffer::allocate_mirrored_and_not_swappable_from_dev_shm(file_extension, queue_size_in_bytes)?)))
	}

	/// Enqueue a message.
	#[inline(always)]
	pub fn enqueue<MessageContents>(&self, compressed_type_identifier: CompressedTypeIdentifier, message_contents_constructor: impl FnOnce(NonNull<MessageContents>))
	{
		Message::enqueue(&self.0, compressed_type_identifier, message_contents_constructor)
	}

	/// Dequeues messages.
	#[inline(always)]
	pub fn dequeue<MessageContents, E>(&self, terminate: &impl Terminate, message_handlers: &mut MutableTypeErasedBoxedFunctionCompressedMap<Result<(), E>>) -> Result<(), E>
	{
		let mut more_data_to_read = true;
		while more_data_to_read && terminate.should_continue()
		{
			more_data_to_read = self.0.single_reader_read_some_data::<_, E>(|buffer| Message::process_next_message_in_buffer::<Result<(), E>>(buffer, message_handlers))?;
		}
		Ok(())
	}
}

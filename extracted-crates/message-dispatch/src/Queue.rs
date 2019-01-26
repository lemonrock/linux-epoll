// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


pub struct Queue
{
	// We actually want to turn this into a vrb!
	// Indeed, it's pretty much essential to be able to go beyond the end of the buffer for memcpy / size calculations.
	allocated_memory: Box<[u8]>,

	queue_memory_pointers: CompareExchangeQueueMemoryPointers,

}

impl Queue
{
	pub fn reserve_bytes(&self, amount: usize)
	{
		let queue_memory_pointers = self.queue_memory_pointers.queue_memory_pointers();

		let last_enqueued_at = queue_memory_pointers.enqueuing_next.relative_pointer;

		let last_dequeued_at = queue_memory_pointers.dequeuing_got_to.relative_pointer;

		if last_enqueued_at > last_dequeued_at
		{
			// remaining head space
			let remaining_head_space = queue_memory_pointers.enqueuing_next.allocation_in_bytes
		}

	}
}

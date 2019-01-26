// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[repr(C, align(16))]
pub(crate) struct CompareExchangeQueueMemoryPointers(AtomicPairOfU64);

impl Default for CompareExchangeQueueMemoryPointers
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(QueueMemoryPointers::default())
	}
}

impl Debug for CompareExchangeQueueMemoryPointers
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "CompareExchangeQueueMemoryPointers {{ queue_memory_pointers: {:?} }}", unsafe { self.queue_memory_pointers() })
	}
}

impl CompareExchangeQueueMemoryPointers
{
	#[inline(always)]
	pub(crate) fn queue_memory_pointers(&self) -> QueueMemoryPointers
	{
		unsafe { transmute(_mm_lddqu_si128(&self.0 as *const AtomicPairOfU64 as *const __m128i)) }
	}

	#[inline(always)]
	pub(crate) fn try_to_update_queue_memory_pointers(&mut self, current_value: QueueMemoryPointers, new_value: QueueMemoryPointers) -> Result<(), QueueMemoryPointers>
	{
		unsafe { transmute(self.0.compare_and_swap(current_value.0, new_value.0)) }
	}
}

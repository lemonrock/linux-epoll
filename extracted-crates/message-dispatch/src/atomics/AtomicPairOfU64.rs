// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[repr(C, align(16))]
struct AtomicPairOfU64(PairOfU64);

impl AtomicPairOfU64
{
	/// Atomically, compares the value currently stored in `self` with `compare_with`.
	///
	/// If they are equal then `store_if_equal` replaces the value of `self` and a result of `Ok(())` is returned.
	/// This is assumed to be the most likely scenario.
	///
	/// If they differ then a result of `Err(current value of self)` is returned.
	#[inline(always)]
	fn compare_and_swap(&mut self, compare_with: PairOfU64, store_if_equal: PairOfU64) -> Result<(), PairOfU64>
	{
		let source = &mut self.0;
		let mut self_was_equal_to_compare_with: bool;
		let mut compare_with_and_update_with_actual_value_if_different = compare_with;

		// `lock cmpxchg16b` does a CAS and sets the `ZF` flag; hence the `setz` assembler.
		// See <https://www.felixcloutier.com/x86/cmpxchg8b:cmpxchg16b>.
		asm!
		(
			"lock cmpxchg16b %source\n\t"
        	"setz %result"
			:
				[source] "+&m" (*source),
				[result] "=q" (self_was_equal_to_compare_with),
				"+d" (compare_with_and_update_with_actual_value_if_different.high_bytes),	// Puts RDX into high bytes.
				"+a" (compare_with_and_update_with_actual_value_if_different.low_bytes)		// Puts RAX into low bytes.
			:
				"c" (store_if_equal.high_bytes),	// Puts high bytes into RCX.
				"b" (store_if_equal.low_bytes)		// Puts low bytes into RBX.
			:
				"cc",
				"memory"
			:
		);

		if likely!(self_was_equal_to_compare_with)
		{
			Ok(())
		}
		else
		{
			Err(compare_with_and_update_with_actual_value_if_different)
		}
	}
}

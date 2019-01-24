// This file is part of terminate. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/terminate/master/COPYRIGHT. No part of terminate, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of terminate. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/terminate/master/COPYRIGHT.


/// Simple implementation of Terminate.
#[derive(Default, Debug, Clone)]
pub struct SimpleTerminate(Arc<AtomicBool>);

impl Terminate for SimpleTerminate
{
	#[inline(always)]
	fn begin_termination(&self)
	{
		self.0.store(true, Relaxed)
	}

	#[inline(always)]
	fn begin_termination_due_to_panic(&self, _panic_info: &PanicInfo)
	{
		self.begin_termination()
	}

	#[inline(always)]
	fn should_finish(&self) -> bool
	{
		self.0.load(Relaxed)
	}
}

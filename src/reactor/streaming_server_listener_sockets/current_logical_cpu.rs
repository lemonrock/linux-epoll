// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[inline(always)]
fn current_logical_cpu() -> i32
{
	#[link_name="c"]
	extern "C"
	{
		fn sched_getcpu() -> c_int;
	}

	let result = unsafe { ::libc::sched_getcpu() };
	if likely!(result != -1)
	{
		result
	}
	else
	{
		panic!("sched_getcpu failed with `{:?}`", errno())
	}
}

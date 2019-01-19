// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A trait that stackful coroutines should implement.
pub trait Coroutine
{
	/// Type of the arguments the coroutine is initially called with, eg `(usize, String)`.
	type StartArguments: Sized;

	/// Type of the arguments the coroutine is resumed with, eg `(u8, Vec<f64>)`.
	type ResumeArguments: Sized;

	/// Type of the result from a yield of the coroutine.
	type Yields: Sized;

	/// Type of the final result from the coroutine.
	type Complete: Sized;

	/// Implement this for the coroutine's behaviour.
	///
	/// Panics inside the coroutine are transferred to the calling thread and raised.
	fn coroutine<'yielder>(start_arguments: Self::StartArguments, yielder: Yielder<'yielder, Self::ResumeArguments, Self::Yields, Self::Complete>) -> Self::Complete;

	#[doc(hidden)]
	#[inline(never)]
	extern "C" fn context_coroutine_wrapper(transfer: Transfer) -> !
	{
		let mut type_safe_transfer = TypeSafeTransfer::<ParentInstructingChild<Self::ResumeArguments>, ChildOutcome<Self::Yields, Self::Complete>>::wrap(transfer);
		let start_child_arguments: Self::StartArguments = type_safe_transfer.start_child_arguments();
		let yielder: Yielder<Self::ResumeArguments, Self::Yields, Self::Complete> = Yielder::new(&mut type_safe_transfer);

		let result = catch_unwind(AssertUnwindSafe(|| Self::coroutine(start_child_arguments, yielder)));
		let child_outcome = ChildOutcome::Complete(result);

		type_safe_transfer.resume_drop_safe(child_outcome);
		unreachable!("Closure has completed")
	}
}

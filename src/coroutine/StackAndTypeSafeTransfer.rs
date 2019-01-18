// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Holds a stack and a type-safe transfer; suitable for the ultimate owner of a coroutine.
///
/// On drop the the closure is killed and the stack is then relinquished.
pub struct StackAndTypeSafeTransfer<S: Sized + Deref<Stack>, C: Coroutine>
{
	stack: S,
	type_safe_transfer: TypeSafeTransfer<ParentInstructingChild<C::ResumeArguments>, ChildOutcome<C::Yields, C::Complete>>,
}

impl<S: Sized + Deref<Stack>, C: Coroutine> StackAndTypeSafeTransfer<S, C>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.type_safe_transfer.resume_drop_safe(ParentInstructingChild::Kill).resume_panic();
	}
}

impl<S: Sized + Deref<Stack>, C: Coroutine> StackAndTypeSafeTransfer<S, C>
{
	/// Creates a new instance and invokes it.
	#[inline(always)]
	pub fn new(stack: S, context_function: ContextFn, initial_data_to_transfer: C::StartArguments) -> Self
	{
		let (stack, type_safe_transfer) = TypeSafeTransfer::new::<C::StartArguments>(stack, C::context_coroutine_wrapper, initial_data_to_transfer);

		Self
		{
			stack,
			type_safe_transfer,
		}
	}

	/// Resumes.
	#[inline(always)]
	pub fn resume(&mut self, arguments: C::ResumeArguments) -> ChildOutcome<C::Yields, C::Complete>
	{
		self.type_safe_transfer.resume_drop_safe(ParentInstructingChild::Resume { resume_arguments: arguments })
	}
}

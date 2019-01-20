// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Holds a stack and a type-safe transfer; suitable for the ultimate owner of a coroutine.
///
/// On drop the the closure is killed and the stack is then relinquished.
#[derive(Debug)]
pub struct StackAndTypeSafeTransfer<S: Sized + Deref<Target=Stack>, C: Coroutine>
{
	stack: S,
	type_safe_transfer: TypeSafeTransfer<ParentInstructingChild<C::ResumeArguments>, ChildOutcome<C::Yields, C::Complete>>,
	child_coroutine_is_active: bool,
}

impl<S: Sized + Deref<Target=Stack>, C: Coroutine> Drop for StackAndTypeSafeTransfer<S, C>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.child_coroutine_is_active
		{
			use self::ChildOutcome::*;

			match self.type_safe_transfer.resume_drop_safe(ParentInstructingChild::Kill)
			{
				WouldLikeToResume(_) => panic!("A killed coroutine MUST NOT return `WouldLikeToResume`"),

				Complete(Err(panic_information)) => resume_unwind(panic_information),

				Complete(Ok(_)) => (),
			}
		}
	}
}

impl<S: Sized + Deref<Target=Stack>, C: Coroutine> StackAndTypeSafeTransfer<S, C>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(stack: S) -> Self
	{
		let (stack, type_safe_transfer) = TypeSafeTransfer::new::<C::StartArguments>(stack, C::context_coroutine_wrapper, ParentInstructingChild::Kill);

		Self
		{
			stack,
			type_safe_transfer,
			child_coroutine_is_active: false,
		}
	}

	/// Starts the coroutine; execution will transfer to the coroutine.
	///
	/// Ownership of `start_arguments` will also transfer.
	///
	/// Returns the data transferred to us after the resume and a guard object to resume the coroutine again (`Left`) or the final result (`Right`).
	///
	/// If the coroutine panicked, this panics.
	#[inline(always)]
	pub fn start(mut self, start_arguments: C::StartArguments) -> Either<(C::Yields, StartedStackAndTypeSafeTransfer<S, C>), C::Complete>
	{
		let child_outcome = self.type_safe_transfer.resume_drop_safe_unsafe_typing(start_arguments);

		use self::ChildOutcome::*;

		match child_outcome
		{
			WouldLikeToResume(yields) => Left((StartedStackAndTypeSafeTransfer::owns(self), yields)),

			Complete(Err(panic_information)) => resume_unwind(panic_information),

			Complete(Ok(complete)) => Right(complete),
		}
	}
}

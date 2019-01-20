// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Holds a stack and a type-safe transfer; suitable for the ultimate owner of a coroutine.
///
/// On drop the the closure is killed and the stack is then relinquished.
pub struct StackAndTypeSafeTransfer<'a, S: Sized + Deref<Target=Stack>, C: Coroutine<'a>>
{
	stack: S,
	type_safe_transfer: TypeSafeTransfer<ChildOutcome<C::Yields, C::Complete>, ParentInstructingChild<C::ResumeArguments>>,
	child_coroutine_is_active: bool,
}

impl<'a, S: Sized + Deref<Target=Stack>, C: Coroutine<'a>> Debug for StackAndTypeSafeTransfer<'a, S, C>
where S: Debug, C::ResumeArguments: Debug, C::Yields: Debug, C::Complete: Debug
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "StackAndTypeSafeTransfer {{ stack: {:?}, type_safe_transfer: {:?}, child_coroutine_is_active: {:?} }}", self.stack, self.type_safe_transfer, self.child_coroutine_is_active)
	}
}

impl<'a, S: Sized + Deref<Target=Stack>, C: Coroutine<'a>> Drop for StackAndTypeSafeTransfer<'a, S, C>
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

impl<'a, S: Sized + Deref<Target=Stack>, C: Coroutine<'a>> StackAndTypeSafeTransfer<'a, S, C>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(stack: S) -> Self
	{
		let (stack, type_safe_transfer) = TypeSafeTransfer::new::<S>(stack, C::context_coroutine_wrapper);

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
	pub fn start(mut self, start_arguments: C::StartArguments) -> Either<(C::Yields, StartedStackAndTypeSafeTransfer<'a, S, C>), C::Complete>
	{
		let child_outcome = self.type_safe_transfer.resume_drop_safe_unsafe_typing(start_arguments);

		use self::ChildOutcome::*;

		match child_outcome
		{
			WouldLikeToResume(yields) => Left((yields, StartedStackAndTypeSafeTransfer::own(self))),

			Complete(Err(panic_information)) => resume_unwind(panic_information),

			Complete(Ok(complete)) => Right(complete),
		}
	}
}

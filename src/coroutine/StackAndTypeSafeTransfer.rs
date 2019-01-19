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

impl<S: Sized + Deref<Stack>, C: Coroutine> Drop for StackAndTypeSafeTransfer<S, C>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let child_outcome = self.type_safe_transfer.resume_drop_safe(ParentInstructingChild::Kill);

		use self::ChildOutcome::*;
		match child_outcome
		{
			Complete(Err(panic_information)) => resume_panic(panic_information),

			WouldLikeToResume(_) => panic!("A killed coroutine MUST NOT return `WouldLikeToResume`"),

			_ => (),
		}
	}
}

impl<S: Sized + Deref<Stack>, C: Coroutine> StackAndTypeSafeTransfer<S, C>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(stack: S, context_function: ContextFn) -> Self
	{
		let (stack, type_safe_transfer) = TypeSafeTransfer::new::<C::StartArguments>(stack, C::context_coroutine_wrapper, ParentInstructingChild::Kill);

		Self
		{
			stack,
			type_safe_transfer,
		}
	}

	/// Starts the coroutine; execution will transfer to the coroutine.
	///
	/// Ownership of `start_arguments` will also transfer.
	#[inline(always)]
	pub fn start(self, start_arguments: C::StartArguments) -> StartedStackAndTypeSafeTransfer<S, C>
	{
		let started = StartedStackAndTypeSafeTransfer
		{
			owns: self,
		};
		let child_outcome = started.start(start_arguments);

	}
}

/// Holds a stack and a type-safe transfer of a started coroutine; suitable for the ultimate owner of a coroutine.
///
/// On drop the the closure is killed and the stack is then relinquished.
pub struct StartedStackAndTypeSafeTransfer<S: Sized + Deref<Stack>, C: Coroutine>
{
	owns: StackAndTypeSafeTransfer<S, C>,
}

impl<S: Sized + Deref<Stack>, C: Coroutine> StartedStackAndTypeSafeTransfer<S, C>
{
	/// Starts the coroutine; execution will transfer to the coroutine.
	///
	/// Ownership of `start_arguments` will also transfer.
	#[inline(always)]
	fn start(self, start_arguments: C::StartArguments) -> ChildOutcome<C::Yields, C::Complete>
	{
		// resume_drop_safe_unsafe_typing
		self.owns.type_safe_transfer.resume_drop_safe_unsafe_typing(start_arguments)
	}

	/// Resumes.
	///
	/// Returns the data transferred to us after the resume.
	#[inline(always)]
	pub fn resume(&mut self, arguments: C::ResumeArguments) -> ChildOutcome<C::Yields, C::Complete>
	{
		self.owns.type_safe_transfer.resume_drop_safe(ParentInstructingChild::Resume(arguments))
	}

	/// Resumes on top.
	///
	/// Returns the data transferred to us after the resume.
	///
	/// It is unlikely you need to use this function.
	#[inline(always)]
	pub fn resume_on_top_drop_safe(&mut self, arguments: C::ResumeArguments, resume_on_top_function: ResumeOnTopFunction) -> ChildOutcome<C::Yields, C::Complete>
	{
		self.owns.type_safe_transfer.resume_on_top_drop_safe(ParentInstructingChild::Resume(arguments), resume_on_top_function)
	}
}

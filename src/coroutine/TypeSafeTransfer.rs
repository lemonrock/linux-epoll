// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Wraps the type of the data being transferred.
pub(crate) struct TypeSafeTransfer<Receive: Sized, Send: Sized>
{
	transfer: Transfer,
	marker: PhantomData<(Receive, Send)>,
}

impl<Receive: Sized, Send: Sized> TypeSafeTransfer<Receive, Send>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new<T>(stack: S, context_function: ContextFn, initial_data_to_transfer: T) -> (S, Self)
	{
		let mut data_to_transfer_drop_safe = Some(data_to_transfer);
		let pointer_out = unsafe { NonNull::new_unchecked(&mut data_to_transfer_drop_safe as *mut Option<T>) };

		let (stack, transfer) = Transfer::new(stack, context_function, pointer_out);
		(stack, Self::wrap(transfer))
	}

	/// Wraps a transfer, eg from first call to `context_function`.
	#[inline(always)]
	pub(crate) fn wrap(transfer: Transfer) -> Self
	{
		Self
		{
			transfer,
			marker: PhantomData,
		}
	}

	/// Resumes with modification in-place; data transferred can implement drop.
	///
	/// Returns the data transferred to us after the resume.
	///
	/// Uses `take()` so that ownership is transferred to the stack that is extant when `resume_drop_safe()` returns.
	#[inline(always)]
	pub(crate) fn resume_drop_safe(&mut self, data_to_transfer: Send) -> Receive
	{
		let mut data_to_transfer_drop_safe = Some(data_to_transfer);
		let pointer_out = unsafe { NonNull::new_unchecked(&mut data_to_transfer_drop_safe as *mut Option<Send>) };

		self.transfer = self.transfer.resume(pointer_out);

		self.take_data()
	}

	/// Resumes on top with modification in-place.
	///
	/// Returns the data transferred to us after the resume.
	///
	/// Uses `take()` so that ownership is transferred to the stack that is extant when `resume_ontop_drop_safe()` returns.
	#[inline(always)]
	pub(crate) fn resume_ontop_drop_safe(&mut self, data_to_transfer: Send, resume_ontop_function: ResumeOntopFn) -> Receive
	{
		let mut data_to_transfer_drop_safe = Some(data_to_transfer);
		let pointer_out = unsafe { NonNull::new_unchecked(&mut data_to_transfer_drop_safe as *mut Option<Send>) };

		self.transfer = self.transfer.resume_ontop(pointer_out, resume_ontop_function);

		self.take_data()
	}

	#[inline(always)]
	pub(crate) fn start_child_arguments<T>(&self) -> T
	{
		self.take_data_unsafe_typing::<T>()
	}

	#[inline(always)]
	fn take_data(&self) -> Receive
	{
		self.take_data_unsafe_typing::<Receive>()
	}

	/// Only call this once per resumption.
	#[inline(always)]
	fn take_data_unsafe_typing<UnsafeT>(&self) -> UnsafeT
	{
		let pointer_in = self.transfer.transferred_data::<NonNull<Option<UnsafeT>>>();
		let data_from_transfer_drop_safe = unsafe { pointer_in.as_mut() };
		data_from_transfer_drop_safe.take().expect("take_data can only be called once per resumption")
	}
}

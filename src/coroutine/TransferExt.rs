// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Extension trait to Transfer.
pub trait TransferExt: Sized
{
	/// Create a new instance with initial data.
	fn new<TD: TransferableData, S: Sized + Deref<Stack>>(stack: S, context_function: ContextFn, initial_data_to_transfer: TD) -> (S, Self);

	/// Get data.
	#[inline(always)]
	fn transferred_data<TD: TransferableData>(&self) -> TD
	{
		TD::from_usize(self.data())
	}

	/// Resume.
	#[inline(always)]
	fn resume<TD: TransferableData>(self, data_to_transfer: TD) -> Self
	{
		unsafe { self.context.resume(data_to_transfer.into_usize()) }
	}

	/// Resume on top.
	#[inline(always)]
	fn resume_ontop<TD: TransferableData>(self, data_to_transfer: TD, resume_ontop_function: ResumeOntopFn) -> Self
	{
		unsafe { self.context.resume_ontop(data_to_transfer.into_usize(), resume_ontop_function) }
	}

	#[doc(hidden)]
	fn data(&self) -> usize;

	#[doc(hidden)]
	fn context(self) -> Context;
}

impl TransferExt for Transfer
{
	#[inline(always)]
	fn new<TD: TransferableData, S: Sized + Deref<Stack>>(stack: S, context_function: ContextFn, initial_data_to_transfer: TD) -> (S, Self)
	{
		let transfer = Transfer::new(unsafe { Context::new(stack.deref(), context_function) }, initial_data_to_transfer.into_usize());
		(stack, transfer)
	}

	#[inline(always)]
	fn data(&self) -> usize
	{
		self.data
	}

	#[inline(always)]
	fn context(&self) -> Context
	{
		self.context
	}
}

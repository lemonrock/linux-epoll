// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A simple structure that wraps up what is required to yield from a coroutine that depends on further input or output data becoming available in order to make progress.
#[derive(Debug)]
pub struct InputOutputYielder<'a>(Yielder<'a, ReactEdgeTriggeredStatus, (), Result<(), CompleteError>>);

impl<'a> InputOutputYielder<'a>
{
	/// Yields to allow for further input or output data to become available.
	#[inline(always)]
	pub fn new(yielder: Yielder<'a, ReactEdgeTriggeredStatus, (), Result<(), CompleteError>>) -> Self
	{
		Self(yielder)
	}

	/// Yields to allow for further input or output data to become available.
	#[inline(always)]
	pub fn await_further_input_or_output_to_become_available(&self) -> Result<ReactEdgeTriggeredStatus, CompleteError>
	{
		self.0.yields((), CompleteError::Killed)
	}
}

// This file is part of message-dispatch. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT. No part of message-dispatch, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of message-dispatch. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT.


/// A map that maps types (not instances) to functions that handle those instances.
///
/// `R` is the result type of calling these functions.
#[derive(Default, Debug, Eq, PartialEq)]
pub struct MutableTypeErasedBoxedFunctionMap<R>(HashMap<TypeId, MutableTypeErasedBoxedFunction<R>>);

impl<R> MutableTypeErasedBoxedFunctionMap<R>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn with_capacity(capacity: usize) -> Self
	{
		Self(HashMap::with_capacity(capacity))
	}

	/// Registers a function to handle mutable references of `Arguments`.
	///
	/// `function` will be moved from the stack to the heap.
	#[inline(always)]
	pub fn register<Function: FnMut(&mut Arguments) -> R + 'static, Arguments: 'static + ?Sized>(&mut self, function: Function)
	{
		let key = TypeId::of::<Arguments>();

		let previous = self.0.insert(key, MutableTypeErasedBoxedFunction::new(function));
		debug_assert!(previous.is_none(), "Registered a function more than once")
	}

	/// Calls the handler with the given arguments.
	///
	/// Returns `Err` if the given type of `Arguments` is not present.
	pub fn call<'map: 'arguments, 'arguments, Arguments: 'static + ?Sized>(&'map mut self, arguments: &'arguments mut Arguments) -> Result<R, ()>
	{
		let key = TypeId::of::<Arguments>();

		match self.0.get_mut(&key)
		{
			None => Err(()),
			Some(function) => Ok(function.call::<Arguments>(arguments)),
		}
	}
}

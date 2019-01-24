// This file is part of message-dispatch. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT. No part of message-dispatch, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of message-dispatch. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT.


/// A wrapper to hold a `Fn(Arguments) -> R` closure which erases the type of `Arguments` so that multiple instances can be created and used as, say, handlers of different messages in maps.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub(crate) struct ImmutableTypeErasedBoxedFunction<R>
{
	boxed_function_pointer: NonNull<BoxedFunctionPointer>,
	call_boxed_function_pointer: fn(NonNull<BoxedFunctionPointer>, NonNull<CallArguments>) -> R,
	drop_boxed_function_pointer: fn(NonNull<BoxedFunctionPointer>),
	#[cfg(debug_assertions)] arguments_type_identifier: TypeId,
}

impl<R> Drop for ImmutableTypeErasedBoxedFunction<R>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		(self.drop_boxed_function_pointer)(self.boxed_function_pointer)
	}
}

impl<R> ImmutableTypeErasedBoxedFunction<R>
{
	/// Creates a new instance, wrapping `function`.
	///
	/// `function` will be moved from the stack to the heap.
	#[inline(always)]
	pub(crate) fn new<Function: Fn(&mut Arguments) -> R, Arguments: 'static + ?Sized>(function: Function) -> Self
	{
		#[inline(always)]
		fn call_boxed_function_pointer<Function: Fn(&mut Arguments) -> R, Arguments: 'static + ?Sized, R>(function: &Function, arguments: &mut Arguments) -> R
		{
			function(arguments)
		}

		#[inline(always)]
		fn drop_boxed_function_pointer<Function: Fn(&mut Arguments) -> R, Arguments: 'static + ?Sized, R>(boxed_function_pointer: NonNull<Function>)
		{
			drop(unsafe { Box::from_raw(boxed_function_pointer.as_ptr()) });
		}

		let call_boxed_function_pointer: for<'r, 's> fn(&'r Function, &'s mut Arguments) -> R = call_boxed_function_pointer::<Function, Arguments, R>;
		let drop_boxed_function_pointer: fn(NonNull<Function>) = drop_boxed_function_pointer::<Function, Arguments, R>;

		ImmutableTypeErasedBoxedFunction
		{
			boxed_function_pointer: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(function)) as *mut BoxedFunctionPointer) },
			call_boxed_function_pointer: unsafe { transmute(call_boxed_function_pointer) },
			drop_boxed_function_pointer: unsafe { transmute(drop_boxed_function_pointer) },
			#[cfg(debug_assertions)] arguments_type_identifier: TypeId::of::<Arguments>(),
		}
	}

	/// A very dangerous method that will fail in subtle yet fatal ways if `arguments` is not the same type used when `new()` was called.
	///
	/// As the whole purpose of this struct is to erase the type of `arguments`, this requirement is not enforced by the type system.
	///
	/// When debug assertions are enabled, a runtime type check is made and will panic if it fails.
	#[inline(always)]
	pub(crate) fn call<'this: 'arguments, 'arguments, Arguments: 'static + ?Sized>(&'this self, arguments: &'arguments mut Arguments) -> R
	{
		#[cfg(debug_assertions)]
		{
			debug_assert_eq!(TypeId::of::<Arguments>(), self.arguments_type_identifier, "Arguments type mismatch")
		}

		let function_pointer: fn(NonNull<BoxedFunctionPointer>, &'arguments mut Arguments) -> R = unsafe { transmute(self.call_boxed_function_pointer) };

		(function_pointer)(self.boxed_function_pointer, arguments)
	}
}

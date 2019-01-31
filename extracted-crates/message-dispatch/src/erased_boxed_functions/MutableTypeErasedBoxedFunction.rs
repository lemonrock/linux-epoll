// This file is part of message-dispatch. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT. No part of message-dispatch, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of message-dispatch. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT.


/// A wrapper to hold a `FnMut(Receiver) -> R` closure which erases the type of `Receiver` so that multiple instances can be created and used as, say, handlers of different messages in maps.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub(crate) struct MutableTypeErasedBoxedFunction<Arguments, Returns>
{
	boxed_function_pointer: NonNull<BoxedFunctionPointer>,
	call_boxed_function_pointer: fn(NonNull<BoxedFunctionPointer>, NonNull<Receiver>, Arguments) -> Returns,
	drop_boxed_function_pointer: fn(NonNull<BoxedFunctionPointer>),
	#[cfg(debug_assertions)] receiver_type_identifier: TypeId,
}

impl<Arguments, Returns> Drop for MutableTypeErasedBoxedFunction<Arguments, Returns>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		(self.drop_boxed_function_pointer)(self.boxed_function_pointer)
	}
}

impl<Arguments, Returns> MutableTypeErasedBoxedFunction<Arguments, Returns>
{
	/// Creates a new instance, wrapping `function`.
	///
	/// `function` will be moved from the stack to the heap.
	#[inline(always)]
	pub(crate) fn new<Function: FnMut(&mut Receiver, Arguments) -> Returns, Receiver: 'static + ?Sized>(function: Function) -> Self
	{
		#[inline(always)]
		fn call_boxed_function_pointer<Function: FnMut(&mut Receiver, Arguments) -> Returns, Receiver: 'static + ?Sized, Arguments, Returns>(function: &mut Function, receiver: &mut Receiver, arguments: Arguments) -> Returns
		{
			function(receiver, arguments)
		}

		#[inline(always)]
		fn drop_boxed_function_pointer<Function: FnMut(&mut Receiver, Arguments) -> Returns, Receiver: 'static + ?Sized, Arguments, Returns>(boxed_function_pointer: NonNull<Function>)
		{
			drop(unsafe { Box::from_raw(boxed_function_pointer.as_ptr()) });
		}

		let call_boxed_function_pointer: for<'r, 's> fn(&'r mut Function, &'s mut Receiver, Arguments) -> Returns = call_boxed_function_pointer::<Function, Receiver, Arguments, Returns>;
		let drop_boxed_function_pointer: fn(NonNull<Function>) = drop_boxed_function_pointer::<Function, Receiver, Arguments, Returns>;

		MutableTypeErasedBoxedFunction
		{
			boxed_function_pointer: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(function)) as *mut BoxedFunctionPointer) },
			call_boxed_function_pointer: unsafe { transmute(call_boxed_function_pointer) },
			drop_boxed_function_pointer: unsafe { transmute(drop_boxed_function_pointer) },
			#[cfg(debug_assertions)] receiver_type_identifier: TypeId::of::<Receiver>(),
		}
	}

	/// A very dangerous method that will fail in subtle yet fatal ways if `receiver` is not the same type used when `new()` was called.
	///
	/// As the whole purpose of this struct is to erase the type of `receiver`, this requirement is not enforced by the type system.
	///
	/// When debug assertions are enabled, a runtime type check is made and will panic if it fails.
	#[inline(always)]
	pub(crate) fn call<'this: 'receiver, 'receiver, Receiver: 'static + ?Sized>(&'this mut self, receiver: &'receiver mut Receiver, arguments: Arguments) -> Returns
	{
		#[cfg(debug_assertions)]
		{
			debug_assert_eq!(TypeId::of::<Receiver>(), self.receiver_type_identifier, "Receiver type mismatch")
		}

		let function_pointer: fn(NonNull<BoxedFunctionPointer>, &'receiver mut Receiver, Arguments) -> Returns = unsafe { transmute(self.call_boxed_function_pointer) };

		(function_pointer)(self.boxed_function_pointer, receiver, arguments)
	}
}

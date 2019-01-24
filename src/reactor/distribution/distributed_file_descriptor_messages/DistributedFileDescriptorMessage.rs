// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


impl DistributedFileDescriptorMessage
{
	// TODO: May not be safely aligned...
	#[inline(always)]
	pub(crate) fn message_contents(&mut self, virtual_tables: &DistributedFileDescriptorMessageContentsVirtualMethodTablesPointerCompression) -> (RawFd, &mut dyn DistributedFileDescriptorMessageContents)
	{
		let data = &mut self.message_contents as *mut VariablySized as *mut ();
		let vtable = virtual_tables.get_virtual_method_table(self.index).to_ptr();

		let fat_pointer: &mut dyn DistributedFileDescriptorMessageContents = unsafe { transmute(TraitObject { data, vtable }) };

		(self.raw_file_descriptor, fat_pointer)
	}
}


/*

	Can we turn tag_bits back into something we can use?

	f

	let type_id: u64 = unsafe { intrinsics::type_id::<T>() };

	let map: HashMap;
	let tag_bits = map.get(type_id).unwrap();

	Every type T has fn get_type_id() (although it needs #![cfg(feature)]


	Rust vtable has 3 special initial usize'd members:-

	- function pointer for drop_in_place()
	- size
	- alignment




*/

extern
{
	type BoxedFunctionPointer;
}

extern
{
	type CallArguments;
}

/// A map that maps types (not instances) to functions that handle those instances.
///
/// `R` is the result type of calling these functions.
#[derive(Default)]
pub struct ImmutableTypeErasedBoxedFunctionMap<R>(HashMap<TypeId, ImmutableTypeErasedBoxedFunction<R>>);

impl<R> ImmutableTypeErasedBoxedFunctionMap<R>
{
	/// Registers a function to handle mutable references of `Arguments`.
	///
	/// `function` will be moved from the stack to the heap.
	#[inline(always)]
	pub fn register<Function: Fn(Arguments) -> R + 'static, Arguments>(&mut self, function: Function)
	{
		let key = Arguments::get_type_id();

		let previous = self.0.insert(key, ImmutableTypeErasedBoxedFunction::new(function));
		debug_assert_eq!(previous, None, "Registered a function more than once")
	}

	/// Calls the handler with the given arguments.
	///
	/// Returns `Err` if the given type of `Arguments` is not present.
	pub fn call<'self: 'arguments, 'arguments, Arguments: 'static + ?Sized>(&'self self, arguments: &'arguments mut Arguments) -> Result<R, ()>
	{
		let key = Arguments::get_type_id();

		match self.0.get(&key)
		{
			None => Err(()),
			Some(function) => Ok(function.call_known_arguments::<Arguments>(arguments)),
		}
	}
}

/// A wrapper to hold a `Fn(Arguments) -> R` closure which erases the type of `Arguments` so that multiple instances can be created and used as, say, handlers of different messages in maps.
pub struct ImmutableTypeErasedBoxedFunction<R>
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
		unsafe
		{
			(self.drop_boxed_function_pointer)(self.boxed_function_pointer)
		}
	}
}

impl<R> ImmutableTypeErasedBoxedFunction<R>
{
	/// Creates a new instance, wrapping `function`.
	///
	/// `function` will be moved from the stack to the heap.
	#[inline(always)]
	pub fn new<Function: Fn(Arguments) -> R, Arguments: 'static + ?Sized>(function: Function) -> Self
	{
		#[inline(always)]
		fn call_boxed_function_pointer<Function: Fn(Arguments), Arguments: 'static + ?Sized>(function: &Function, arguments: &mut Arguments)
		{
			function(arguments)
		}

		#[inline(always)]
		fn drop_boxed_function_pointer<Function: Fn(Arguments), Arguments: 'static + ?Sized>(boxed_function_pointer: NonNull<Function>)
		{
			drop(unsafe { Box::from_raw(boxed_function_pointer.as_ptr()) });
		}

		ImmutableTypeErasedBoxedFunction
		{
			boxed_function_pointer: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(function)) as *mut BoxedFunctionPointer) },
			call_boxed_function_pointer: call_boxed_function_pointer::<Function, Arguments> as fn(&'static BoxedFunctionPointer, &'static mut CallArguments),
			drop_boxed_function_pointer: drop_boxed_function_pointer::<Function, Arguments> as fn(NonNull<BoxedFunctionPointer>),
			#[cfg(debug_assertions)] arguments_type_identifier: Arguments::get_type_id(),
	}
	}

	/// A very dangerous method that will fail in subtle yet fatal ways if `arguments` is not the same type used when `new()` was called.
	///
	/// As the whole purpose of this struct is to erase the type of `arguments`, this requirement is not enforced by the type system.
	///
	/// When debug assertions are enabled, a runtime type check is made and will panic if it fails.
	#[inline(always)]
	pub fn call_known_arguments<'self: 'arguments, 'arguments, Arguments: 'static + ?Sized>(&'self self, arguments: &'arguments mut Arguments) -> R
	{
		#[cfg(debug_assertions)]
		{
			debug_assert_eq!(Arguments::get_type_id(), self.arguments_type_identifier, "Arguments type mismatch")
		}

		let function_pointer: fn(NonNull<BoxedFunctionPointer>, &'arguments mut Arguments) = unsafe { transmute(self.call_boxed_function_pointer) };

		(function_pointer)(self.boxed_function_pointer, arguments)
	}
}




struct ProducerMessage
{
	message_virtual_method_table: TaggedVirtualMethodTablePointer,
	message_contents: VariablySized,
}







trait ProducerMessageContents<I: 'static, P: 'static>: Any + 'static
{
	unsafe fn initialize(self: *mut Self, arguments: I);

	/// After this is called, `drop()` is ***NEVER*** called.
	unsafe fn process(&self, arguments: P);
}

impl<SF: StreamFactory<SD>, SU: StreamUser<SF::S>, SD: SocketData> ProducerMessageContents<SF, SU, SD> for StreamingSocketProducerMessageContents<SF, SU, SD>
{
	#[inline(always)]
	unsafe fn initialize(self: *mut Self, arguments: I)
	{

	}

	#[inline(always)]
	unsafe fn process(&mut self, arguments: P)
	{

	}
}

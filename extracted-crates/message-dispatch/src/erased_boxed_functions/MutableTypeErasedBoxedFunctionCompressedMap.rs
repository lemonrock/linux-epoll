// This file is part of message-dispatch. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT. No part of message-dispatch, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of message-dispatch. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT.


/// A map that maps Types (not instances) to function closures that handle instances of those Types.
///
/// Holds state that lives longer than each call to a function closure.
///
/// Can not hold more than 256 functions, but this restriction makes it perform quicker.
///
/// What do the various type arguments relate to?
///
/// * `Arguments`: This is short-lived data that is passed by move every time to a call to a function closure, `Function`.
/// `Returns` is the result type of calling a function closure, `Function`. Typically it will be `Result<X, Y>`.
/// `Function` is the type of a function closure that takes an instance, `Receiver` ('&mut Self') and arguments `Arguments`.
/// `Receiver` is the instance of a Type.
///
/// `Arguments` and `Returns` have to be the same for all registered function closures.
/// `Function` and `Receiver` are of a different type for each registered function closure.
///
/// A very clever optimization of this structure could produce a jump table at runtime, so reducing indirect calls to direct calls, should this be necessary.
#[derive(Debug, Eq, PartialEq)]
pub struct MutableTypeErasedBoxedFunctionCompressedMap<Arguments: Debug + Copy, Returns: Debug>
{
	compressed_type_identifier_to_function: ArrayVec<[(MutableTypeErasedBoxedFunction<Arguments, Returns>, DropInPlaceFunctionPointer); CompressedTypeIdentifier::Size]>,
	type_identifier_to_compressed_type_identifier: HashMap<TypeId, CompressedTypeIdentifier>,
}

impl<Arguments: Debug + Copy, Returns: Debug> Default for MutableTypeErasedBoxedFunctionCompressedMap<Arguments, Returns>
{
	fn default() -> Self
	{
		Self
		{
			compressed_type_identifier_to_function: ArrayVec::default(),
			type_identifier_to_compressed_type_identifier: HashMap::with_capacity(CompressedTypeIdentifier::Size),
		}
	}
}

impl<Arguments: Debug + Copy, Returns: Debug> Register<Arguments, Returns> for MutableTypeErasedBoxedFunctionCompressedMap<Arguments, Returns>
{
	#[inline(always)]
	fn enter_into_the_register<Function: FnMut(&mut Receiver, Arguments) -> Returns + 'static, Receiver: 'static>(&mut self, function: Function) -> CompressedTypeIdentifier
	{
		let virtual_method_table_pointer = VirtualMethodTablePointer::from_any::<Receiver>();
		let drop_in_place_function_pointer = virtual_method_table_pointer.drop_in_place_function_pointer();

		let type_identifier = TypeId::of::<Receiver>();

		let length = self.compressed_type_identifier_to_function.len();
		debug_assert_ne!(length, CompressedTypeIdentifier::Size, "No more space available");

		let next_type_identifier = CompressedTypeIdentifier(length as u8);
		let previous = self.type_identifier_to_compressed_type_identifier.insert(type_identifier, next_type_identifier);
		debug_assert!(previous.is_none(), "Duplicate registration");

		self.compressed_type_identifier_to_function.push((MutableTypeErasedBoxedFunction::new(function), drop_in_place_function_pointer));

		next_type_identifier
	}
}

impl<Arguments: Debug + Copy, Returns: Debug> MutableTypeErasedBoxedFunctionCompressedMap<Arguments, Returns>
{
	/// Calls the function registered for this compressed type identifier.
	///
	/// Panics if no function is registered (only if `debug_assertions` are configured).
	#[inline(always)]
	pub fn call_and_drop_in_place<'map: 'receiver, 'receiver, Receiver: 'static + ?Sized>(&'map mut self, compressed_type_identifier: CompressedTypeIdentifier, receiver: &'receiver mut Receiver, arguments: Arguments) -> Returns
	{
		let (function, drop_in_place_function_pointer) = self.entry(compressed_type_identifier);
		let result = function.call::<Receiver>(receiver, arguments);
		drop_in_place_function_pointer(unsafe { NonNull::new_unchecked(receiver as *mut Receiver as *mut ()) });
		result
	}

	/// Calls the drop in place function registered for this compressed type identifier.
	///
	/// Panics if no function is registered (only if `debug_assertions` are configured).
	#[inline(always)]
	pub fn drop_in_place<'map: 'receiver, 'receiver, Receiver: 'static + ?Sized>(&'map mut self, compressed_type_identifier: CompressedTypeIdentifier, receiver: &'receiver mut Receiver)
	{
		let (_function, drop_in_place_function_pointer) = self.entry(compressed_type_identifier);
		drop_in_place_function_pointer(unsafe { NonNull::new_unchecked(receiver as *mut Receiver as *mut ()) })
	}

	/// Calls the function registered for this compressed type identifier.
	///
	/// Panics if no function is registered (only if `debug_assertions` are configured).
	#[inline(always)]
	fn entry(&mut self, compressed_type_identifier: CompressedTypeIdentifier) -> &mut (MutableTypeErasedBoxedFunction<Arguments, Returns>, DropInPlaceFunctionPointer)
	{
		let index = compressed_type_identifier.index();

		if cfg!(debug_assertions)
		{
			self.compressed_type_identifier_to_function.get_mut(index).unwrap()
		}
		else
		{
			unsafe { self.compressed_type_identifier_to_function.get_unchecked_mut(index) }
		}
	}

	/// Finds a compressed type identifier for a given type.
	///
	/// Slow as it uses a HashMap look up; do not do this on the critical path.
	#[inline(always)]
	pub fn find_compressed_type_identifier<Receiver: 'static>(&self) -> Option<CompressedTypeIdentifier>
	{
		let type_identifier = TypeId::of::<Receiver>();
		self.find_compressed_type_identifier_from_type_identifier(type_identifier)
	}

	/// Finds a compressed type identifier for a given type identifier (`TypeId`).
	///
	/// Slow as it uses a HashMap look up; do not do this on the critical path.
	#[inline(always)]
	pub fn find_compressed_type_identifier_from_type_identifier(&self, type_identifier: TypeId) -> Option<CompressedTypeIdentifier>
	{
		self.type_identifier_to_compressed_type_identifier.get(&type_identifier).map(|value| *value)
	}
}

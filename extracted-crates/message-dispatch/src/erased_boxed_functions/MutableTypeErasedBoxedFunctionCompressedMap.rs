// This file is part of message-dispatch. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT. No part of message-dispatch, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of message-dispatch. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT.


/// A map that maps types (not instances) to functions that handle those instances.
///
/// Can not hold more than 256 functions, but this restriction makes it perform quicker.
///
/// `R` is the result type of calling these functions.
///
/// A very clever optimization of this structure could produce a jump table at runtime, so reducing indirect calls to direct calls, should this be necessary.
#[derive(Debug, Eq, PartialEq)]
pub struct MutableTypeErasedBoxedFunctionCompressedMap<R>
{
	compressed_type_identifier_to_function: ArrayVec<[(MutableTypeErasedBoxedFunction<R>, DropInPlaceFunctionPointer); CompressedTypeIdentifier::Size]>,
	type_identifier_to_compressed_type_identifier: HashMap<TypeId, CompressedTypeIdentifier>,
}

impl<R> Default for MutableTypeErasedBoxedFunctionCompressedMap<R>
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

impl<R> Register<R> for MutableTypeErasedBoxedFunctionCompressedMap<R>
{
	#[inline(always)]
	fn enter_into_the_register<Function: FnMut(&mut Receiver) -> R + 'static, Receiver: 'static>(&mut self, function: Function) -> CompressedTypeIdentifier
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

impl<R> MutableTypeErasedBoxedFunctionCompressedMap<R>
{

	/// Calls the function registered for this compressed type identifier.
	///
	/// Panics if no function is registered (only if `debug_assertions` are configured).
	#[inline(always)]
	pub fn call_and_drop_in_place<'map: 'receiver, 'receiver, Receiver: 'static + ?Sized>(&'map mut self, compressed_type_identifier: CompressedTypeIdentifier, receiver: &'receiver mut Receiver) -> R
	{
		let (function, drop_in_place_function_pointer) = self.entry(compressed_type_identifier);
		let result = function.call::<Receiver>(receiver);
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
	fn entry(&mut self, compressed_type_identifier: CompressedTypeIdentifier) -> &mut (MutableTypeErasedBoxedFunction<R>, DropInPlaceFunctionPointer)
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

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
pub struct ImmutableTypeErasedBoxedFunctionCompressedMap<R>
{
	compressed_type_identifier_to_function: ArrayVec<[ImmutableTypeErasedBoxedFunction<R>; CompressedTypeIdentifier::Size]>,
	type_identifier_to_compressed_type_identifier: HashMap<TypeId, CompressedTypeIdentifier>,
}

impl<R> Default for ImmutableTypeErasedBoxedFunctionCompressedMap<R>
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

impl<R> ImmutableTypeErasedBoxedFunctionCompressedMap<R>
{
	/// Registers a handler and returns a `CompressedTypeIdentifier` to refer to it.
	///
	/// Panics if the handler has already been registered (only if `debug_assertions` are configured).
	///
	/// Panics if there is not space for more handlers (only 256 handlers are allowed) (only if `debug_assertions` are configured).
	#[inline(always)]
	pub fn register<Function: Fn(&mut Arguments) -> R + 'static, Arguments: 'static + ?Sized>(&mut self, function: Function) -> CompressedTypeIdentifier
	{
		let type_identifier = TypeId::of::<Arguments>();

		let length = self.compressed_type_identifier_to_function.len();
		debug_assert_ne!(length, CompressedTypeIdentifier::Size, "No more space available");

		let next_type_identifier = CompressedTypeIdentifier(length as u8);
		let previous = self.type_identifier_to_compressed_type_identifier.insert(type_identifier, next_type_identifier);
		debug_assert!(previous.is_none(), "Duplicate registration");

		self.compressed_type_identifier_to_function.push(ImmutableTypeErasedBoxedFunction::new(function));

		next_type_identifier
	}

	/// Calls the function registered for this compressed type identifier.
	///
	/// Panics if no function is registered (only if `debug_assertions` are configured).
	#[inline(always)]
	pub fn call<'map: 'arguments, 'arguments, Arguments: 'static + ?Sized>(&self, compressed_type_identifier: CompressedTypeIdentifier, arguments: &'arguments mut Arguments) -> R
	{
		let index = compressed_type_identifier.index();

		let function = if cfg!(debug_assertions)
		{
			self.compressed_type_identifier_to_function.get(index).unwrap()
		}
		else
		{
			unsafe { self.compressed_type_identifier_to_function.get_unchecked(index) }
		};
		function.call::<Arguments>(arguments)
	}

	/// Finds a compressed type identifier for a given type.
	///
	/// Slow as it uses a HashMap look up; do not do this on the critical path.
	#[inline(always)]
	pub fn find_compressed_type_identifier<Arguments: 'static + ?Sized>(&self) -> Option<CompressedTypeIdentifier>
	{
		let type_identifier = TypeId::of::<Arguments>();
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

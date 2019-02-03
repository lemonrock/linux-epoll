// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Holds arenas of different types.
#[derive(Debug)]
pub(crate) struct Arenas<T: Terminate>
{
	terminate: PhantomData<T>,

	reactor_compressed_type_lookup_table: HashMap<TypeId, (CompressedTypeIdentifier, TypeId)>,
	arenas: ArrayVec<[(NonNull<UnsizedArena>, UnsizedArenaDropInPlaceFunctionPointer, UnsizedReactFunctionPointer<T>); CompressedTypeIdentifier::Size]>,

	last_reactor_type_identifier_looked_up: Cell<TypeId>,
	last_unsized_arena_and_reactor_compressed_type_identifier_for_last_reactor_type_identifier_looked_up: Cell<(NonNull<UnsizedArena>, CompressedTypeIdentifier)>,
}

impl<T: Terminate> Drop for Arenas<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		for (unsized_arena, arena_drop_in_place_function_pointer, _) in self.arenas.drain(..)
		{
			arena_drop_in_place_function_pointer(unsized_arena)
		}
	}
}

impl<T: Terminate> Default for Arenas<T>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			terminate: PhantomData,

			reactor_compressed_type_lookup_table: HashMap::with_capacity(CompressedTypeIdentifier::Size),
			arenas: ArrayVec::new(),

			last_reactor_type_identifier_looked_up: Cell::new(Self::empty_type_identifier()),
			last_unsized_arena_and_reactor_compressed_type_identifier_for_last_reactor_type_identifier_looked_up: Cell::new(unsafe { uninitialized() }),
		}
	}
}

impl<T: Terminate> Arenas<T>
{
	#[inline(always)]
	fn empty_type_identifier() -> TypeId
	{
		unsafe { zeroed() }
	}

	/// Register an arena.
	///
	/// It is not permissible to register multiple arenas for the same type of `R`.
	pub(crate) fn register<A: Arena<R> + 'static, R: Reactor + 'static>(&mut self, arena: A) -> CompressedTypeIdentifier
	{
		let arena_type_identifier = TypeId::of::<A>();
		let reactor_type_identifier = TypeId::of::<R>();
		let reactor_compressed_type_identifier = CompressedTypeIdentifier::from(self.arenas.len() as u8);

		let previous = self.reactor_compressed_type_lookup_table.insert(reactor_type_identifier, (reactor_compressed_type_identifier, arena_type_identifier));
		debug_assert!(previous.is_none(), "Reactor type already registered!");

		let unsized_arena = unsafe { NonNull::new_unchecked(arena.to_non_null().as_ptr() as *mut UnsizedArena) };

		let sized_arena_drop_in_place_function_pointer: fn(NonNull<A>) = A::drop_from_non_null;
		let unsized_arena_drop_in_place_function_pointer: UnsizedArenaDropInPlaceFunctionPointer = unsafe { transmute(sized_arena_drop_in_place_function_pointer) };

		let sized_react_function_pointer: for<'event_poll, 'terminate> fn(&'event_poll EventPoll<T>, NonNull<A>, EventPollToken, EPollEventFlags, &'terminate T) -> Result<(), String> = EventPoll::<T>::react_callback::<A, R>;
		let unsized_react_function_pointer: UnsizedReactFunctionPointer<T> = unsafe { transmute(sized_react_function_pointer) };

		self.arenas.push((unsized_arena, unsized_arena_drop_in_place_function_pointer, unsized_react_function_pointer));

		reactor_compressed_type_identifier
	}

	/// Gets an arena.
	///
	/// Assumes the `reactor_compressed_type_identifier` is correct.
	#[inline(always)]
	pub(crate) fn get_arena<A: Arena<R> + 'static, R: Reactor + 'static>(&self, reactor_compressed_type_identifier: CompressedTypeIdentifier) -> &A
	{
		let unsized_arena = self.get_unsized_arena(reactor_compressed_type_identifier);
		unsafe { & * (unsized_arena.as_ptr() as *const _ as *const A)  }
	}

	#[inline(always)]
	pub(crate) fn get_unsized_arena_and_react_function_pointer(&self, reactor_compressed_type_identifier: CompressedTypeIdentifier) -> (NonNull<UnsizedArena>, UnsizedReactFunctionPointer<T>)
	{
		let value: u8 = reactor_compressed_type_identifier.into();
		let index = value as usize;

		let (unsized_arena, _, react_function_pointer) = if cfg!(debug_assertions)
		{
			self.arenas[index]
		}
		else
		{
			*self.arenas.get_unchecked(index)
		};

		(unsized_arena, react_function_pointer)
	}

	#[inline(always)]
	pub(crate) fn get_unsized_arena(&self, reactor_compressed_type_identifier: CompressedTypeIdentifier) -> NonNull<UnsizedArena>
	{
		let value: u8 = reactor_compressed_type_identifier.into();

		let index = value as usize;
		if cfg!(debug_assertions)
		{
			self.arenas[index].0
		}
		else
		{
			self.arenas.get_unchecked(index).0
		}
	}

	/// Gets and arena and the associated reactor type identifier, or panics if not present (or in debug, the Arena type mismatches).
	///
	/// Optimized so that repeated look ups of the same Reactor type are very fast.
	#[inline(always)]
	pub(crate) fn get_arena_and_reactor_compressed_type_identifier<A: Arena<R> + 'static, R: Reactor + 'static>(&self) -> (&A, CompressedTypeIdentifier)
	{
		let reactor_type_identifier = TypeId::of::<R>();
		debug_assert_ne!(reactor_type_identifier, Self::empty_type_identifier(), "Oh dear; we can't use a zeroed type identifier as a sentinel");

		let (unsized_arena, reactor_compressed_type_identifier) = if likely!(self.last_reactor_type_identifier_looked_up.get() == reactor_type_identifier)
		{
			self.last_unsized_arena_and_reactor_compressed_type_identifier_for_last_reactor_type_identifier_looked_up.get()
		}
		else
		{
			let (reactor_compressed_type_identifier, arena_type_identifier) = self.reactor_compressed_type_lookup_table.get(&reactor_type_identifier).expect("Reactor was never registered");
			debug_assert_eq!(*arena_type_identifier, TypeId::of::<A>(), "Reactor was registered for a different Arena type");

			let pair = (self.get_unsized_arena(*reactor_compressed_type_identifier), *reactor_compressed_type_identifier);

			self.last_reactor_type_identifier_looked_up.set(reactor_type_identifier);
			self.last_unsized_arena_and_reactor_compressed_type_identifier_for_last_reactor_type_identifier_looked_up.set(pair);

			pair
		};

		(unsafe { & * (unsized_arena.as_ptr() as *const _ as *const A)  }, reactor_compressed_type_identifier)
	}
}

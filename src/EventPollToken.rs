// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Represents a token registered with event poll (epoll).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
struct EventPollToken(u64);

impl EventPollToken
{
	/// Extracts the Reactor compressed type identifier from an event poll token.
	#[inline(always)]
	pub(crate) fn reactor_compressed_type_identifier(self) -> CompressedTypeIdentifier
	{
		CompressedTypeIdentifier::from(((self.0 & 0xFF00_0000_0000_0000) >> 56) as u8)
	}

	/// Extracts the arena index from an event poll token.
	#[inline(always)]
	pub(crate) fn arena_index(self) -> ArenaIndex
	{
		(self.0 & !0xFF00_0000_0000_0000) as ArenaIndex
	}

	/// Creates a new instance.
	#[inline(always)]
	pub(crate) fn new(reactor_compressed_type_identifier: CompressedTypeIdentifier, arena_index: ArenaIndex) -> Self
	{
		let value: u8 = reactor_compressed_type_identifier.into();
		Self((value as u64) << 56 | (arena_index as u64))
	}
}

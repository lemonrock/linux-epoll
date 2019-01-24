// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Represents a token registered with event poll (epoll).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventPollToken(pub(crate) u64);

impl BitAnd<u64> for EventPollToken
{
	type Output = u64;

	#[inline(always)]
	fn bitand(self, rhs: u64) -> Self::Output
	{
		self.0 & rhs
	}
}

impl EventPollToken
{
	const EventPollTokenSize: u64 = Self::size_in_bits::<u64>();

	const FileDescriptorKindSize: u64 = 5;

	const FileDescriptorKindShift: u64 = Self::EventPollTokenSize - Self::FileDescriptorKindSize;

	const FileDescriptorKindMask: u64 = Self::bitmask(Self::FileDescriptorKindSize, Self::FileDescriptorKindShift);

	const RawFileDescriptorSize: u64 = Self::size_in_bits::<RawFd>();

	const RawFileDescriptorShift: u64 = Self::FileDescriptorKindShift - Self::RawFileDescriptorSize;

	const RawFileDescriptorMask: u64 = Self::bitmask(Self::RawFileDescriptorSize, Self::RawFileDescriptorShift);

	const ArenaIndexMask: u64 = !(Self::FileDescriptorKindMask | Self::RawFileDescriptorMask);

	#[inline(always)]
	const fn size_in_bits<T: Sized>() -> u64
	{
		(size_of::<T>() as u64) * 8
	}

	#[inline(always)]
	const fn bitmask(number_of_bits: u64, shift: u64) -> u64
	{
		#[inline(always)]
		const fn set_bits(number_of_bits: u64) -> u64
		{
			(1 << number_of_bits) - 1
		}

		set_bits(number_of_bits) << shift
	}

	/// Extracts the file descriptor kind from an event poll token.
	#[inline(always)]
	pub(crate) fn file_descriptor_kind(self) -> FileDescriptorKind
	{
		let value = ((self & Self::FileDescriptorKindMask) >> Self::FileDescriptorKindShift) as u8;
		unsafe { transmute(value) }
	}

	/// Extracts the file descriptor kind from an event poll token.
	#[inline(always)]
	pub(crate) fn raw_file_descriptor(self) -> RawFd
	{
		((self & Self::RawFileDescriptorMask) >> Self::RawFileDescriptorShift) as RawFd
	}

	/// Extracts the arena index from an event poll token.
	#[inline(always)]
	pub(crate) fn arena_index(self) -> ArenaIndex
	{
		(self & Self::ArenaIndexMask) as ArenaIndex
	}

	/// Extracts the file descriptor kind from an event poll token.
	#[inline(always)]
	pub(crate) fn new(file_descriptor_kind: FileDescriptorKind, raw_file_descriptor: &impl AsRawFd, arena_index: ArenaIndex) -> Self
	{
		Self(((file_descriptor_kind as u8 as u64) << Self::FileDescriptorKindShift) | ((raw_file_descriptor.as_raw_fd() as u64) << Self::RawFileDescriptorShift) | (arena_index as u64))
	}
}

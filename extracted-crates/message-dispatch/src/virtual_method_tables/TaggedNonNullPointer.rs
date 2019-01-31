// This file is part of message-dispatch. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT. No part of message-dispatch, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of message-dispatch. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT.


/// A pointer with a tag value for unused bits in the raw pointer.
///
/// Always a total of 64-bits in size, even on on 32-bit systems.
///
/// Consequently, on both Arm32, AArch64 and x86-64 systems, a total of 19 bits is available (in theory, 20 bits are available on x86-64 but this is not made available; if a file descriptor follows this value, an additional bit could be snaffled as raw file descriptors are always unsigned - the top bit is only ever used to signal failure).
/// Additionally, the values `0` to `2^19 - 1` can be special-cased as the virtual method table pointer can never be null.
///
/// Notes:-
///
/// * On a 32-bit system all pointers are normally 32-bit aligned (if they come from the stack or malloc); indeed on Arm systems, they have to be.
/// * On a 64-bit system all pointers are normally 64-bit aligned (if they come from the stack or malloc).
/// * On AArch64 and x86-64 currently the top 16-bits are not used (virtual addresses are a maximum of 48-bit).
/// * On x86-64 (and ?AArch64) the bit 46 (if counting from zero) is always a fixed value, either 0 (for user space) or 1 (for the kernel).
/// * On x86-64, bits 63:48 are a sign-extension of bit 47 - ie all 1s or all 0s.
/// * On all systems, a virtual method table can never be the null pointer (all zeros).
/// * However, 52-bit pointers are proposed for ARMv8.2 and 56-bit pointers by Intel for x86-64 (<https://software.intel.com/sites/default/files/managed/2b/80/5-level_paging_white_paper.pdf>), but these features are a way off and will require opt-in by Linux.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TaggedNonNullPointer(u64);

impl TaggedNonNullPointer
{
	#[cfg(target_pointer_width = "64")] const AlignedPointerShift: u64 = 3;
	#[cfg(target_pointer_width = "32")] const AlignedPointerShift: u64 = 2;

	#[cfg(target_pointer_width = "64")] const PointerOverheadInBitsWhen64BitEncoded: u64 = 16 + Self::AlignedPointerShift;
	#[cfg(target_pointer_width = "32")] const PointerOverheadInBitsWhen64BitEncoded: u64 = 32 + Self::AlignedPointerShift;

	const MaximumTagBitsValue: u64 = (1 << Self::PointerOverheadInBitsWhen64BitEncoded) - 1;

	const TagBitsShift: u64 = 64 - Self::PointerOverheadInBitsWhen64BitEncoded;

	const TagBitsMask: u64 = Self::MaximumTagBitsValue << Self::TagBitsShift;

	/// This logic assumes the `non_null_pointer` is aligned to the target architecture's native alignment.
	///
	/// Tag bits must not exceed `2^19 - 1` to be platform-portable.
	#[inline(always)]
	pub fn tag(tag_bits: u32, non_null_pointer: NonNull<()>) -> Self
	{
		Self::new(tag_bits, non_null_pointer.into())
	}

	/// This assumes no virtual method table pointer.
	///
	/// Tag bits must not exceed `2^19 - 1` to be platform-portable.
	#[inline(always)]
	pub fn special(tag_bits: u32) -> Self
	{
		Self::new(tag_bits, null_mut())
	}

	#[inline(always)]
	fn new(tag_bits: u32, pointer: *mut ()) -> Self
	{
		let tag_bits = tag_bits as u64;
		debug_assert!(tag_bits as u64 <= Self::MaximumTagBitsValue, "tag_bits `{}` is larger than the maximum possible `{}`", tag_bits, Self::MaximumTagBitsValue);
		let tag_bits_shifted = tag_bits << Self::TagBitsShift;

		let pointer_downshifted = (pointer as usize as u64) >> Self::AlignedPointerShift;

		Self(tag_bits_shifted | pointer_downshifted)
	}

	/// Tag bits.
	#[inline(always)]
	pub fn tag_bits(self) -> u32
	{
		((self.0 & Self::TagBitsMask) >> Self::TagBitsShift) as u32
	}

	/// Virtual Method Table pointer (or None if special).
	#[inline(always)]
	pub fn non_null_pointer<T>(self) -> Option<NonNull<T>>
	{
		let raw_pointer = ((self.0 & !Self::TagBitsMask) << Self::AlignedPointerShift) as usize as *mut ();
		if raw_pointer.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { NonNull::new_unchecked(raw_pointer as *mut T) })
		}
	}
}

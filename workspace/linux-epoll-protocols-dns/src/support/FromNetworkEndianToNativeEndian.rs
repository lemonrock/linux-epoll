// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


pub(crate) trait FromNetworkEndianToNativeEndian
{
	type NumericType;

	#[inline(always)]
	fn from_network_endian_to_native_endian(self) -> Self::NumericType;
}

impl FromNetworkEndianToNativeEndian for [u8; size_of::<u16>()]
{
	type NumericType = u16;

	#[inline(always)]
	fn from_network_endian_to_native_endian(self) -> Self::NumericType
	{
		Self::NumericType::from_be_bytes(self)
	}
}

impl FromNetworkEndianToNativeEndian for [u8; size_of::<u32>()]
{
	type NumericType = u32;

	#[inline(always)]
	fn from_network_endian_to_native_endian(self) -> Self::NumericType
	{
		Self::NumericType::from_be_bytes(self)
	}
}

impl FromNetworkEndianToNativeEndian for [u8; size_of::<u64>()]
{
	type NumericType = u64;

	#[inline(always)]
	fn from_network_endian_to_native_endian(self) -> Self::NumericType
	{
		Self::NumericType::from_be_bytes(self)
	}
}

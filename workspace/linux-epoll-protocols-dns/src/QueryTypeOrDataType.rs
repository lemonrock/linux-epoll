// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[repr(C, packed)]
#[derive(Copy, Clone)]
union QueryTypeOrDataType
{
	query_type: QueryType,
	resource_record_data_type: DataType,
	bytes: [u8; 2],
}

impl Debug for QueryTypeOrDataType
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		unsafe { self.bytes.fmt(f) }
	}
}

impl PartialEq for QueryTypeOrDataType
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.bytes == other.bytes }
	}
}

impl Eq for QueryTypeOrDataType
{
}

impl PartialOrd for QueryTypeOrDataType
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.bytes.partial_cmp(&other.bytes) }
	}
}

impl Ord for QueryTypeOrDataType
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.bytes.cmp(&other.bytes) }
	}
}

impl Hash for QueryTypeOrDataType
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		unsafe { self.bytes.hash(state) }
	}
}

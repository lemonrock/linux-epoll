// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


extern
{
	type QuerySectionEntry;
}

impl QuerySectionEntry
{
	/// `QNAME` field.
	#[inline(always)]
	fn name(&self) -> &Name
	{
		unsafe { & * (self as *const Self as *const Name) }
	}

	/// `QNAME` field.
	#[inline(always)]
	fn name_mutable(&mut self) -> &mut Name
	{
		unsafe { &mut * (self as *mut Self as *mut Name) }
	}

	/// `QTYPE` field.
	#[inline(always)]
	fn query_type_or_data_type(&self) -> QueryTypeOrDataType
	{
		self.query_section_entry_footer().query_type()
	}

	/// `QTYPE` field.
	#[inline(always)]
	fn set_query_type_or_data_type(&self, query_type_or_data_type: QueryTypeOrDataType)
	{
		self.query_section_entry_footer_mutable().set_query_type(query_type_or_data_type)
	}

	/// `QCLASS` field.
	#[inline(always)]
	fn query_class(&self) -> Result<QueryClass, DnsProtocolError>
	{
		self.query_section_entry_footer().query_class()
	}

	/// `QCLASS` field.
	#[inline(always)]
	fn set_query_class(&self, query_class: QueryClass)
	{
		self.query_section_entry_footer_mutable().set_query_class(query_class)
	}

	#[inline(always)]
	fn query_section_entry_footer(&self) -> &QuerySectionEntryFooter
	{
		unimplemented!();
	}

	#[inline(always)]
	fn query_section_entry_footer_mutable(&mut self) -> &mut QuerySectionEntryFooter
	{
		unimplemented!();
	}
}
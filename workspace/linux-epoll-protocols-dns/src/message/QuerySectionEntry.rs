// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


pub(crate) struct QuerySectionEntry;

impl QuerySectionEntry
{
	#[inline(always)]
	pub(crate) fn parse_response<'message>(&'message mut self, parsed_labels: &mut ParsedLabels, end_of_message_pointer: usize, request_query_identification: RequestQueryIdentification) -> Result<(usize, DataType), DnsProtocolError>
	{
		let (qname, end_of_qname_pointer) = self.name().parse_without_compression_but_register_labels_for_compression(parsed_labels, end_of_message_pointer)?;

		let query_class = self.query_class(end_of_qname_pointer)?;
		debug_assert_eq!(query_class, QueryClass::Internet);

		let data_type = self.data_type(end_of_qname_pointer);
		request_query_identification.matches(qname, data_type)?;

		Ok((Self::end_of_query_section(end_of_qname_pointer), data_type))
	}

	#[inline(always)]
	fn data_type(&self, end_of_name_pointer: usize) -> DataType
	{
		self.query_type_or_data_type(end_of_name_pointer).data_type()
	}

	#[inline(always)]
	const fn end_of_query_section(end_of_qname_pointer: usize) -> usize
	{
		const QueryTypeOrDataTypeSize: usize = 2;
		const QueryClassSize: usize = 2;
		end_of_qname_pointer + QueryTypeOrDataTypeSize + QueryClassSize
	}

	/// `QNAME` field.
	#[inline(always)]
	pub(crate) fn name(&mut self) -> &mut Name
	{
		self.unsafe_cast_mut::<Name>()
	}

	/// `QTYPE` field.
	#[inline(always)]
	pub(crate) fn query_type_or_data_type(&self, end_of_name_pointer: usize) -> QueryTypeOrDataType
	{
		self.query_section_entry_footer(end_of_name_pointer).query_type_or_data_type()
	}

	/// `QCLASS` field.
	#[inline(always)]
	pub(crate) fn query_class(&self, end_of_name_pointer: usize) -> Result<QueryClass, DnsProtocolError>
	{
		self.query_section_entry_footer(end_of_name_pointer).query_class()
	}

	#[inline(always)]
	fn query_section_entry_footer(&self, end_of_name_pointer: usize) -> &QuerySectionEntryFooter
	{
		end_of_name_pointer.unsafe_cast::<QuerySectionEntryFooter>()
	}
}

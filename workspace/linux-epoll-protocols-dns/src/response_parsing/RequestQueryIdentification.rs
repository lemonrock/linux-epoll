// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct RequestQueryIdentification
{
	data_type: DataType,
	query_name_labels_excluding_root: (u8, Vec<Vec<u8>>),
}

impl RequestQueryIdentification
{
	pub(crate) fn matches<'a>(&self, qname: WithoutCompressionParsedName<'a>, data_type: DataType) -> Result<(), DnsProtocolError>
	{
		if unlikely!(self.data_type != data_type)
		{
			return Err(ResponseWasForADifferentDataType)
		}

		if unlikely!(self.query_name_labels_excluding_root.0 != qname.name_length)
		{
			return Err(ResponseWasForADifferentName)
		}

		if unlikely!(self.query_name_labels_excluding_root.1.len() != qname.number_of_labels as usize)
		{
			return Err(ResponseWasForADifferentName)
		}

		let mut index = 0;
		for query_label in qname
		{
			let expected_label = unsafe { self.query_name_labels_excluding_root.get_unchecked(index) };
			if unlikely!(&expected_label[..] != query_label)
			{
				return Err(ResponseWasForADifferentName)
			}

			index += 1;
		}

		Ok(())
	}
}

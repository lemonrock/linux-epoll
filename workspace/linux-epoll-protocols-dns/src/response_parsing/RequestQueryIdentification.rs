// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct RequestQueryIdentification
{
	expected_data_type: DataType,
	expected_query_name: UncompressedName,
}

impl RequestQueryIdentification
{
	pub(crate) fn matches<'message>(&self, received_data_type: DataType, received_query_name: WithoutCompressionParsedName<'message>) -> Result<(), DnsProtocolError>
	{
		if unlikely!(self.data_type != received_data_type)
		{
			return Err(ResponseWasForADifferentDataType)
		}

		let expected_query_name = self.expected_query_name.name();

		if likely!(expected_query_name == received_query_name)
		{
			Ok(())
		}
		else
		{
			Err(ResponseWasForADifferentName)
		}
	}
}

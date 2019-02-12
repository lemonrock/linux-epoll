// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
struct QuerySectionEntryFooter
{
	/// A two octet code which specifies the type of the query.
	///
	/// The values for this field include all codes valid for a `TYPE` field, together with some more general codes which  can match more than one type of Resource Record (RR).
	qtype: [u8; 2],

	/// A two octet code that specifies the class of the query.
	///
	/// For example, the `QCLASS` field is `IN` for the Internet.
	qclass: [u8; 2],
}

impl QuerySectionEntryFooter
{
	#[inline(always)]
	fn query_type(&self) -> [u8; 2]
	{
		self.qtype
	}

	#[inline(always)]
	fn set_query_type(&mut self, question_type: [u8; 2])
	{
		self.qtype = question_type;
	}

	#[inline(always)]
	fn query_class(&self) -> Result<QueryClass, DnsProtocolError>
	{
		use self::DnsProtocolError::ClassIsReservedUnassignedOrObsolete;

		let upper = unsafe { *self.qclass.get_unchecked(0) };

		if likely!(upper == 0x00)
		{
			use self::QueryClass::*;

			let lower = unsafe { *self.qclass.get_unchecked(1) };
			match lower
			{
				0x01 => Ok(Internet),
				0xFE => Ok(None),
				0xFF => Ok(Asterisk),
				_ => Err(ClassIsReservedUnassignedOrObsolete(self.qclass))
			}
		}
		else
		{
			Err(ClassIsReservedUnassignedOrObsolete(self.qclass))
		}
	}

	#[inline(always)]
	fn set_query_class(&mut self, query_class: QueryClass)
	{
		unsafe { *self.qclass.get_unchecked_mut(0) = 0 };
		unsafe { *self.qclass.get_unchecked_mut(1) = query_class as u8 };
	}
}

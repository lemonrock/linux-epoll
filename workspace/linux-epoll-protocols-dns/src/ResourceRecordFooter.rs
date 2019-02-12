// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[repr(C, packed)]
struct ResourceRecordFooter
{
	type_: ResourceRecordType,
	class: [u8; 2],
	ttl: [u8; 4],
	rdlength: [u8; 2],
	rdata: ResourceData,
}

impl ResourceRecordFooter
{
	const MinimumSize: usize = size_of::<ResourceRecordType>() + size_of::<[u8; 2]>() + size_of::<[u8; 4]>() + size_of::<[u8; 2]>() + ResourceData::MinimumSize;

	#[inline(always)]
	fn resource_record_type(&self) -> ResourceRecordType
	{
		self.type_
	}

	#[inline(always)]
	fn set_resource_record_type(&mut self, resource_record_type: ResourceRecordType)
	{
		self.type_ = resource_record_type
	}

	#[inline(always)]
	fn resource_record_class(&self) -> Result<ResourceRecordClass, ProtocolError>
	{
		use self::DnsProtocolError::ClassIsReservedUnassignedOrObsolete;

		let upper = unsafe { *self.class.get_unchecked(0) };

		if likely!(upper == 0x00)
		{
			use self::ResourceRecordClass::*;

			let lower = unsafe { *self.class.get_unchecked(1) };
			match lower
			{
				0x01 => Ok(Internet),
				_ => Err(ClassIsReservedUnassignedOrObsolete(self.class))
			}
		}
		else
		{
			Err(ClassIsReservedUnassignedOrObsolete(self.class))
		}
	}

	#[inline(always)]
	fn set_resource_record_class(&mut self, resource_record_class: ResourceRecordClass)
	{
		unsafe { *self.class.get_unchecked_mut(0) = 0 };
		unsafe { *self.class.get_unchecked_mut(1) = resource_record_class as u8 };
	}

	#[inline(always)]
	fn requestors_udp_payload_size(&self) -> u16
	{
		self.debug_assert_is_opt();

		u16::from_be_bytes(self.class)
	}

	/// `size` is typically a MTU, like 1280; realistically, as we use TCP, we should probably set this to 4Kb or some such.
	#[inline(always)]
	fn set_requestors_udp_payload_size(&mut self, size: u16)
	{
		self.debug_assert_is_opt();

		self.class = size.to_be_bytes()
	}

	#[inline(always)]
	fn time_to_live(&self) -> TimeToLiveInSeconds
	{
		TimeToLiveInSeconds(self.ttl)
	}

	#[inline(always)]
	fn set_time_to_live(&mut self, time_to_live: TimeToLiveInSeconds)
	{
		self.ttl = time_to_live.0
	}

	#[inline(always)]
	fn extended_response_code_and_flags(&self) -> ExtendedResponseCodeAndFlags
	{
		self.debug_assert_is_opt();

		ExtendedResponseCodeAndFlags(self.ttl)
	}

	#[inline(always)]
	fn set_extended_response_code_and_flags(&mut self, value: ExtendedResponseCodeAndFlags)
	{
		self.debug_assert_is_opt();

		self.ttl = value.0
	}

	#[inline(always)]
	fn resource_data_length(&self) -> u16
	{
		u16::from_be_bytes(self.rdlength)
	}

	#[inline(always)]
	fn set_resource_data_length(&mut self, length: u16)
	{
		self.rdlength = length.to_be_bytes()
	}

	#[inline(always)]
	fn resource_data(&self) -> &ResourceData
	{
		&self.message
	}

	#[inline(always)]
	fn resource_data_mutable(&mut self) -> &mut ResourceData
	{
		&mut self.message
	}

	#[inline(always)]
	fn debug_assert_is_opt(&self)
	{
		debug_assert!(unsafe{ self.type_.known } , WellKnownResourceRecordType::OPT, "This is not an EDNS0 extension record")
	}
}

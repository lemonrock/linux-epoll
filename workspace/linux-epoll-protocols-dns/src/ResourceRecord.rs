// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


extern
{
	type ResourceRecord;
}

impl ResourceRecord
{
	const MinimumSize: usize = Name::MinimumSize + ResourceRecordFooter::MinimumSize;

	#[inline(always)]
	fn parse_resource_record<'a>(&self, end_of_message_pointer: usize, parsed_labels: &mut ParsedLabels<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, processing_additional_record_section: bool, have_already_seen_an_edns_opt_record: bool) -> Result<(), DnsProtocolError>
	{
		use self::DnsProtocolError::*;

		let start_of_resource_record_pointer = self as *const Self as usize;

		if unlikely!(start_of_resource_record_pointer + Self::MinimumSize > end_of_message_pointer)
		{
			return Err(ResourceRecordIsShorterThanMinimumSize)
		}

		let (parsed_name_iterator, end_of_name_pointer) = parsed_labels.parse_name(start_of_resource_record_pointer, end_of_message_pointer)?;
		debug_assert!(end_of_name_pointer < end_of_message_pointer, "end_of_name_pointer exceeds end_of_message_pointer");

		if unlikely!(end_of_name_pointer + ResourceRecordFooter::MinimumSize > end_of_message_pointer)
		{
			return Err(ResourceRecordIsShorterThanMinimumSizeAfterParsingName)
		}

		self.dispatch_resource_record_type(end_of_name_pointer, end_of_message_pointer, parsed_labels, resource_record_visitor)
	}

	#[inline(always)]
	fn dispatch_resource_record_type<'a>(&self, end_of_name_pointer: usize, end_of_message_pointer: usize, parsed_labels: &mut ParsedLabels<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<(), DnsProtocolError>
	{
		match self.resource_record_type(end_of_name_pointer)
		{
			ResourceRecordType::A => self.handle_a(end_of_name_pointer, end_of_message_pointer, parsed_name_iterator, resource_record_visitor),

			ResourceRecordType::NS => self.handle_ns(end_of_name_pointer, end_of_message_pointer, parsed_name_iterator, resource_record_visitor),

			ResourceRecordType::CNAME => self.handle_cname(end_of_name_pointer, end_of_message_pointer, parsed_name_iterator, resource_record_visitor),

			ResourceRecordType::SOA => self.handle_soa(end_of_name_pointer, end_of_message_pointer, parsed_name_iterator, resource_record_visitor),

			ResourceRecordType::PTR => self.handle_ptr(end_of_name_pointer, end_of_message_pointer, parsed_name_iterator, resource_record_visitor),

			ResourceRecordType::MX => self.handle_mx(end_of_name_pointer, end_of_message_pointer, parsed_name_iterator, resource_record_visitor),

			ResourceRecordType::TXT => self.handle_txt(end_of_name_pointer, end_of_message_pointer, parsed_name_iterator, resource_record_visitor),

			ResourceRecordType::AAAA => self.handle_aaaa(end_of_name_pointer, end_of_message_pointer, parsed_name_iterator, resource_record_visitor),

			ResourceRecordType::LOC => self.handle_loc(end_of_name_pointer, end_of_message_pointer, parsed_name_iterator, resource_record_visitor),

			ResourceRecordType::SRV =>
			{
			}

			ResourceRecordType::OPT => self.handle_opt(end_of_name_pointer, end_of_message_pointer, processing_additional_record_section, have_already_seen_an_edns_opt_record),

			ResourceRecordType::SSHFP => self.handle_sshfp(end_of_name_pointer, end_of_message_pointer, parsed_name_iterator, resource_record_visitor),

			ResourceRecordType::OPENPGPKEY =>
			{
				// rfc4880 Transferable Public Key
			}

			ResourceRecordType::TLSA =>
			{
			}

			ResourceRecordType::Asterisk => Err(ResourceRecordTypeAsteriskShouldNotOccurOutsideOfAQuestionSectionEntry),

			ResourceRecordType::CAA =>
			{
			}

			xxx
			// TODO: Review - may be handle some unsupported but very occasionally encountered kinds, eg HINFO, with an extension function rather than an error.
			unsupported_resource_record_type @ _ => Err(ResourceRecordTypeIsUnsupported(unsupported_resource_record_type))
		}
	}

	#[inline(always)]
	fn handle_a<'a>(&self, end_of_name_pointer: usize, end_of_message_pointer: usize, parsed_name_iterator: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<(), DnsProtocolError>
	{
		let (time_to_live, record) = self.parse_internet_protocol_address_only(end_of_name_pointer, end_of_message_pointer)?;
		resource_record_visitor.A(parsed_name_iterator, time_to_live, record)
	}

	#[inline(always)]
	fn handle_ns<'a>(&self, end_of_name_pointer: usize, end_of_message_pointer: usize, parsed_name_iterator: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<(), DnsProtocolError>
	{
		let (time_to_live, record) = self.parse_name_only(end_of_name_pointer, end_of_message_pointer)?;
		resource_record_visitor.NS(parsed_name_iterator, time_to_live, record)
	}

	#[inline(always)]
	fn handle_cname<'a>(&self, end_of_name_pointer: usize, end_of_message_pointer: usize, parsed_name_iterator: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<(), DnsProtocolError>
	{
		let (time_to_live, record) = self.parse_name_only(end_of_name_pointer, end_of_message_pointer)?;
		resource_record_visitor.CNAME(parsed_name_iterator, time_to_live, record)
	}

	#[inline(always)]
	fn handle_soa<'a>(&self, end_of_name_pointer: usize, end_of_message_pointer: usize, parsed_name_iterator: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<(), DnsProtocolError>
	{
		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;
		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;
		let start_of_resource_data = resource_data.as_ptr() as usize;

		let (mname, end_of_mname_pointer) = parsed_labels.parse_name_in_slice(resource_data)?;
		let (rname, end_of_rname_pointer) = parsed_labels.parse_name_in_slice(&resource_data[(end_of_mname_pointer - start_of_resource_data) .. ])?;

		let end_of_resource_data = start_of_resource_data + resource_data.len();
		if likely!((end_of_resource_data - end_of_rname_pointer) == size_of::<StartOfAuthorityFooter>())
		{
			let start_of_authority = StartOfAuthority
			{
				mname,
				rname,
				footer: unsafe { & * (end_of_rname_pointer as *const StartOfAuthorityFooter) },
			};

			resource_record_visitor.SOA(parsed_name_iterator, time_to_live, start_of_authority)
		}
		else
		{
			Err(StartOfAuthorityIsIncorrectSizeAfterParsingMNAMEAndRNAME)
		}
	}

	#[inline(always)]
	fn handle_ptr<'a>(&self, end_of_name_pointer: usize, end_of_message_pointer: usize, parsed_name_iterator: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<(), DnsProtocolError>
	{
		let (time_to_live, record) = self.parse_name_only(end_of_name_pointer, end_of_message_pointer)?;
		resource_record_visitor.PTR(parsed_name_iterator, time_to_live, record)
	}

	#[inline(always)]
	fn handle_mx<'a>(&self, end_of_name_pointer: usize, end_of_message_pointer: usize, parsed_name_iterator: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<(), DnsProtocolError>
	{
		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;
		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		if unlikely!(resource_data.len() < 3)
		{
			Err(ResourceDataForTypeMXHasTooShortALength(resource_data.len()))
		}

		let record = MailExchange
		{
			preference: u16::from_be_bytes(unsafe { *(resource_data.as_ptr() as *const [u8; 2]) }),
			mail_server_name: parsed_labels.parse_name_in_slice_with_nothing_left(&resource_data[2 .. ])?,
		};

		resource_record_visitor.MX(parsed_name_iterator, time_to_live, record)
	}

	#[inline(always)]
	fn handle_txt<'a>(&self, end_of_name_pointer: usize, end_of_message_pointer: usize, parsed_name_iterator: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<(), DnsProtocolError>
	{
		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;

		let text_strings_iterator = TextStringsIterator::new(self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?)?;

		resource_record_visitor.TXT(parsed_name_iterator, time_to_live, text_strings_iterator)
	}

	#[inline(always)]
	fn handle_aaaa<'a>(&self, end_of_name_pointer: usize, end_of_message_pointer: usize, parsed_name_iterator: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<(), DnsProtocolError>
	{
		let (time_to_live, record) = self.parse_internet_protocol_address_only(end_of_name_pointer, end_of_message_pointer)?;
		resource_record_visitor.AAAA(parsed_name_iterator, time_to_live, record)
	}

	#[inline(always)]
	fn handle_loc<'a>(&self, end_of_name_pointer: usize, end_of_message_pointer: usize, parsed_name_iterator: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<(), DnsProtocolError>
	{
		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;

		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;
		let length = resource_data.len();
		if unlikely!(length != 14)
		{
			return Err(ResourceDataForTypeLOCHasAnIncorrectLength(length))
		}

		let location = unsafe { & * (resource_data.as_ptr() as *const Location) };
		if location.version != 0
		{
			return Err(ResourceDataForTypeLOCHasAnIncorrectVersion(location.version))
		}

		resource_record_visitor.LOC(parsed_name_iterator, time_to_live, location)
	}

	#[inline(always)]
	fn handle_opt(&self, end_of_name_pointer: usize, end_of_message_pointer: usize, processing_additional_record_section: bool, have_already_seen_an_edns_opt_record: bool) -> Result<(), DnsProtocolError>
	{
		if !processing_additional_record_section
		{
			return Err(EdnsOptRecordOutsideOfAdditionalDataSection)
		}

		if have_already_seen_an_edns_opt_record
		{
			return Err(MoreThanOneEdnsOptRecord)
		}

		let start_of_name_pointer = (self.name as *const Name as usize);
		if unlikely!(end_of_name_pointer - start_of_name_pointer != 1)
		{
			return Err(EdnsOptRecordNameTooLong)
		}

		let name_length_or_offset = unsafe { * (self.name as *const Name as *const u8) };
		if unlikely!(name_length_or_offset != 0x00)
		{
			return Err(EdnsOptRecordNameNotRoot)
		}

		let requestors_udp_payload_size = self.requestors_udp_payload_size(end_of_name_pointer);
		let extended_response_code_and_flags = self.extended_response_code_and_flags(end_of_name_pointer);
		let upper_8_bits = extended_response_code_and_flags.extended_response_code_upper_8_bits();
		let version = extended_response_code_and_flags.version()?;
		debug_assert!(version, ExtendedDnsVersion::Version0, "Why do we support EDNS versions other than 0?");
		let dnssec_ok = extended_response_code_and_flags.dnssec_ok();
		extended_response_code_and_flags.z()?;

		// TODO: Parse TLV options.
		xxx

		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;
	}

	#[inline(always)]
	fn handle_sshfp<'a>(&self, end_of_name_pointer: usize, end_of_message_pointer: usize, parsed_name_iterator: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<(), DnsProtocolError>
	{
		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;

		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;
		let length = resource_data.len();
		if unlikely!(length < 2)
		{
			return Err(ResourceDataForTypeSSHFPHasAnIncorrectLength(length))
		}

		let raw_public_key_algorithm = unsafe { * resource_data.get_unchecked(0) };
		let public_key_algorithm: PublicKeyAlgorithm = match raw_public_key_algorithm
		{
			0x01 ... 0x04 => unsafe { transmute(raw_fingerprint_type) },

			_ => return Err(ResourceDataForTypeSSHFPHasAnUnrecognisedPublicKeyAlgorithm(raw_fingerprint_type)),
		};

		use self::FingerprintType::*;
		let raw_fingerprint_type = unsafe { * resource_data.get_unchecked(1) };
		let (digest_algorithm, digest_size) = match raw_fingerprint_type
		{
			0x01 => (SHA_1, 160 / 8),

			0x02 => (SHA_256, 256 / 8),

			_ => return Err(ResourceDataForTypeSSHFPHasAnUnrecognisedFingerprintType(raw_fingerprint_type)),
		};

		let digest_bytes = &resource_data[2 .. ];
		if unlikely!(digest_bytes.len() != digest_size)
		{
			Err(ResourceDataForTypeSSHFPHasADigestOfIncorrectSizeForTheFingerprintType(digest_algorithm, digest_bytes.len()))
		}
		else
		{
			let record = PublicKeyFingerprint
			{
				public_key_algorithm,
				digest_algorithm,
				digest_bytes,
			};

			resource_record_visitor.SSHFP(parsed_name_iterator, time_to_live, record)
		}
	}

	#[inline(always)]
	fn parse_internet_protocol_address_only<Address: Sized>(&self, end_of_name_pointer: usize, end_of_message_pointer: usize) -> Result<(TimeToLiveInSeconds, &Address), DnsProtocolError>
	{
		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;

		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;
		let length = resource_data.len();
		if unlikely!(length != size_of::<Address>())
		{
			Err(ResourceDataForTypeAOrAAAAHasAnIncorrectLength(length))
		}
		else
		{
			let address = unsafe { &*(resource_data.as_ptr() as *const Address) };
			Ok((time_to_live, address))
		}
	}

	#[inline(always)]
	fn parse_name_only<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize) -> Result<(TimeToLiveInSeconds, ParsedNameIterator<'a>), DnsProtocolError>
	{
		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;
		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;
		let record = parsed_labels.parse_name_in_slice_with_nothing_left(resource_data)?;

		Ok((time_to_live, record))
	}

	#[inline(always)]
	fn validate_class_and_get_time_to_live(&self, end_of_name_pointer: usize) -> Result<TimeToLiveInSeconds, DnsProtocolError>
	{
		let class = self.resource_record_class(end_of_name_pointer)?;
		debug_assert!(class, ResourceRecordClass::Internet, "Why do we support classes other than Internet?");

		Ok(self.time_to_live(end_of_name_pointer))
	}

	#[inline(always)]
	fn safely_access_resource_data(&self, end_of_name_pointer: usize, end_of_message_pointer: usize) -> Result<&[u8], DnsProtocolError>
	{
		let resource_data_length = self.resource_data_length(end_of_name_pointer);
		if unlikely!(end_of_name_pointer + resource_data_length > end_of_message_pointer)
		{
			Err(ResourceDataLengthOverflows)
		}
		else
		{
			Ok(unsafe { from_raw_parts(self.resource_data(end_of_name_pointer) as *const ResourceDataOctets as *const u8, resource_data_length as usize) })
		}
	}

	/// `NAME` field.
	///
	/// For an `OPT` record, this should always just be `0x00`.
	#[inline(always)]
	fn name(&self) -> &Name
	{
		unsafe { & * (self as *const Self as *const Name) }
	}

	/// `NAME` field.
	///
	/// For an `OPT` record, this should always just be `0x00`.
	#[inline(always)]
	fn name_mutable(&self) -> &mut Name
	{
		unsafe { &mut * (self as *mut Self as *mut Name) }
	}

	/// `TYPE` field.
	#[inline(always)]
	fn resource_record_type(&self, end_of_name_pointer: usize) -> ResourceRecordType
	{
		self.footer(end_of_name_pointer).resource_record_type()
	}

	/// `TYPE` field.
	#[inline(always)]
	fn set_resource_record_type(&mut self, end_of_name_pointer: usize, resource_record_type: ResourceRecordType)
	{
		self.footer_mutable(end_of_name_pointer).set_resource_record_type(resource_record_type)
	}

	/// `CLASS` field.
	#[inline(always)]
	fn resource_record_class(&self, end_of_name_pointer: usize) -> Result<ResourceRecordClass, ProtocolError>
	{
		self.footer(end_of_name_pointer).resource_record_class()
	}

	/// `CLASS` field.
	#[inline(always)]
	fn set_resource_record_class(&mut self, end_of_name_pointer: usize, resource_record_class: ResourceRecordClass)
	{
		self.footer_mutable(end_of_name_pointer).set_resource_record_class(resource_record_class)
	}

	/// `EDNS(0)` field.
	///
	/// RFC 6891.
	#[inline(always)]
	fn requestors_udp_payload_size(&self, end_of_name_pointer: usize) -> u16
	{
		self.footer(end_of_name_pointer).requestors_udp_payload_size()
	}

	/// `EDNS(0)` field.
	///
	/// RFC 6891.
	///
	/// `size` is typically a MTU, like 1280; realistically, as we use TCP, we should probably set this to 4Kb or some such.
	#[inline(always)]
	fn set_requestors_udp_payload_size(&mut self, end_of_name_pointer: usize, size: u16)
	{
		self.footer_mutable(end_of_name_pointer).set_requestors_udp_payload_size(size)
	}

	/// `TTL` field.
	#[inline(always)]
	fn time_to_live(&self, end_of_name_pointer: usize) -> TimeToLiveInSeconds
	{
		self.footer(end_of_name_pointer).time_to_live()
	}

	/// `TTL` field.
	#[inline(always)]
	fn set_time_to_live(&mut self, end_of_name_pointer: usize, time_to_live: TimeToLiveInSeconds)
	{
		self.footer_mutable(end_of_name_pointer).set_time_to_live(time_to_live)
	}

	/// `EDNS(0)` field.
	///
	/// RFC 6891.
	#[inline(always)]
	fn extended_response_code_and_flags(&self, end_of_name_pointer: usize) -> ExtendedResponseCodeAndFlags
	{
		self.footer(end_of_name_pointer).extended_response_code_and_flags()
	}

	/// `EDNS(0)` field.
	///
	/// RFC 6891.
	#[inline(always)]
	fn set_extended_response_code_and_flags(&mut self, end_of_name_pointer: usize, value: ExtendedResponseCodeAndFlags)
	{
		self.footer_mutable(end_of_name_pointer).set_extended_response_code_and_flags(value)
	}

	/// `RDLEN` field.
	#[inline(always)]
	fn resource_data_length(&self, end_of_name_pointer: usize) -> u16
	{
		self.footer(end_of_name_pointer).resource_data_length(length)
	}

	/// `RDLEN` field.
	#[inline(always)]
	fn set_resource_data_length(&mut self, end_of_name_pointer: usize, length: u16)
	{
		self.footer_mutable(end_of_name_pointer).set_resource_data_length(length)
	}

	/// `RDATA` field.
	#[inline(always)]
	fn resource_data(&self, end_of_name_pointer: usize) -> &ResourceData
	{
		self.footer(end_of_name_pointer).resource_data()
	}

	/// `RDATA` field.
	#[inline(always)]
	pub fn resource_data_mutable(&mut self, end_of_name_pointer: usize) -> &mut ResourceData
	{
		self.footer_mutable(end_of_name_pointer).resource_data_mutable()
	}

	#[inline(always)]
	fn footer(&self, end_of_name_pointer: usize) -> &ResourceRecordFooter
	{
		unsafe { & * (end_of_name_pointer as *const ResourceRecordFooter) }
	}

	#[inline(always)]
	fn footer_mutable(&self, end_of_name_pointer: usize) -> &mut ResourceRecordFooter
	{
		unsafe { &mut * (end_of_name_pointer as *mut ResourceRecordFooter) }
	}
}


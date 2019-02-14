// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


extern
{
	type ResourceRecord;
}

impl ResourceRecord
{
	const MinimumSize: usize = Name::MinimumSize + ResourceRecordFooter::MinimumSize;

	/// Returns `Ok(end_of_resource_data_pointer)` unless there is an error.
	#[inline(always)]
	fn parse_resource_record<'a>(&'a self, end_of_message_pointer: usize, parsed_labels: &mut ParsedLabels<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, processing_additional_record_section: bool, have_already_seen_an_edns_opt_record: bool, is_a_response: bool) -> Result<usize, DnsProtocolError>
	{
		debug_assert!(is_a_response, "Server validation (of requests) is not supported");

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

		self.dispatch_resource_record_type(end_of_name_pointer, end_of_message_pointer, parsed_name_iterator, parsed_labels, resource_record_visitor, processing_additional_record_section, have_already_seen_an_edns_opt_record, is_a_response)
	}

	/// Compression of names within `RDATA` is a mess.
	///
	/// RFC 3597, Section 4, Paragraph 2 restricts the records to which name (label) compression can be applied to be those defined in RFC 1035 which implicitly contain a name, hence:-
	///
	/// * `CNAME`
	/// * `MB`
	/// * `MD`
	/// * `MF`
	/// * `MG`
	/// * `MINFO`
	/// * `MR`
	/// * `MX`
	/// * `NS`
	/// * `PTR`
	/// * `SOA`
	///
	/// Of these, many are obsolete, leaving the list as:-
	///
	/// * `CNAME`
	/// * `MX`
	/// * `NS`
	/// * `PTR`
	/// * `SOA`
	///
	/// Additionally:-
	///
	/// * RFC 2163 permits compression to `PX` records;
	/// * RFC 2535 permits compression in `SIG` and `NXT` records;
	/// * RFC 3597 permits compression in `RP`, `AFSDB`, `RT` and `NAPTR` records;
	/// * RFC 3597 prohibits compression in `PX`, `SIG` and `NXT` records;
	/// * RFC 2782 prohibits compression in `SRV` records but the original RFC 2052 mandated it;
	/// * RFC 3597 prohibits compression for all future record types;
	/// * RFC 6672 prohibits compression for `DNAME`, but historically, there was confusion in the original RFC 2672 about whether it was permitted.
	///
	/// Of the records listed in the clause above, all are obsolete apart from `NAPTR`, `SRV` and `DNAME`.
	///
	/// Observations:-
	///
	/// * Given the history of `SRV`, it seems prudent to permit compression.
	/// * Given the similarity of `DNAME` to `CNAME`, and the historic confusion, it seems prudent to permit compression;
	///
	/// This gives a list of
	///
	/// * `CNAME`
	/// * `MX`
	/// * `NS`
	/// * `PTR`
	/// * `SOA`
	/// * `NAPTR`
	/// * `SRV`
	/// * `DNAME`
	#[inline(always)]
	fn dispatch_resource_record_type<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, parsed_labels: &mut ParsedLabels<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, processing_additional_record_section: bool, have_already_seen_an_edns_opt_record: bool, is_a_response: bool) -> Result<usize, DnsProtocolError>
	{
		use self::DnsProtocolError::*;

		let resource_record_type = self.resource_record_type(end_of_name_pointer);
		let resource_record_type_bytes = resource_record_type.0;

		let type_upper = unsafe { * resource_record_type_bytes.get_unchecked(0) };
		let type_lower = unsafe { * resource_record_type_bytes.get_unchecked(1) };

		/// Based on RFC 6895, Section 3.1, Page 8.
		match type_upper
		{
			0x00 => match type_lower
			{
				DataType::SIG0_lower => self.handle_sig0(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::A_lower => self.handle_a(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::NS_lower => self.handle_ns(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels),

				DataType::MD_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::MF_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::CNAME_lower => self.handle_cname(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels),

				DataType::SOA_lower => self.handle_soa(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels),

				DataType::MB_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::MG_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::MR_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::NULL_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::WKS_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::PTR_lower => self.handle_ptr(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels),

				DataType::HINFO_lower => self.handle_hinfo(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::MINFO_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::MX_lower => self.handle_mx(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels),

				DataType::TXT_lower => self.handle_txt(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::RP_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::AFSDB_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::X25_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::ISDN_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::RT_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::NSAP_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::NSAP_PTR_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::SIG_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::KEY_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::PX_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::GPOS_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::AAAA_lower => self.handle_aaaa(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::LOC_lower => self.handle_loc(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::NXT_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::EID_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::NIMLOC_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::SRV_lower => self.handle_srv(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels),

				DataType::ATMA_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::NAPTR_lower => self.handle_naptr(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels),

				DataType::KX_lower => self.handle_kx(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels),

				DataType::CERT_lower => self.handle_unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, resource_record_type),

				DataType::A6_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::DNAME_lower => self.handle_dname(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels),

				DataType::SINK_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				MetaType::OPT_lower => self.handle_opt(end_of_name_pointer, end_of_message_pointer, processing_additional_record_section, have_already_seen_an_edns_opt_record, is_a_response),

				DataType::APL_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::DS_lower => self.handle_unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::SSHFP_lower => self.handle_sshfp(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::IPSECKEY_lower => self.unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::NSEC_lower => self.unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::RRSIG_lower => self.unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::DNSKEY_lower => self.unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::DHCID_lower => self.unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::NSEC3_lower => self.unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::NSEC3PARAM_lower => self.unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::TLSA_lower => self.handle_tlsa(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::SMIMEA_lower => self.handle_smimea(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				54 => self.unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::HIP_lower => self.unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::NINFO_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::RKEY_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::TALINK_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::CDS_lower => self.unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::CDNSKEY_lower => self.unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::OPENPGPKEY_lower => self.handle_openpgpkey(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::CSYNC_lower => self.unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::ZONEMD_lower => self.unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				64 ... 98 => self.unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::SPF_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::UINFO_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::UID_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::GID_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::UNSPEC_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::NID_lower => self.swallow("Class independent"),

				DataType::L32_lower => self.swallow("Class independent"),

				DataType::L64_lower => self.swallow("Class independent"),

				DataType::LP_lower => self.swallow("Class independent"),

				DataType::EUI48_lower => self.swallow("Class independent"),

				DataType::EUI64_lower => self.swallow("Class independent"),

				110 ... 0x7F => self.handle_unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, resource_record_type),

				0x80 ... 0xF8 => Err(UnknownQueryTypeOrMetaType(resource_record_type)),

				MetaType::TKEY_lower => self.swallow("Only really useful for updates, which, frankly, are probably better done out-of-band than using DNS; regardless, when using DNS over TLS a client certificate is much more useful"),

				MetaType::TSIG_lower => self.swallow("Only really useful for updates, which, frankly, are probably better done out-of-band than using DNS; regardless, when using DNS over TLS a client certificate is much more useful"),

				QueryType::IXFR_lower => Err(QueryTypeIXFRShouldNotOccurOutsideOfAQuestionSectionEntry),

				QueryType::AXFR_lower => Err(QueryTypeAXFRShouldNotOccurOutsideOfAQuestionSectionEntry),

				QueryType::MAILB_lower => Err(QueryTypeMAILBShouldNotOccurOutsideOfAQuestionSectionEntry),

				QueryType::MAILA_lower => Err(QueryTypeMAILAShouldNotOccurOutsideOfAQuestionSectionEntry),

				QueryType::Asterisk_lower => Err(QueryTypeAsteriskShouldNotOccurOutsideOfAQuestionSectionEntry),

				_ => self.handle_unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, resource_record_type),
			},

			0x01 => match type_lower
			{
				DataType::URI_lower => self.handle_unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, resource_record_type),

				DataType::CAA_lower => self.handle_unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, resource_record_type),

				DataType::DOA_lower => self.handle_unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, resource_record_type),

				DataType::AMTRELAY_lower => self.handle_unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, resource_record_type),
				
				_ => self.handle_unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, resource_record_type),
			},

			0x02 ... 0x7F => self.handle_unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, resource_record_type),

			0x80 => match type_lower
			{
				DataType::TA_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				DataType::DLV_lower => Err(ObsoleteResourceRecordType(resource_record_type)),

				_ => self.handle_unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, resource_record_type),
			},

			0x81 ... 0xEF => self.handle_unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, resource_record_type),

			_ => Err(ReservedRecordType(resource_record_type))
		}

		NAPTR
			compressed labels allowed.
		CERT*
		DS*

		NSEC
		RRSIG*
		DNSKEY*
		DHCID
		NSEC3
		NSEC3PARAM

		CDS
		CDNSKEY

		NID
		L32
		L64
		LP

		EUI48
		EUI64

		CAA
		URI

	}

	#[inline(always)]
	fn swallow<'a>(&'a self, _reason: &'static str) -> Result<usize, DnsProtocolError>
	{
		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;
		Ok(Self::end_of_resource_data_pointer(resource_data))
	}

	#[inline(always)]
	fn handle_a<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.parse_internet_protocol_address_only(end_of_name_pointer, end_of_message_pointer)?;
		resource_record_visitor.A(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_ns<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.parse_name_only(end_of_name_pointer, end_of_message_pointer, parsed_labels)?;
		resource_record_visitor.NS(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_cname<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.parse_name_only(end_of_name_pointer, end_of_message_pointer, parsed_labels)?;
		resource_record_visitor.CNAME(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_soa<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
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

			resource_record_visitor.SOA(resource_record_name, time_to_live, start_of_authority)?;
			Ok(Self::end_of_resource_data_pointer(resource_data))
		}
		else
		{
			Err(DnsProtocolError::StartOfAuthorityIsIncorrectSizeAfterParsingMNAMEAndRNAME)
		}
	}

	#[inline(always)]
	fn handle_ptr<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.parse_name_only(end_of_name_pointer, end_of_message_pointer, parsed_labels)?;
		resource_record_visitor.PTR(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_hinfo<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		use self::DnsProtocolError::*;

		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;

		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;
		if unlikely!(resource_data.len() < 1 + 1)
		{
			return Err(ResourceDataForTypeHINFOHasTooShortALength(resource_data.len()))
		}

		let character_strings_iterator = CharacterStringsIterator::new(resource_data);

		let cpu = character_strings_iterator.next().ok_or(ResourceDataForTypeHINFOWouldHaveCpuDataOverflow(resource_data.len()))?;

		let os = character_strings_iterator.next().ok_or(ResourceDataForTypeHINFOWouldHaveOsDataOverflow(resource_data.len()))?;

		if likely!(character_strings_iterator.is_empty())
		{
			resource_record_visitor.HINFO(resource_record_name, time_to_live, record)?;
			Ok(Self::end_of_resource_data_pointer(resource_data))
		}
		else
		{
			Err(ResourceDataForTypeHINFOWouldHaveUnusuedDataRemaining)
		}
	}

	#[inline(always)]
	fn handle_mx<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
	{
		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;
		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		if unlikely!(resource_data.len() < 3)
		{
			return Err(DnsProtocolError::ResourceDataForTypeMXHasTooShortALength(resource_data.len()))
		}

		let record = MailExchange
		{
			preference: u16::from_be_bytes(unsafe { *(resource_data.as_ptr() as *const [u8; 2]) }),
			mail_server_name: parsed_labels.parse_name_in_slice_with_nothing_left(&resource_data[2 .. ])?,
		};

		resource_record_visitor.MX(resource_record_name, time_to_live, record)?;
		Ok(Self::end_of_resource_data_pointer(resource_data))
	}

	#[inline(always)]
	fn handle_txt<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;

		let text_strings_iterator = CharacterStringsIterator::new(self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?)?;

		resource_record_visitor.TXT(resource_record_name, time_to_live, text_strings_iterator)?;

		if likely!(text_strings_iterator.is_empty())
		{
			Ok(Self::end_of_resource_data_pointer(resource_data))
		}
		else
		{
			Err(DnsProtocolError::ResourceDataForTypeTXTWouldHaveUnusuedDataRemaining)
		}
	}

	#[inline(always)]
	fn handle_aaaa<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.parse_internet_protocol_address_only(end_of_name_pointer, end_of_message_pointer)?;
		resource_record_visitor.AAAA(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_loc<'a>(&self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		use self::DnsProtocolError::*;

		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;

		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;
		let length = resource_data.len();
		if unlikely!(length != size_of::<Location>())
		{
			return Err(ResourceDataForTypeLOCHasAnIncorrectLength(length))
		}

		let location = unsafe { & * (resource_data.as_ptr() as *const Location) };

		let version = location.version()?;
		debug_assert_eq!(version, LocationVersion::Version0, "Why are we supporting a version other than 0 of LOC records?");

		resource_record_visitor.LOC(resource_record_name, time_to_live, location)?;
		Ok(Self::end_of_resource_data_pointer(resource_data))
	}

	#[inline(always)]
	fn handle_srv<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
	{
		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;

		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;
		let length = resource_data.len();
		if unlikely!(length < 7)
		{
			return Err(DnsProtocolError::ResourceDataForTypeSRVHasAnIncorrectLength(length))
		}

		let record = Service
		{
			priority: u16::from_be_bytes(unsafe { *(resource_data.get_unchecked(0) as *const u8 as *const [u8; 2]) }),
			weight: u16::from_be_bytes(unsafe { *(resource_data.get_unchecked(2) as *const u8 as *const [u8; 2]) }),
			port: u16::from_be_bytes(unsafe { *(resource_data.get_unchecked(4) as *const u8 as *const [u8; 2]) }),
			target: parsed_labels.parse_name_in_slice_with_nothing_left(&resource_data[6 .. ])?,
		};

		resource_record_visitor.SRV(resource_record_name, time_to_live, record)?;
		Ok(Self::end_of_resource_data_pointer(resource_data))
	}

	#[inline(always)]
	fn handle_naptr<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
	{
		use self::DnsProtocolError::*;

		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;

		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;
		let length = resource_data.len();
		if unlikely!(length < 2 + 2 + 1 + 1 + 1 + 1)
		{
			return Err(ResourceDataForTypeNAPTRHasAnIncorrectLength(length))
		}

		let order = u16::from_be_bytes(unsafe { * (resource_data.get_unchecked(0) as *const u8 as *const [u8; 2]) });
		let preference = u16::from_be_bytes(unsafe { * (resource_data.get_unchecked(2) as *const u8 as *const [u8; 2]) });


		let character_strings_iterator = CharacterStringsIterator::new(resource_data);

		let flags = character_strings_iterator.next().ok_or(ResourceDataForTypeHINFOWouldHaveCpuDataOverflow(resource_data.len()))?;

		let services = character_strings_iterator.next().ok_or(ResourceDataForTypeHINFOWouldHaveOsDataOverflow(resource_data.len()))?;

		let regular_expression = character_strings_iterator.next().ok_or(ResourceDataForTypeHINFOWouldHaveOsDataOverflow(resource_data.len()))?;

		let remaining_resource_data = character_strings_iterator.remaining_resource_data();
		let start_of_name_pointer = remaining_resource_data.as_ptr() as usize;
		let end_of_resource_data = start_of_name_pointer + remaining_resource_data.len();

		let header = NamingAuthorityPointerHeader
		{
			order,
			preference,
			flags,
			services,
		};

		if regular_expression.is_empty()
		{
			let (domain_name, end_of_name_pointer) = ParsedNameIterator::parse_without_compression(start_of_name_pointer, end_of_resource_data);
			if unlikely!(end_of_name_pointer != end_of_resource_data)
			{
				return Err(ResourceDataForTypeNAPTRHasDataLeftOver)
			}

			let record = NamingAuthorityPointerWithDomainName
			{
				header,
				domain_name
			};

			resource_record_visitor.NAPTR_domain_name(resource_record_name, time_to_live, record)?;
			Ok(Self::end_of_resource_data_pointer(resource_data))
		}
		else
		{
			let end_of_name_pointer = start_of_name_pointer + 1;

			if unlikely!(end_of_name_pointer != end_of_resource_data)
			{
				return Err(ResourceDataForTypeNAPTRHasDataLeftOver)
			}

			let domain_name_byte = unsafe { * (start_of_name_pointer as *const u8) };
			if unlikely!(domain_name_byte != 0)
			{
				return Err(ResourceDataForTypeNAPTRHasBothARegularExpressionAndADomainName)
			}

			let record = NamingAuthorityPointerWithRegularExpression
			{
				header,
				regular_expression
			};

			resource_record_visitor.NAPTR_regular_expression(resource_record_name, time_to_live, record)?;
		}





		if regular_expression.is_empty()
		{

		}
		else
		{

		}

		xxxx;



		// order - 16-bit
		// PREFERENCE - 16-bit
		// flags - character-string, with values A-Z and 0-9.
		// services - character-string, unconstrained.
		// regexp - character-string, complex rules.
		// domain-name.

		// Either one of regexp or domain-name should be present; both is an error. Absence for a domain name is simply the presence of the root label. Absence for regexp is an empty text string.
	}

	#[inline(always)]
	fn handle_kx<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
	{
		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;
		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		if unlikely!(resource_data.len() < 3)
		{
			return Err(DnsProtocolError::ResourceDataForTypeKXHasTooShortALength(resource_data.len()))
		}

		let record = KeyExchange
		{
			preference: u16::from_be_bytes(unsafe { *(resource_data.as_ptr() as *const [u8; 2]) }),
			mail_server_name: parsed_labels.parse_name_in_slice_with_nothing_left(&resource_data[2 .. ])?,
		};

		resource_record_visitor.KX(resource_record_name, time_to_live, record)?;
		Ok(Self::end_of_resource_data_pointer(resource_data))
	}

	#[inline(always)]
	fn handle_opt(&self, end_of_name_pointer: usize, end_of_message_pointer: usize, processing_additional_record_section: bool, have_already_seen_an_edns_opt_record: bool, is_a_response: bool) -> Result<usize, DnsProtocolError>
	{
		use self::DnsProtocolError::*;

		if !processing_additional_record_section
		{
			return Err(ExtendedDnsOptRecordOutsideOfAdditionalDataSection)
		}

		if have_already_seen_an_edns_opt_record
		{
			return Err(MoreThanOneExtendedDnsOptRecord)
		}

		let start_of_name_pointer = self.name() as *const Name as usize;
		if unlikely!(end_of_name_pointer - start_of_name_pointer != 1)
		{
			return Err(ExtendedDnsOptRecordNameTooLong)
		}

		let name_length_or_offset = unsafe { * (self.name() as *const Name as *const u8) };
		if unlikely!(name_length_or_offset != 0x00)
		{
			return Err(ExtendedDnsOptRecordNameNotRoot)
		}

		let requestors_udp_payload_size = self.requestors_udp_payload_size(end_of_name_pointer);
		// TODO: This value is supposed to be a minimum of 512 bytes.
		x

		let extended_response_code_and_flags = self.extended_response_code_and_flags(end_of_name_pointer);

		let upper_8_bits = extended_response_code_and_flags.extended_response_code_upper_8_bits();
		// TODO: Any value that isn't zero is effectively an error.


		let version = extended_response_code_and_flags.version()?;
		debug_assert_eq!(version, ExtendedDnsVersion::Version0, "Why do we support EDNS versions other than 0?");

		let dnssec_ok = extended_response_code_and_flags.dnssec_ok();

		extended_response_code_and_flags.z()?;

		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		// 2-byte option code, 2-byte option length
		let mut current_pointer = resource_data.as_ptr() as usize;
		let end_of_options_pointer = current_pointer + resource_data.len();
		while current_pointer != end_of_options_pointer
		{
			if unlikely!(current_pointer + 4 > end_of_options_pointer)
			{
				return Err(ExtendedDnsOptionTooShort)
			}

			let option_code = u16::from_be_bytes(unsafe { * (current_pointer as *const [u8; 2]) });

			#[inline(always)]
			fn validate_raw_option_data<'a>(current_pointer: usize) -> Result<&'a [u8], DnsProtocolError>
			{
				let option_length = u16::from_be_bytes(unsafe { * ((current_pointer + 2) as *const [u8; 2]) }) as usize;
				let option_data_pointer = current_pointer + 4;
				if unlikely!(option_data_pointer + option_length > end_of_options_pointer)
				{
					Err(ExtendedDnsOptionDataOverflows)
				}
				else
				{
					Ok(unsafe { from_raw_parts(option_data_pointer as *const u8, option_length) })
				}
			}

			const DAU: u16 = 5;
			const DHU: u16 = 6;
			const N3U: u16 = 7;

			// Authoritative servers MUST NOT set the DAU, DHU, and/or N3U option(s) on any responses.  These values are only set in queries.
			match option_code
			{
				0 | 65001 ... 65535 => return Err(ExtendedDnsOptionCodeWasReserved(option_code)),

				// RFC 6975, Section 6, Paragraph 4: "Authoritative servers MUST NOT set the DAU, DHU, and/or N3U option(s) on any responses.
				// These values are only set in queries".
				DAU => if likely!(is_a_response)
				{
					return Err(ExtendedDnsOptionDAUMustOnlyBeSetInARequest)
				}
				else
				{
					let option_data = validate_raw_option_data(current_pointer)?;
					// 2-byte length
					// list of 1-byte algorithm codes
				}

				// RFC 6975, Section 6, Paragraph 4: "Authoritative servers MUST NOT set the DAU, DHU, and/or N3U option(s) on any responses.
				// These values are only set in queries".
				DHU => if likely!(is_a_response)
				{
					return Err(ExtendedDnsOptionDHUMustOnlyBeSetInARequest)
				}
				else
				{
					let option_data = validate_raw_option_data(current_pointer)?;
					// 2-byte length
					// list of 1-byte algorithm codes
				}

				// RFC 6975, Section 6, Paragraph 4: "Authoritative servers MUST NOT set the DAU, DHU, and/or N3U option(s) on any responses.
				// These values are only set in queries".
				N3U => if likely!(is_a_response)
				{
					return Err(ExtendedDnsOptionN3UMustOnlyBeSetInARequest)
				}
				else
				{
					let option_data = validate_raw_option_data(current_pointer)?;
					// 2-byte length
					// list of 1-byte algorithm codes
				}

				_ =>
				{
					let option_data = validate_raw_option_data(current_pointer)?;
				}
			}
		}
	}

	#[inline(always)]
	fn handle_dname<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.parse_name_only(end_of_name_pointer, end_of_message_pointer, parsed_labels)?;
		resource_record_visitor.DNAME(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_sshfp<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		use self::DnsProtocolError::*;

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
			0x01 ... 0x04 => unsafe { transmute(raw_public_key_algorithm) },

			_ => return Err(ResourceDataForTypeSSHFPHasAnUnrecognisedPublicKeyAlgorithm(raw_public_key_algorithm)),
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

			resource_record_visitor.SSHFP(resource_record_name, time_to_live, record)?;
			Ok(Self::end_of_resource_data_pointer(resource_data))
		}
	}

	#[inline(always)]
	fn handle_ipseckey<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{

		// IPSECKEY  https://tools.ietf.org/html/rfc4025#section-2.1
		// Name must not be compressed.


		xxxx
	}

	#[inline(always)]
	fn handle_openpgpkey<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;

		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		resource_record_visitor.OPENPGPKEY(resource_record_name, time_to_live, resource_data)?;
		Ok(Self::end_of_resource_data_pointer(resource_data))
	}
	#[inline(always)]
	fn handle_tlsa<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.handle_tlsa_or_smimea(end_of_name_pointer, end_of_message_pointer)?;
		resource_record_visitor.TLSA(resource_record_name, time_to_live, record)?;
		Ok(Self::end_of_resource_data_pointer(resource_data))
	}

	#[inline(always)]
	fn handle_smimea<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.handle_tlsa_or_smimea(end_of_name_pointer, end_of_message_pointer)?;
		resource_record_visitor.SMIMEA(resource_record_name, time_to_live, record)?;
		Ok(Self::end_of_resource_data_pointer(resource_data))
	}

	#[inline(always)]
	fn handle_tlsa_or_smimea<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize) -> Result<(TimeToLiveInSeconds, DnsBasedAuthenticationOfNamedEntities<'a>, usize), DnsProtocolError>
	{
		use self::DnsProtocolError::*;

		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;

		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		let length = resource_data.len();
		if unlikely!(length < 3)
		{
			return Err(ResourceDataForTypeTLSAHasAnIncorrectLength(length))
		}

		let raw_certificate_usage = unsafe { * resource_data.get_unchecked(0) };
		let certificate_usage: CertificateUsage = match raw_certificate_usage
		{
			0x00 ... 0x03 => unsafe { transmute(raw_certificate_usage) },

			_ => return Err(ResourceDataForTypeTLSAHasAnUnrecognisedCertificateUsage(raw_certificate_usage)),
		};

		let raw_selector = unsafe { * resource_data.get_unchecked(1) };
		let selector: Selector = match raw_selector
		{
			0x00 ... 0x01 => unsafe { transmute(raw_selector) },

			_ => return Err(ResourceDataForTypeTLSAHasAnUnrecognisedSelector(raw_selector)),
		};

		let certificate_association_data = &resource_data[3 .. ];

		#[inline(always)]
		fn validate_hash_digest(certificate_association_data: &[u8], matching_type: MatchingType, digest_size_in_bits: usize) -> Result<MatchingType, DnsProtocolError>
		{
			if unlikely!(certificate_association_data.len() != digest_size_in_bits / 8)
			{
				Err(ResourceDataForTypeTLSAHasADigestLengthThatIsIncorrectForTheMatchingType(matching_type, certificate_association_data.len()))
			}
			else
			{
				Ok(matching_type)
			}
		}

		use self::MatchingType::*;
		let raw_matching_type = unsafe { * resource_data.get_unchecked(2) };
		let matching_type = match raw_matching_type
		{
			0x00 => NoHashUsed,

			0x01 => validate_hash_digest(certificate_association_data, Sha2_256, 256)?,

			0x02 => validate_hash_digest(certificate_association_data, Sha2_512, 512)?,

			_ => return Err(ResourceDataForTypeTLSAHasAnUnrecognisedMatchingType(raw_matching_type)),
		};

		Ok
		(
			time_to_live,
			DnsBasedAuthenticationOfNamedEntities
			{
				certificate_usage,
				selector,
				matching_type,
				certificate_association_data,
			},
			Self::end_of_resource_data_pointer(resource_data)
		)
	}

	#[inline(always)]
	fn handle_unsupported<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>, unsupported_resource_record_type: DataType) -> Result<usize, DnsProtocolError>
	{
		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;

		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		resource_record_visitor.unsupported(resource_record_name, time_to_live, resource_data, parsed_labels, unsupported_resource_record_type)?;
		Ok(Self::end_of_resource_data_pointer(resource_data))
	}

	#[inline(always)]
	fn parse_internet_protocol_address_only<'a, Address: 'a + Sized>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize) -> Result<(TimeToLiveInSeconds, &'a Address, usize), DnsProtocolError>
	{
		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;

		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;
		let length = resource_data.len();
		if unlikely!(length != size_of::<Address>())
		{
			Err(DnsProtocolError::ResourceDataForTypeAOrAAAAHasAnIncorrectLength(length))
		}
		else
		{
			let address = unsafe { &*(resource_data.as_ptr() as *const Address) };
			Ok((time_to_live, address, Self::end_of_resource_data_pointer(resource_data)))
		}
	}

	#[inline(always)]
	fn parse_name_only<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, parsed_labels: &mut ParsedLabels<'a>) -> Result<(TimeToLiveInSeconds, ParsedNameIterator<'a>, usize), DnsProtocolError>
	{
		let time_to_live = self.validate_class_and_get_time_to_live(end_of_name_pointer)?;
		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;
		let record = parsed_labels.parse_name_in_slice_with_nothing_left(resource_data)?;

		Ok((time_to_live, record, Self::end_of_resource_data_pointer(resource_data)))
	}

	#[inline(always)]
	fn validate_class_and_get_time_to_live(&self, end_of_name_pointer: usize) -> Result<TimeToLiveInSeconds, DnsProtocolError>
	{
		let class = self.resource_record_class(end_of_name_pointer)?;
		debug_assert_eq!(class, ResourceRecordClass::Internet, "Why do we support classes other than Internet?");

		Ok(self.time_to_live(end_of_name_pointer))
	}

	#[inline(always)]
	fn safely_access_resource_data(&self, end_of_name_pointer: usize, end_of_message_pointer: usize) -> Result<&[u8], DnsProtocolError>
	{
		let resource_data_length = self.resource_data_length(end_of_name_pointer) as usize;
		if unlikely!(end_of_name_pointer + resource_data_length > end_of_message_pointer)
		{
			Err(DnsProtocolError::ResourceDataLengthOverflows)
		}
		else
		{
			Ok(unsafe { from_raw_parts(self.resource_data(end_of_name_pointer) as *const ResourceData as *const u8, resource_data_length) })
		}
	}

	#[inline(always)]
	fn end_of_resource_data_pointer(resource_data: &[u8]) -> usize
	{
		resource_data.as_ptr() as usize + resource_data.len()
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
	fn name_mutable(&mut self) -> &mut Name
	{
		unsafe { &mut * (self as *mut Self as *mut Name) }
	}

	/// `TYPE` field.
	#[inline(always)]
	fn resource_record_type(&self, end_of_name_pointer: usize) -> DataType
	{
		self.footer(end_of_name_pointer).resource_record_type()
	}

	/// `TYPE` field.
	#[inline(always)]
	fn set_resource_record_type(&mut self, end_of_name_pointer: usize, resource_record_type: DataType)
	{
		self.footer_mutable(end_of_name_pointer).set_resource_record_type(resource_record_type)
	}

	/// `CLASS` field.
	#[inline(always)]
	fn resource_record_class(&self, end_of_name_pointer: usize) -> Result<ResourceRecordClass, DnsProtocolError>
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
		self.footer(end_of_name_pointer).resource_data_length()
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


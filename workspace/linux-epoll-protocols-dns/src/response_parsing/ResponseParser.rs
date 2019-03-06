// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ResponseHeader
{
	is_authenticated_data: bool,
	is_authoritative_answer: bool,
	is_recursion_available: bool,
}

pub enum Outcome
{
	Normal,
	DnsSecDataFailedAuthentication,
	AuthoritativeServerReportsNoDomainButThisIsNotValidated,
}

impl RequestQueryIdentification
{
	pub(crate) fn matches<'a>(&self, qname: WithoutCompressionParsedNameIterator<'a>, data_type: DataType) -> Result<(), DnsProtocolError>
	{
		if unlikely!(self.data_type != data_type)
		{
			return Err(ResponseWasForADifferentDataType)
		}

		if unlikely!(self.query_name_labels_excluding_root.len() != qname.number_of_labels as usize)
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

struct RequestQuery<'a>
{
	answer_section: Box<dyn ResourceRecordVisitor<'a>>,
}

pub struct OutstandingRequests<'a>
{
	requests_by_identifier: HashMap<MessageIdentifier, (RequestQueryIdentification, RequestQuery<'a>)>
}

impl<'a> OutstandingRequests<'a>
{
	pub fn parse_slice_after_trimming_tcp_message_size_bytes<'message>(&mut self, raw_message: &'message mut [u8]) -> Result<Outcome, DnsProtocolError>
	{
		let message = raw_message.cast_mut::<Message>(0);
		let message_header = message.message_header();

		let identifier = message_header.identifier();
		let (request_query_identification, request_query) = match self.requests_by_identifier.remove(&identifier)
		{
			// TODO: Add this DnsProtocolError!
			// This MAY be possible for timed-out queries we later throw away, but I suspect a better technique if a query times out is to just discard the entire set of outstanding requests and re-init the connection.
			None => panic!("FIX ME"),

			//return Err(ResponseWasForAnUnknownRequest(identifier)),
			Some((request_query_identification, request_query)) => (request_query_identification, request_query),
		};

		if unlikely!(message_header.query_response() != MessageType::Response)
		{
			return Err(ResponseWasAQuery)
		}

		message_header.parse_number_of_entries_in_the_question_section()?;

		let start_of_message_pointer = raw_message.start_pointer();
		let end_of_message_pointer = raw_message.end_pointer();
		let mut parsed_labels = ParsedLabels::new(start_of_message_pointer);

		let (next_resource_record_pointer, data_type) = message.message_body_as_query_section_entry().parse_response(&mut parsed_labels, end_of_message_pointer, request_query_identification)?;

		macro_rules! validate_opcode
		{
			($message_header: ident) =>
			{
				match $message_header.raw_opcode()
				{
					MessageOpcode::Query => (),
					MessageOpcode::InverseQuery => return Err(InvalidResponseOpcode(MessageOpcode::InverseQuery)),
					MessageOpcode::Status => return Err(InvalidResponseOpcode(MessageOpcode::Status)),
					opcode @ 3 => return Err(UnassignedResponseOpcode(opcode)),
					MessageOpcode::Notify => return Err(InvalidResponseOpcode(MessageOpcode::Notify)),
					MessageOpcode::Update => return Err(InvalidResponseOpcode(MessageOpcode::Update)),
					MessageOpcode::DnsStatefulOperations => return Err(InvalidResponseOpcode(MessageOpcode::DnsStatefulOperations)),
					opcode @ 7 ... 15 => return Err(UnassignedResponseOpcode(opcode)),
					_ => unreachable!(),
				}
			}
		}
		validate_opcode!(message_header);

		if unlikely!(!message_header.z())
		{
			return Err(ResponseUsedReservedHeaderBits)
		}

		if unlikely!(message_header.is_truncated())
		{
			return Err(ResponseIsTruncated)
		}

		if unlikely!(!message_header.recursion_desired())
		{
			return Err(ResponseFailedToCopyRecursionDesiredBit)
		}

		if unlikely!(message_header.checking_disabled())
		{
			return Err(ResponseFailedToCopyCheckingDisabledBit)
		}

		let is_authoritative_answer = message_header.authoritative_answer();
		let is_authenticated_data = message_header.authentic_data();

		if unlikely!(is_authoritative_answer)
		{
			if unlikely!(message_header.authentic_data())
			{
				return Err(ResponseWasAuthoritativeButHasTheAuthoritativeDataBitSet)
			}
		}

		macro_rules! validate_message_response_code
		{
			($message_header: ident, $is_authenticated_data: ident, $is_authoritative_answer: ident) =>
			{
				{
					use self::Outcome::*;

					match message_header.raw_response_code()
					{
						MessageResponseCode::NoError => Normal,

						MessageResponseCode::FormatError => return Err(MessageResponseCodeWasFormatError),

						MessageResponseCode::ServerFailure => if unlikely!(!$is_authenticated_data)
						{
							return Ok(DnsSecDataFailedAuthentication)
						}
						else
						{
							return Err(MessageResponseCodeWasServerFailure)
						},

						MessageResponseCode::NonExistentDomain => if unlikely!($is_authoritative_answer)
						{
							AuthoritativeServerReportsNoDomainButThisIsNotValidated
						}
						else
						{
							return Err(MessageResponseCodeWasNonExistentDomainForANonAuthoritativeServer)
						},

						MessageResponseCode::NotImplemented => return Err(MessageResponseCodeWasNotImplemented),

						MessageResponseCode::Refused => return Err(MessageResponseCodeWasRefused),

						MessageResponseCode::NameExistsWhenItShouldNot => return Err(MessageResponseCodeShouldNotBeDynamicDnsAssociated(MessageResponseCode::NameExistsWhenItShouldNot)),

						MessageResponseCode::ResourceRecordSetExistsWhenItShouldNot => return Err(MessageResponseCodeShouldNotBeDynamicDnsAssociated(MessageResponseCode::ResourceRecordSetExistsWhenItShouldNot)),

						MessageResponseCode::ResourceRecordSetThatShouldExistDoesNot => return Err(MessageResponseCodeShouldNotBeDynamicDnsAssociated(MessageResponseCode::ResourceRecordSetThatShouldExistDoesNot)),

						MessageResponseCode::ServerNotAuthoritativeForZoneOrNotAuthorized => return Err(MessageResponseCodeShouldNotBeDynamicDnsAssociated(MessageResponseCode::ServerNotAuthoritativeForZoneOrNotAuthorized)),

						MessageResponseCode::NameNotContainedInZone => return Err(MessageResponseCodeShouldNotBeDynamicDnsAssociated(MessageResponseCode::NameNotContainedInZone)),

						MessageResponseCode::DnsStatefulOperationsTypeNotImplemented => return Err(MessageResponseCodeShouldNotBeDnsStatefulOperationsTypeNotImplemented),

						response_code @ 12 ... 15 => return Err(MessageResponseCodeUnassigned(response_code)),

						_ => unreachable!(),
					}
				}
			}
		}

		let outcome = validate_message_response_code!(message_header, is_authenticated_data, is_authoritative_answer);

		let mut response_parsing_state = ResponseParsingState::default();


// in practice, we'll implement dedicated handlers for the various resource records of interest.
// A, AAAA, NS (no CNAMEs allowed), SOA, PTR (mostly useless on the modern internet), SRV, MX, ?KX, IPSECKEY, LOC, URI, CAA, TXT

// NS, SOA, PTR, MX, ?KX, SRV are not allowed to be aliases.
// Looking up A can give a CNAME.
// Looking up AAAA can give a CNAME.
// Looking up PTR can give a CNAME.


		// Look for the presence of a SOA record; use its minimum TTL or TTL (whichever is lower) to cache a negative response, with may be a sensible cap ontop, eg 1 hour, 5 minutes, etc.
		// See https://tools.ietf.org/html/rfc2308 Section 2.2 for various permutations; a CNAME (and presumably DNAME) is allowed in the answer section. This accommodates the lack of IPv6 for the cloudflare blog.
		// SOA name will be for the resolved CNAME (ie right-hand-side of CNAME) less host name or the original QNAME.
		// eg AAAA for blog.cloudflare.com CNAME cloudflare.ghost.io and  ghost.io SOA sara.ns.cloudflare.com ...
		// eg AAAA for cloudflare.ghost.io give no answers and   ghost.io SOA sara.ns.cloudflare.com ...

		// 3 types of negative NODATA response and a 4th which is a referral; all have to be inferred.
		// Negative responses without SOA records SHOULD NOT be cached.

		// Just about anything can be CNAME'd (DNAME'd), although it would seem odd for a SRV record to be redirected to _udp from _tcp.

		// PTR: Not supported - use https://securitytrails.com instead. It is useless for a security check and is often wrong.

		// CNAME chain limits; BIND uses 16 (and also to detect loops).

		/*
			query for A, get back bunch of A
				- hash_map.insert(A, As);

			query for A, get back just a CNAME
				- hash_map.insert(A, CNAME)
				- do a further query; need to be careful with circular CNAME chains and long CNAME chains

			query for A, get back CNAME and a bunch of As
				- hash_map.insert(A, CNAME)
				- hash_map.insert(A, As)

			query for A, get back CNAME and SOA in authority section
				- hash_map.insert(A, CNAME)
				- hash_map.insert_negative_cache(A, SOA); calculate cache TTL is the lower of RR TTL and the SOA minimum TTL (technically it should be SOA minimum TTL but RR TTL may be short implying the SOA record itself may have become updated)

			struct Cache

		*/

//		// expires_at_time = RR TTL + now(); OR lower of RR TTL / SOA min TTL + now();
//		type ExpiresAtTime = Timespec;
//
//		enum CacheEntry<ResourceRecord>
//		{
//			Alias(Name), // 'CNAME'. Done properly Name is a Rc or Arc.
//
//			NoSuchDomain, // 'SOA'.
//
//			Records(Vec<ResourceRecord>), // eg bunch of A records, ought to be sorted.
//		}
//
//		struct Cache
//		{
//			cname: HashMap<(Name, DataType), (expires_at_time, CacheEntry<CNAME>)>,
//			a: HashMap<(Name, DataType), (expires_at_time, CacheEntry<Ipv4Addr>)>,
//			aaaaa: HashMap<(Name, DataType), (expires_at_time, CacheEntry<Ipv6Addr>)>,
//			services: HashMap<(Name, DataType), (expires_at_time, CacheEntry<SRV>)>,
//			uri: HashMap<(Name, DataType), (expires_at_time, CacheEntry<URI>)>,
//			tlsa: HashMap<(Name, DataType), (expires_at_time, CacheEntry<TLSA>)>,
//			smimea: HashMap<(Name, DataType), (expires_at_time, CacheEntry<SMIMEA>)>,
//			// loc, sshfp, openpgpkey, ipseckey, mx, kx
//			//services HashMap<(Name, DataType), (expires_at_time, CacheEntry<SRV>)>,
//
//			// we need some sort of ordering to the inner so that when we reach a fixed capacity we know what to drop.
//			// use an IndexMap and just drop the last inserted (we've done something like this before).
//		}

		#[inline(always)]
		fn loop_over_resource_records(end_of_message_pointer: usize, next_resource_record_pointer: usize, number_of_resource_records: u16, parse_method: impl for<'a> Fn(&mut ResourceRecord, usize) -> Result<usize, DnsProtocolError>) -> Result<usize, DnsProtocolError>
		{
			let mut next_resource_record_pointer = next_resource_record_pointer;
			for _ in 0 .. number_of_resource_records
			{
				if unlikely!(next_resource_record_pointer == end_of_message_pointer)
				{
					return Err(ResourceRecordsOverflowAnswerSection)
				}
				let resource_record = next_resource_record_pointer.unsafe_cast_mut::<ResourceRecord>();
				next_resource_record_pointer = parse_method(resource_record, end_of_message_pointer)?;
			}
			Ok(next_resource_record_pointer)
		}

		// TODO: Fix this.
		let resource_record_visitor = XXXX;

		let next_resource_record_pointer = loop_over_resource_records(end_of_message_pointer, next_resource_record_pointer, message_header.number_of_resource_records_in_the_answer_section(), |resource_record, end_of_message_pointer| resource_record.parse_answer_section_resource_record_in_response(data_type, end_of_message_pointer, &mut parsed_labels, &mut resource_record_visitor, &mut response_parsing_state))?;

		let next_resource_record_pointer = loop_over_resource_records(end_of_message_pointer, next_resource_record_pointer, message_header.number_of_resource_records_in_the_authority_records_section(), |resource_record, end_of_message_pointer| resource_record.parse_authority_section_resource_record_in_response(end_of_message_pointer, &mut parsed_labels, &mut resource_record_visitor, &mut response_parsing_state))?;

		let next_resource_record_pointer = loop_over_resource_records(end_of_message_pointer, next_resource_record_pointer, message_header.number_of_resource_records_in_the_additional_records_section(), |resource_record, end_of_message_pointer| resource_record.parse_additional_section_resource_record_in_response(end_of_message_pointer, &mut parsed_labels, &mut DiscardingResourceRecordVisitor::default(), &mut response_parsing_state))?;

		match response_parsing_state.dnssec_ok
		{
			None => return Err(ResponseDoesNotSupportExtendedDns),
			Some(false) => return Err(ResponseIgnoredDnsSec),
			Some(true) => (),
		}

		let response_header = ResponseHeader
		{
			is_authenticated_data,
			is_authoritative_answer,
			is_recursion_available: message_header.recursion_available(),
		};

		Ok(outcome)
	}

	/*
		Request, outbound
			- EDNS(0) with DO bit set
				No EDNS options.
			- Zero CD bit
			- One AD bit
			- One RD bit
			- Zero RA bit
			- Zero TC bit
			- error code always zero.
			- QCLASS always IN
			- Always one query and no additional records.
			- May need a client certificate.
	*/
}

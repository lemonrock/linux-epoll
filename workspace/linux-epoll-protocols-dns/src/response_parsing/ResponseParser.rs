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

struct RequestQuery<'a>
{
	answer_section: Box<dyn ResourceRecordVisitor<'a>>,
}

pub struct OutstandingRequests<'a>
{
	requests_by_identifier: HashMap<MessageIdentifier, (RequestQueryIdentification, RequestQuery<'a>)>
}

macro_rules! validate_number_of_entries_in_the_question_section_is_one
{
	($message_header: ident) =>
	{
		{
			let number_of_entries_in_the_question_section = $message_header.number_of_entries_in_the_question_section();
			if unlikely!(number_of_entries_in_the_question_section != 1)
			{
				return Err(ResponseDoesNotContainExactlyOneQuestion(number_of_entries_in_the_question_section))
			}
		}
	}
}

macro_rules! validate_is_response
{
	($message_header: ident) =>
	{
		if unlikely!($message_header.is_query())
		{
			return Err(ResponseWasAQuery)
		}
	}
}

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

macro_rules! validate_reserved_header_bits_are_zero
{
	($message_header: ident) =>
	{
		if unlikely!(!$message_header.z())
		{
			return Err(ResponseUsedReservedHeaderBits)
		}
	}
}

macro_rules! validate_response_is_not_truncated
{
	($message_header: ident) =>
	{
		if unlikely!(!$message_header.is_truncated())
		{
			return Err(ResponseIsTruncated)
		}
	}
}

macro_rules! validate_recursion_desired_bit_was_copied_from_query_and_is_one
{
	($message_header: ident) =>
	{
		if unlikely!(!$message_header.recursion_desired())
		{
			return Err(ResponseFailedToCopyRecursionDesiredBit)
		}
	}
}

macro_rules! validate_checking_bit_was_copied_from_query_and_is_zero
{
	($message_header: ident) =>
	{
		if unlikely!(!$message_header.recursion_desired())
		{
			return Err(ResponseFailedToCopyCheckingDisabledBit)
		}
	}
}

macro_rules! validate_authentic_answers_do_not_have_authoritative_data_bit_set
{
	($message_header: ident) =>
	{
		{
			let is_authoritative_answer = $message_header.authoritative_answer();
			let is_authenticated_data = $message_header.authentic_data();

			if unlikely!(is_authoritative_answer)
			{
				if unlikely!(is_authenticated_data)
				{
					return Err(ResponseWasAuthoritativeButHasTheAuthoritativeDataBitSet)
				}
			}
			(is_authoritative_answer, is_authenticated_data)
		}
	}
}

macro_rules! validate_message_response_code
{
	($message_header: ident, $is_authoritative_answer: ident, $is_authenticated_data: ident) =>
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

impl<'message> OutstandingRequests<'message>
{
	pub fn parse_slice_after_trimming_tcp_message_size_bytes<'message>(&mut self, raw_message: &'message mut [u8]) -> Result<Outcome, DnsProtocolError>
	{
		let message = raw_message.cast_mut::<Message>(0);
		let message_header = message.message_header();

		let identifier = message_header.identifier();
		let (request_query_identification, request_query) = match self.requests_by_identifier.remove(&identifier)
		{
			// TODO: This MAY be possible for timed-out queries we later throw away, but I suspect a better technique if a query times out is to just discard the entire set of outstanding requests and re-init the connection.

			// RFC 2308 Section 7.1: "In either case a resolver MAY cache a server failure response.
			// If it does so it MUST NOT cache it for longer than five (5) minutes, and it MUST be cached against the specific query tuple <query name, type, class, server IP address>".

			// RFC 2308 Section 7.2: "A server may be deemed to be dead or unreachable if it has not responded to an outstanding query within 120 seconds.
			// ...
			// A server MAY cache a dead server indication.
			// If it does so it MUST NOT be deemed dead for longer than five (5) minutes".
			None => return Err(ResponseWasForAnUnknownRequest(identifier)),

			Some((request_query_identification, request_query)) => (request_query_identification, request_query),
		};

		validate_is_response!(message_header);
		validate_number_of_entries_in_the_question_section_is_one!(message_header);
		validate_opcode!(message_header);
		validate_reserved_header_bits_are_zero!(message_header);
		validate_response_is_not_truncated!(message_header);
		validate_recursion_desired_bit_was_copied_from_query_and_is_one!(message_header);
		validate_checking_bit_was_copied_from_query_and_is_zero!(message_header);
		let (is_authoritative_answer, is_authenticated_data) = validate_authentic_answers_do_not_have_authoritative_data_bit_set!(message_header);
		let outcome = validate_message_response_code!(message_header, is_authoritative_answer, is_authenticated_data);

		let start_of_message_pointer = raw_message.start_pointer();
		let end_of_message_pointer = raw_message.end_pointer();
		let mut parsed_labels = ParsedLabels::new(start_of_message_pointer);

		let (next_resource_record_pointer, data_type) = message.message_body_as_query_section_entry().parse_response(&mut parsed_labels, end_of_message_pointer, request_query_identification)?;



		Self::response_record_section_parsing(end_of_message_pointer, next_resource_record_pointer, message_header, &mut parsed_labels, data_type)?;


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

	fn response_record_section_parsing(end_of_message_pointer: usize, next_resource_record_pointer: usize, message_header: &MessageHeader, parsed_labels: &mut ParsedLabels, data_type: DataType) -> Result<AnswerOutcome, DnsProtocolError>
	{
		let mut response_parsing_state = ResponseParsingState::default();

		let (next_resource_record_pointer, canonical_name_chain) = Self::parse_answer_section(end_of_message_pointer, next_resource_record_pointer, message_header, parsed_labels, &mut response_parsing_state, data_type)?;

		let (next_resource_record_pointer, answer_outcome) = Self::parse_authority_section(end_of_message_pointer, next_resource_record_pointer, message_header, parsed_labels, &mut response_parsing_state, canonical_name_chain)?;

		Self::parse_additional_section(end_of_message_pointer, next_resource_record_pointer, message_header, parsed_labels, response_parsing_state)?;

		Ok(answer_outcome)
	}

	#[inline(always)]
	fn parse_answer_section<'message>(end_of_message_pointer: usize, next_resource_record_pointer: usize, message_header: &MessageHeader, parsed_labels: &mut ParsedLabels, response_parsing_state: &mut ResponseParsingState, data_type: DataType) -> Result<(usize, CanonicalNameChain<'message>), DnsProtocolError>
	{
		let number_of_resource_records = message_header.number_of_resource_records_in_the_authority_records_section();

		// TODO: Fix this.
		let resource_record_visitor = XXXX;
		let canonical_name_chain = XXXX;

		let next_resource_record_pointer = Self::loop_over_resource_records(end_of_message_pointer, next_resource_record_pointer, number_of_resource_records, |response_record_section_parsing, resource_record, end_of_message_pointer| resource_record.parse_answer_section_resource_record_in_response(data_type, end_of_message_pointer, parsed_labels, &mut resource_record_visitor, response_parsing_state))?;


		Ok(next_resource_record_pointer, canonical_name_chain)
	}

	#[inline(always)]
	fn parse_authority_section<'message>(end_of_message_pointer: usize, next_resource_record_pointer: usize, message_header: &MessageHeader, parsed_labels: &mut ParsedLabels, response_parsing_state: &mut ResponseParsingState, canonical_name_chain: CanonicalNameChain<'message>) -> Result<(usize, AnswerOutcome), DnsProtocolError>
	{
		let number_of_resource_records = message_header.number_of_resource_records_in_the_authority_records_section();

		let mut authority_resource_record_visitor = AuthorityResourceRecordVisitor::new(canonical_name_chain);

		let next_resource_record_pointer = Self::loop_over_resource_records(end_of_message_pointer, next_resource_record_pointer, number_of_resource_records, |response_record_section_parsing, resource_record, end_of_message_pointer| resource_record.parse_authority_section_resource_record_in_response(end_of_message_pointer, parsed_labels, &mut authority_resource_record_visitor, response_parsing_state))?;

		let answer_outcome = authority_resource_record_visitor.answer_outcome(is_authoritative_answer, has_nxdomain_error_code, answer_section_has_at_least_one_record_of_requested_data_type);

		Ok((next_resource_record_pointer, answer_outcome))
	}

	#[inline(always)]
	fn parse_additional_section(end_of_message_pointer: usize, next_resource_record_pointer: usize, message_header: &MessageHeader, parsed_labels: &mut ParsedLabels, mut response_parsing_state: ResponseParsingState) -> Result<(), DnsProtocolError>
	{
		let number_of_resource_records = message_header.number_of_resource_records_in_the_additional_records_section();

		let mut discarding_resource_record_visitor = DiscardingResourceRecordVisitor::default();

		let next_resource_record_pointer = Self::loop_over_resource_records(end_of_message_pointer, next_resource_record_pointer, number_of_resource_records, |response_record_section_parsing, resource_record, end_of_message_pointer| resource_record.parse_additional_section_resource_record_in_response(end_of_message_pointer, parsed_labels, &mut discarding_resource_record_visitor, &mut response_parsing_state))?;

		if unlikely!(response_parsing_state.have_yet_to_see_an_edns_opt_resource_record)
		{
			return Err(ResponseDidNotContainAnExtendedDnsOptMetaResourceRecord)
		}

		match response_parsing_state.dnssec_ok
		{
			None => Err(ResponseDoesNotSupportExtendedDns),

			Some(false) => Err(ResponseIgnoredDnsSec),

			Some(true) => Ok(()),
		}
	}

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
}

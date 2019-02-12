// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A DNS message header.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
pub struct MessageHeader
{
	identifier: MessageIdentifer,
	bitfield1: MessageBitField1,
	bitfield2: MessageBitField2,
	qd_count: [u8; 2],
	an_count: [u8; 2],
	ns_count: [u8; 2],
	ar_count: [u8; 2],
}

impl MessageHeader
{
	/// `ID` field.
	#[inline(always)]
	pub fn identifier(&self) -> MessageIdentifer
	{
		self.identifier
	}

	/// `QR` field.
	///
	/// A one bit field that specifies whether this message is a query or a response.
	#[inline(always)]
	pub fn query_response(self) -> MessageType
	{
		self.bitfield1.query_response()
	}

	/// `Opcode` field.
	///
	/// A four bit field that specifies kind of query in this message.
	///
	/// This value is set by the originator of a query and copied into the response.
	///
	/// Only values 0 - 6 inclusive are defined by IANA, and some of those are for drafts, reserved or obsolete.
	#[inline(always)]
	pub fn raw_opcode(self) -> u8
	{
		self.bitfield1.raw_opcode()
	}

	/// `Opcode` field.
	///
	/// A four bit field that specifies kind of query in this message.
	///
	/// This value is set by the originator of a query and copied into the response.
	///
	/// Only values 0 - 6 inclusive are defined by IANA, and some of those are for drafts, reserved or obsolete.
	#[inline(always)]
	pub fn assumed_to_be_valid_opcode(self) -> MessageOpcode
	{
		self.bitfield1.assumed_to_be_valid_opcode()
	}

	/// `AA` field.
	///
	/// This bit is valid in responses, and specifies that the responding name server is an authority for the domain name in question section.
	///
	/// Note that the contents of the answer section may have multiple owner names because of aliases.
	///
	/// The `AA` field corresponds to the name which matches the query name, or the first owner name in the answer section.
	#[inline(always)]
	pub fn authoritative_answer(self) -> bool
	{
		self.bitfield1.authoritative_answer()
	}

	/// `TC` field.
	///
	/// Is this message truncated due to limitations on packet sizes on the underlying transport?
	#[inline(always)]
	pub fn is_truncated(self) -> bool
	{
		self.bitfield1.truncation()
	}

	/// `RD` field.
	///
	/// This bit may be set in a query and is copied into the response.
	///
	/// If `RD` is set, it directs the name server to pursue the query recursively.
	///
	/// Recursive query support is optional.
	#[inline(always)]
	pub fn recursion_desired(self) -> bool
	{
		self.bitfield1.recursion_desired()
	}

	/// `RA` field.
	///
	/// This bit is set or cleared in a response, and denotes whether recursive query support is available in the name server.
	#[inline(always)]
	pub fn recursion_available(self) -> bool
	{
		self.bitfield2.recursion_available()
	}

	/// `Z` field.
	///
	/// Reserved for future use.
	///
	/// Must be zero in all queries and responses.
	#[inline(always)]
	pub fn z(self) -> bool
	{
		self.bitfield2.z()
	}

	/// `AD` field.
	///
	/// Defined in RFC 2535.
	///
    /// From [RFC 4035, DNSSEC Resource Records, March 2005](https://tools.ietf.org/html/rfc4035#section-3.1.6):-
    ///
    /// ```text
    ///
    /// 3.1.6.  The AD and CD Bits in an Authoritative Response
    ///
    ///   The CD and AD bits are designed for use in communication between
    ///   security-aware resolvers and security-aware recursive name servers.
    ///   These bits are for the most part not relevant to query processing by
    ///   security-aware authoritative name servers.
    ///
    ///   A security-aware name server does not perform signature validation
    ///   for authoritative data during query processing, even when the CD bit
    ///   is clear.  A security-aware name server SHOULD clear the CD bit when
    ///   composing an authoritative response.
    ///
    ///   A security-aware name server MUST NOT set the AD bit in a response
    ///   unless the name server considers all RRsets in the Answer and
    ///   Authority sections of the response to be authentic.  A security-aware
    ///   name server's local policy MAY consider data from an authoritative
    ///   zone to be authentic without further validation.  However, the name
    ///   server MUST NOT do so unless the name server obtained the
    ///   authoritative zone via secure means (such as a secure zone transfer
    ///   mechanism) and MUST NOT do so unless this behavior has been
    ///   configured explicitly.
    ///
    ///   A security-aware name server that supports recursion MUST follow the
    ///   rules for the CD and AD bits given in Section 3.2 when generating a
    ///   response that involves data obtained via recursion.
	/// ```
	#[inline(always)]
	pub fn authentic_data(self) -> bool
	{
		self.bitfield2.authentic_data()
	}

	/// `CD` field.
	///
	/// Defined in RFC 2535.
	///
	/// See documentation for `authentic_data()`.
	#[inline(always)]
	pub fn checking_disabled(self) -> bool
	{
		self.bitfield2.checking_disabled()
	}

	/// `RCODE` field.
	///
	/// A four bit field that specifies the response outcome.
	#[inline(always)]
	pub fn raw_response_code(self) -> u8
	{
		self.bitfield2.raw_rcode()
	}

	/// `RCODE` field.
	///
	/// A four bit field that specifies the response outcome.
	#[inline(always)]
	pub fn assumed_to_be_valid_response_code(self) -> MessageRCode
	{
		self.bitfield2.assumed_to_be_valid_rcode()
	}

	/// `QDCOUNT` field.
	///
	/// The number of entries in the question section.
	///
	/// Only a value of 1 is normally encountered.
	#[inline(always)]
	pub fn number_of_entries_in_the_question_section(self) -> u16
	{
		u16::from_be_bytes(self.qd_count)
	}

	/// `ANCOUNT` field.
	///
	/// The number of resource records in the answer section.
	#[inline(always)]
	pub fn number_of_resource_records_in_the_answer_section(self) -> u16
	{
		u16::from_be_bytes(self.an_count)
	}

	/// `NSCOUNT` field.
	///
	/// The number of name server resource records in the authority records section.
	#[inline(always)]
	pub fn number_of_name_server_resource_records_in_the_authority_records_section(self) -> u16
	{
		u16::from_be_bytes(self.ns_count)
	}

	/// `ARCOUNT` field.
	///
	/// The number of resource records in the additional records section.
	#[inline(always)]
	pub fn number_of_resource_records_in_the_additional_records_section(self) -> u16
	{
		u16::from_be_bytes(self.ar_count)
	}
}

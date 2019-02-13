// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Visits different kinds of records.
pub trait ResourceRecordVisitor
{
	/// Visits a record of type `A`, which may not be aligned (this is a problem for 32-bit ARM).
	fn A<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: &Ipv4Addr) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `NS`, which may not be aligned (this is a problem for 32-bit ARM).
	fn NS<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: ParsedNameIterator<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `SOA`, which may not be aligned (this is a problem for 32-bit ARM).
	fn SOA<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: StartOfAuthority<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `CNAME`, which may not be aligned (this is a problem for 32-bit ARM).
	fn CNAME<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: ParsedNameIterator<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `PTR`, which may not be aligned (this is a problem for 32-bit ARM).
	fn PTR<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: ParsedNameIterator<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `MX`, which may not be aligned (this is a problem for 32-bit ARM).
	fn MX<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: MailExchange<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `HINFO`, which may not be aligned (this is a problem for 32-bit ARM).
	///
	/// `HINFO` had been brought back into use by RFC 8482.
	fn HINFO<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: HostInformation<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `TXT`.
	fn TXT<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: TextStringsIterator) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `AAAA`, which may not be aligned (this is a problem for 32-bit ARM).
	fn AAAA<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: &Ipv6Addr) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `LOC`, which may not be aligned (this is a problem for 32-bit ARM).
	fn LOC<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: &Location) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `SRV`, which may not be aligned (this is a problem for 32-bit ARM).
	fn SRV<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: Service) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `KX`, which may not be aligned (this is a problem for 32-bit ARM).
	fn KX<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: KeyExchange<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `DNAME`.
	fn DNAME<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: ParsedNameIterator<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `SSHFP`.
	fn SSHFP<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: PublicKeyFingerprint<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `OPENPGPKEY`.
	fn OPENPGPKEY<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: OpenPgpRfc4880TransferablePublicKey<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `TLSA`.
	fn TLSA<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: DnsBasedAuthenticationOfNamedEntities<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `SMIMEA`.
	fn SMIMEA<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: DnsBasedAuthenticationOfNamedEntities<'a>) -> Result<(), DnsProtocolError>;

	/// Visits an unsupported record type.
	///
	/// Default implementation ignores it.
	#[inline(always)]
	fn unsupported<'a>(&mut self, _name: ParsedNameIterator<'a>, _time_to_live: TimeToLiveInSeconds, _record: &'a [u8], _parsed_labels: &mut ParsedLabels<'a>, _unsupported_resource_record_type: DataType) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}
}

/*
https://tools.ietf.org/html/rfc6840#section-2

[RFC5155] describes the use and behavior of the NSEC3 and NSEC3PARAM
   records for hashed denial of existence.  Validator implementations
   are strongly encouraged to include support for NSEC3 because a number
   of highly visible zones use it.  Validators that do not support
   validation of responses using NSEC3 will be hampered in validating
   large portions of the DNS space.

resolvers MUST ignore the DO bit in responses even if they set in in requests due to broken implementations

*/

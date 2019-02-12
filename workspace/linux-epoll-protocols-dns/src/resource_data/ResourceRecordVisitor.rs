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

	/// Visits a record of type `AAAA`, which may not be aligned (this is a problem for 32-bit ARM).
	fn AAAA<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: &Ipv6Addr) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `LOC`, which may not be aligned (this is a problem for 32-bit ARM).
	fn LOC<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: &Location) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `TXT`.
	fn TXT<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: TextStringsIterator) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `SSHFP`.
	fn SSHFP<'a>(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: PublicKeyFingerprint<'a>) -> Result<(), DnsProtocolError>;
}

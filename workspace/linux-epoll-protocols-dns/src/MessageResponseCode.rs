// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Original 4-bit response code (RCODE).
///
/// Response codes 0 - 5 inclusive are for regular DNS; codes 6 - 10 inclusive are for dynamic DNS.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MessageResponseCode
{
	/// No Error ('NoError').
	///
	/// Defined in RFC 1035.
	NoError = 0,

	/// Format Error ('FormErr').
	///
	/// The name server was unable to interpret the query
	///
	/// Defined in RFC 1035.
	FormatError = 1,

	/// Server Failure ('ServFail').
	///
	/// The name server was unable to process this query due to a problem with the name server.
	///
	/// Defined in RFC 1035.
	ServerFailure = 2,

	/// Non-Existent Domain ('NXDomain').
	///
	/// Meaningful only for responses from an authoritative name server, this code signifies that the domain name referenced in the query does not exist.
	///
	/// Defined in RFC 1035.
	NonExistentDomain = 3,

	/// Not Implemented ('NotImp').
	///
	/// The name server does not support the requested kind of query.
	///
	/// Defined in RFC 1035.
	NotImplemented = 4,

	/// Refused ('Refused').
	///
	/// The name server refuses to perform the specified operation for policy reasons.
	/// For example, a name server may not wish to provide the information to the particular requester, or a name server may not wish to perform a particular operation (eg a zone transfer) for particular data.
	///
	/// Defined in RFC 1035.
	Refused = 5,

	/// Name exists when it should not ('YXDomain').
	///
	/// Defined in RFC 2136 and RFC 6672.
	NameExistsWhenItShouldNot = 6,

	/// Resource record (RR) set exists when it should not ('YXRRSet').
	///
	/// Defined in RFC 2136.
	ResourceRecordSetExistsWhenItShouldNot = 7,

	/// Resource record (RR) set that should exist does not ('NXRRSet').
	///
	/// Defined in RFC 2136.
	ResourceRecordSetThatShouldExistDoesNot = 8,

	/// Server not authoritative for zone or not authorized ('NotAuth').
	///
	/// Defined in RFC 2136 and RFC 2845.
	ServerNotAuthoritativeForZoneOrNotAuthorized = 9,

	/// Name not contained in zone ('NotZone').
	///
	/// Defined in RFC 2136.
	NameNotContainedInZone = 10,

	/// DNS Stateful Operations TYPE (DSO-TYPE) not implemented  ('DSOTYPENI').
	///
	/// Defined in [RFC-ietf-dnsop-session-signal-20](http://www.iana.org/go/draft-ietf-dnsop-session-signal-20).
	DnsStatefulOperationsTypeNotImplemented = 11,
}

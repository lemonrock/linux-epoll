// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[repr(C, packed)]
struct ResourceRecordType([u8; 2]);

impl ResourceRecordType
{
	/// Internet Protocol version 4 address.
	///
	/// Defined in RFC 1035.
    pub const A: [u8; 2] = [0x00, 0x01];

	/// Name server.
	///
	/// Defined in RFC 1035.
	pub const NS: [u8; 2] = [0x00, 0x02];

	/// Canonical name.
	///
	/// Defined in RFC 1035.
	pub const CNAME: [u8; 2] = [0x00, 0x05];

	/// Marks the start of a zone of authority.
	///
	/// Defined in RFC 1035 and RFC 2308.
	pub const SOA: [u8; 2] = [0x00, 0x06];

	/// Pointer.
	///
	/// Defined in RFC 1035.
	pub const PTR: [u8; 2] = [0x00, 0x12];

	/// Mail exchange.
	///
	/// Defined in RFC 1035.
	pub const MX: [u8; 2] = [0x00, 0x15];

	/// Text string.
	///
	/// Defined in RFC 1035 and more fully specified in RFC 1464.
	pub const TXT: [u8; 2] = [0x00, 0x16];

    /// Internet Protocol version 6 address.
	///
	/// Defined in RFC 3596.
	pub const AAAA: [u8; 2] = [0x00, 0x28];

	/// Location (similar to Geographic Position, but with size and accuracy also encoded).
	///
	/// Defined in RFC 1876.
	pub const LOC: [u8; 2] = [0x00, 0x29];

	/// Server locations, port numbers and preference for a particular service, eg http.
	///
	/// Defined in RFC 2782.
	pub const SRV: [u8; 2] = [0x00, 0x33];

	/// A psuedo record type.
	///
	/// Defined in RFC 6891.
	pub const OPT: [u8; 2] = [0x00, 0x41];

	/// SSH (Secure Shell Protocol) public key fingerprint.
	///
	/// Defined in RFC 4255.
	pub const SSHFP: [u8; 2] = [0x00, 0x44];

	/// DNS-Based Authentication of Named Entities (DANE) for TLS.
	///
	/// Data represents a certificate association.
	///
	/// Defined in RFC 6698.
	pub const TLSA: [u8; 2] = [0x00, 0x52];

    /// OpenPGP public key.
    ///
    /// Defined in RFC 7929.
	pub const OPENPGPKEY: [u8; 2] = [0x00, 0x61];

    /// `*`.
    ///
    /// Returns all records of all types currently cached for a domain name from a name server; if no records are cached then the request will be forwarded on.
    ///
    /// Also known as `ANY`.
    ///
    /// Defined in RFC 1035.
	pub const Asterisk: [u8; 2] = [0x00, 0xFF];

    /// Certification Authority Authorization.
    ///
    /// Defined in RFC 6844.
	pub const CAA: [u8; 2] = [0x01, 0x01];

	#[inline(always)]
	fn is_edns_opt_pseudo_record_type(self) -> bool
	{
		&self.0 == &Self::OPT.0
	}
}

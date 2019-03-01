// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Visits different kinds of records.
pub trait ResourceRecordVisitor<'a>
{
	/// Visits a record of type `A`, which may not be aligned (this is a problem for 32-bit ARM).
	fn A(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: &Ipv4Addr) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `NS`, which may not be aligned (this is a problem for 32-bit ARM).
	fn NS(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: ParsedNameIterator<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `SOA`, which may not be aligned (this is a problem for 32-bit ARM).
	fn SOA(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: StartOfAuthority<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `CNAME`, which may not be aligned (this is a problem for 32-bit ARM).
	fn CNAME(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: ParsedNameIterator<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `PTR`, which may not be aligned (this is a problem for 32-bit ARM).
	fn PTR(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: ParsedNameIterator<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `MX`, which may not be aligned (this is a problem for 32-bit ARM).
	fn MX(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: MailExchange<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `HINFO`, which may not be aligned (this is a problem for 32-bit ARM).
	///
	/// `HINFO` had been brought back into use by RFC 8482.
	fn HINFO(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: HostInformation<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `TXT`.
	fn TXT(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: CharacterStringsIterator) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `AAAA`, which may not be aligned (this is a problem for 32-bit ARM).
	fn AAAA(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: &Ipv6Addr) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `LOC`, which may not be aligned (this is a problem for 32-bit ARM).
	fn LOC(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: &Location) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `SRV`, which may not be aligned (this is a problem for 32-bit ARM).
	fn SRV(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: Service) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `NAPTR`, which may not be aligned (this is a problem for 32-bit ARM), with a domain name.
	fn NAPTR_domain_name(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: NamingAuthorityPointerWithDomainName<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `NAPTR`, which may not be aligned (this is a problem for 32-bit ARM), with a regular expression.
	fn NAPTR_regular_expression(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: NamingAuthorityPointerWithRegularExpression<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `KX`, which may not be aligned (this is a problem for 32-bit ARM).
	fn KX(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: KeyExchange<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `CERT`.
	fn CERT(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: Certificate<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `CERT` which was ignored.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn CERT_ignored(&mut self, _name: ParsedNameIterator<'a>, _resource_record_ignored_because_reason: CertificateResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `DNAME`.
	fn DNAME(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: ParsedNameIterator<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `DS`.
	fn DS(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: DelegationSigner<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `DS` which was ignored.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn DS_ignored(&mut self, _name: ParsedNameIterator<'a>, _resource_record_ignored_because_reason: DelegationSignerResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `SSHFP`.
	fn SSHFP(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: PublicKeyFingerprint<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `SSHFP` which was ignored.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn SSHFP_ignored(&mut self, _name: ParsedNameIterator<'a>, _resource_record_ignored_because_reason: SshFingerprintResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `IPSECKEY`.
	///
	/// Note that the leading bytes of the exponent and modulus are unchecked for a RSA public key.
	fn IPSECKEY(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: IpsecPublicKey<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `IPSECKEY` which was ignored.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn IPSECKEY_ignored(&mut self, _name: ParsedNameIterator<'a>, _resource_record_ignored_because_reason: IpsecKeyResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `NSEC`.
	fn NSEC(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: NextDomain<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `RRSIG`.
	fn RRSIG(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: ResourceRecordSetSignature<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `RRSIG` which was ignored.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn RRSIG_ignored(&mut self, _name: ParsedNameIterator<'a>, _resource_record_ignored_because_reason: ResourceRecordSetSignatureResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `DNSKEY`.
	fn DNSKEY(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: DnsKey<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `DNSKEY` which was ignored.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn DNSKEY_ignored(&mut self, _name: ParsedNameIterator<'a>, _resource_record_ignored_because_reason: DnsKeyResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `DHCID`.
	fn DHCID(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: Dhcid<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `DHCID` which was ignored.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn DHCID_ignored(&mut self, _name: ParsedNameIterator<'a>, _resource_record_ignored_because_reason: DhcidResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `TLSA`.
	fn TLSA(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: DnsBasedAuthenticationOfNamedEntities<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `TLSA` which was ignored.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn TLSA_ignored(&mut self, _name: ParsedNameIterator<'a>, _resource_record_ignored_because_reason: DnsBasedAuthenticationOfNamedEntitiesResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `SMIMEA`.
	fn SMIMEA(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: DnsBasedAuthenticationOfNamedEntities<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `SMIMEA` which was ignored.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn SMIMEA_ignored(&mut self, _name: ParsedNameIterator<'a>, _resource_record_ignored_because_reason: DnsBasedAuthenticationOfNamedEntitiesResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `HIP`.
	fn HIP(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: HostIdentityProtocol<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `HIP` which was ignored.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn HIP_ignored(&mut self, _name: ParsedNameIterator<'a>, _resource_record_ignored_because_reason: HostIdentityProtocolResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `OPENPGPKEY`.
	fn OPENPGPKEY(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: OpenPgpRfc4880TransferablePublicKey<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `NID`.
	fn NID(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: NodeIdentifier) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `L32`.
	fn L32(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: Locator32) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `L64`.
	fn L64(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: Locator64) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `LP`.
	fn LP(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: LocatorPointer<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `EUI48`.
	fn EUI48(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: &'a [u8; 6]) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `EUI64`.
	fn EUI64(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: &'a [u8; 8]) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `URI`.
	fn URI(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: Uri<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `CAA`.
	fn CAA(&mut self, name: ParsedNameIterator<'a>, time_to_live: TimeToLiveInSeconds, record: CertificateAuthorityAuthorization<'a>) -> Result<(), DnsProtocolError>;

	/// Visits a record of type `CAA` which was ignored.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn CAA_ignored(&mut self, _name: ParsedNameIterator<'a>, _resource_record_ignored_because_reason: CertificationAuthorityAuthorizationResourceRecordIgnoredBecauseReason<'a>)
	{
	}

	/// Visits an unsupported record type.
	///
	/// Default implementation ignores it.
	#[inline(always)]
	fn unsupported(&mut self, _name: ParsedNameIterator<'a>, _time_to_live: TimeToLiveInSeconds, _record: &'a [u8], _parsed_labels: &mut ParsedLabels<'a>, _unsupported_resource_record_type: DataType) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits an unassigned record type.
	///
	/// Default implementation ignores it.
	#[inline(always)]
	fn unassigned(&mut self, _name: ParsedNameIterator<'a>, _time_to_live: TimeToLiveInSeconds, _record: &'a [u8], _parsed_labels: &mut ParsedLabels<'a>, _unassigned_resource_record_type: DataType) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A DNS protocol error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum DnsProtocolError
{
	/// DNS `QCLASS` is reserved (including for private use), unassigned or obsolete (ie Chaos or Hesiod).
	///
	/// Tuple contains value.
	///
	/// See [IANA](https://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#dns-parameters-2) and RFC 6895 for further details.
	ClassIsReservedUnassignedOrObsolete([u8; 2]),

	/// A text record string length was longer than that permitted by the resource data (`RDATA`) length (`RDLEN`).
	TextRecordStringLengthIncorrect,

	/// A resource record is shorter than the minimum size.
	ResourceRecordIsShorterThanMinimumSize,

	/// A resource record is shorter than the minimum size (after parsing the Name).
	ResourceRecordIsShorterThanMinimumSizeAfterParsingName,

	/// Resource data length overflows the space available.
	ResourceDataLengthOverflows,

	/// A record type was present in the answer section which should not have been (eg it was not queried for and is not `CNAME` or `DNAME`).
	ResourceRecordTypeIsNotValidInAnswerSection(DataType),

	/// A record type was present in the authority section which should not have been (only `SOA` records are allowed).
	ResourceRecordTypeIsNotValidInAuthoritySection(DataType),

	/// More than one `SOA` resource records.
	MoreThanOneStatementOfAuthorityResourceRecord,

	/// A `SOA` record type was present a section it should not have been in.
	StartOfAuthorityResourceRecordTypeIsNotPermittedInThisSection(DataType),

	/// An `OPT` record type was present a section it should not have been in.
	ExtendedDnsOptResourceRecordTypeIsNotPermittedInThisSection(DataType),

	/// A very obsolete data type was present.
	VeryObsoleteResourceRecordType(DataType),

	/// An unknown query or meta type was present; contains upper 8 bits and lower 8 bits.
	UnknownQueryTypeOrMetaType(u8, u8),

	/// A reserved record type was present; contains upper 8 bits and lower 8 bits.
	ReservedRecordType(u8, u8),

	/// Query type (`QTYPE`) `IXFR` is in a resource record.
	QueryTypeIXFRShouldNotOccurOutsideOfAQuestionSectionEntry,

	/// Query type (`QTYPE`) `AXFR` is in a resource record.
	QueryTypeAXFRShouldNotOccurOutsideOfAQuestionSectionEntry,

	/// Query type (`QTYPE`) `MAILB` is in a resource record.
	QueryTypeMAILBShouldNotOccurOutsideOfAQuestionSectionEntry,

	/// Query type (`QTYPE`) `MAILA` is in a resource record.
	QueryTypeMAILAShouldNotOccurOutsideOfAQuestionSectionEntry,

	/// Query type (`QTYPE`) `*` is in a resource record.
	QueryTypeAsteriskShouldNotOccurOutsideOfAQuestionSectionEntry,

	/// Resource data for resource record type `A` or `AAAA` has an incorrect length (value in tuple).
	ResourceDataForTypeAOrAAAAHasAnIncorrectLength(usize),

	/// Resource data for resource record type `LOC` has an incorrect length (value in tuple).
	ResourceDataForTypeLOCHasAnIncorrectLength(usize),

	/// Resource data for resource record type `LOC` has an incorrect version (value in tuple).
	ResourceDataForTypeLOCHasAnIncorrectVersion(u8),

	/// Resource data for resource record type `TLSA` or `SMIMEA` has an incorrect length (value in tuple).
	ResourceDataForTypeTLSAOrSMIMEAHasAnIncorrectLength(usize),

	/// Resource data for resource record type `TLSA` or `SMIMEA` has an incorrect digest length (value in tuple).
	ResourceDataForTypeTLSAOrSMIMEAHasADigestLengthThatIsIncorrectForTheMatchingType(usize),

	/// Resource data for resource record type `SRV` has an incorrect length (value in tuple).
	ResourceDataForTypeSRVHasAnIncorrectLength(usize),

	/// Resource data for resource record type `NAPTR` has an incorrect length (value in tuple).
	ResourceDataForTypeNAPTRHasAnIncorrectLength(usize),

	/// Resource data for resource record type `NAPTR` has data left over.
	ResourceDataForTypeNAPTRHasDataLeftOver,

	/// Resource data for resource record type `IPSECKEY` has an incorrect length (value in tuple).
	ResourceDataForTypeIPSECKEYHasTooShortALength(usize),

	/// Resource data for resource record type `IPSECKEY` has an incorrect length (value in tuple).
	ResourceDataForTypeIPSECKEYHasTooShortALengthForAnEmptyDomainNameGateway(usize),

	/// Resource data for resource record type `IPSECKEY` has an incorrect length (value in tuple).
	ResourceDataForTypeIPSECKEYHasTooShortALengthForAnInternetProtocolVersion4Gateway(usize),

	/// Resource data for resource record type `IPSECKEY` has an incorrect length (value in tuple).
	ResourceDataForTypeIPSECKEYHasTooShortALengthForAnInternetProtocolVersion6Gateway(usize),

	/// Resource data for resource record type `IPSECKEY` has an incorrect length (value in tuple).
	ResourceDataForTypeIPSECKEYHasTooShortALengthForDomainNameGateway(usize),

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for no public key (value in tuple).
	ResourceDataForTypeIPSECKEYHasWrongLengthForNoPublicKey(usize),

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a DSA public key (value in tuple).
	ResourceDataForTypeIPSECKEYOrHIPHasTooShortALengthForADSAPublicKey(usize),

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a DSA public key (value in tuple).
	ResourceDataForTypeIPSECKEYOrHIPHasWrongLengthForADSAPublicKeyOnceTIsKnown(usize),

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a RSA public key (value in tuple).
	ResourceDataForTypeIPSECKEYOrHIPHasTooShortALengthForRSAPublicKey(usize),

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a RSA public key (value in tuple).
	ResourceDataForTypeIPSECKEYOrHIPHasTooShortALengthForRSAPublicKeyForAThreeByteExponentLength(usize),

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a RSA public key exponent.
	ResourceDataForTypeIPSECKEYOrHIPHasAZeroExponentForARSAPublicKey,

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a RSA public key modulus.
	ResourceDataForTypeIPSECKEYOrHIPHasAZeroModulusForARSAPublicKey,

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a RSA public key exponent.
	ResourceDataForTypeIPSECKEYOrHIPHasTooShortALengthForARSAPublicKeyForExponentLength,

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an unusual length for a ECDSA public key (ie it does not seem to be for `P-256` or `P-384`).
	ResourceDataForTypeIPSECKEYOrHIPHasAUnusualLengthForAnECDSAPublicKey(usize),

	/// Resource data for resource record type `NAPTR` has both a regular expression and a domain name.
	ResourceDataForTypeNAPTRHasBothARegularExpressionAndADomainName,

	/// Resource data for resource record type `DS` has an incorrect length (value in tuple).
	ResourceDataForTypeDSHasAnIncorrectLength(usize),

	/// Resource data for resource record type `DHCID` has an incorrect length (value in tuple).
	ResourceDataForTypeDHCIDHasAnIncorrectLength(usize),

	/// Resource data for resource record type `DHCID` has reserved identifier type code.
	ResourceDataForTypeDHCIDHasAReservedIdentifierTypeCode,

	/// Resource data for resource record type `DHCID` has reserved digest type code.
	ResourceDataForTypeDHCIDHasAReservedDigestTypeCode,

	/// Resource data for resource record type `DHCID` has an incorrect digest length (value in tuple).
	ResourceDataForTypeDHCIDHasADigestLengthThatIsIncorrectForTheMatchingType(usize),

	/// Resource data for resource record type `SSHFP` has an incorrect length (value in tuple).
	ResourceDataForTypeSSHFPHasAnIncorrectLength(usize),

	/// Resource data for resource record type `SSHFP` has a reserved public key algorithm.
	ResourceDataForTypeSSHFPHasAReservedPublicKeyAlgorithm(u8),

	/// Resource data for resource record type `SSHFP` has a reserved digest algorithm.
	ResourceDataForTypeSSHFPHasAReservedDigestAlgorithm(u8),

	/// Resource data for resource record type `SSHFP` has an incorrect digest length (value in tuple).
	ResourceDataForTypeSSHFPAHasADigestLengthThatIsIncorrectForTheMatchingType(usize),

	/// Resource data for resource record type `SSHFP` has a digest size which is incorrect for the fingerprint type.
	///
	/// Tuple contains the fingerprint type and the actual digest size.
	ResourceDataForTypeSSHFPHasADigestOfIncorrectSizeForTheFingerprintType(FingerprintType, usize),

	/// Resource data for resource record type `NID` has an incorrect length (value in tuple).
	ResourceDataForTypeNIDHasAnIncorrectLength(usize),

	/// Resource data for resource record type `L32` has an incorrect length (value in tuple).
	ResourceDataForTypeL32HasAnIncorrectLength(usize),

	/// Resource data for resource record type `L64` has an incorrect length (value in tuple).
	ResourceDataForTypeL64HasAnIncorrectLength(usize),

	/// Resource data for resource record type `LP` has too short a length (value in tuple).
	ResourceDataForTypeLPHasTooShortALength(usize),

	/// Resource data for resource record type `LP` has data left over after parsing the domain name.
	ResourceDataForTypeLPHasDataLeftOver(usize),

	/// Resource data for resource record type `EUI48` has an incorrect length (value in tuple).
	ResourceDataForTypeEUI48HasAnIncorrectLength(usize),

	/// Resource data for resource record type `EUI64` has an incorrect length (value in tuple).
	ResourceDataForTypeEUI64HasAnIncorrectLength(usize),

	/// Resource data for resource record type `URI` has an incorrect length (value in tuple).
	ResourceDataForTypeURIHasAnIncorrectLength(usize),

	/// Resource data for resource record type `CAA` has an incorrect length (value in tuple).
	ResourceDataForTypeCAAHasAnIncorrectLength(usize),

	/// Resource data for resource record type `CAA` has a zero tag length.
	ResourceDataForTypeCAAHasAZeroTagLength,

	/// Resource data for resource record type `CERT` has too short a length (value in tuple).
	ResourceDataForTypeCERTHasTooShortALength(usize),

	/// Resource data for resource record type `CERT` uses a reserved certificate type value (value in tuple).
	ResourceDataForTypeCERTUsesAReservedCertificateTypeValue(u16),

	/// Resource data for resource record type `CERT` uses an experimental certificate type value (value in tuple).
	ResourceDataForTypeCERTUsesAnExperimentalCertificateTypeValue(u16),

	/// Resource data for resource record type `DNSKEY` has an incorrect length (value in tuple).
	ResourceDataForTypeDNSKEYHasAnIncorrectLength(usize),

	/// Resource data for resource record type `NSEC` has an incorrect length (value in tuple).
	ResourceDataForTypeNSECHasAnIncorrectLength(usize),

	/// Resource data for resource record type `CSYNC` or `NSEC` has an incorrect length (value in tuple).
	ResourceDataForTypeCSYNCOrNSECHasAnOverflowingBlockLength(usize),

	/// Resource data for resource record type `CSYNC` or `NSEC` has a repeated or decreasing window number.
	ResourceDataForTypeCSYNCOrNSECHasARepeatedOrDecreasingWindowNumber,

	/// Resource data for resource record type `CSYNC` or `NSEC` has a zero bitmap length (value in tuple).
	ResourceDataForTypeCSYNCONSECHasAZeroBitmapLength,

	/// Resource data for resource record type `CSYNC` or `NSEC` has an incorrect bitmap length (value in tuple).
	ResourceDataForTypeCSYNCOrNSECHasAnIncorrectBitmapLength(usize),

	/// Resource data for resource record type `CSYNC` or `NSEC` has an incorrect bitmap length (value in tuple).
	ResourceDataForTypeCSYNCOrNSECHasAnOverflowingBitmapLength(usize),

	/// Resource data for resource record type `NSEC3` has an incorrect length (value in tuple).
	ResourceDataForTypeNSEC3HasAnIncorrectLength(usize),

	/// Resource data for resource record type `NSEC3` has a reserved hash algorithm.
	ResourceDataForTypeNSEC3HasAReservedHashAlgorithm(usize),

	/// Resource data for resource record type `NSEC3` has an incorrect hash length for a SHA-1 hash.
	ResourceDataForTypeNSEC3HasAnIncorrectHashLengthForASha1Hash(usize),

	/// Resource data for resource record type `NSEC3` has an overflowing salt length.
	ResourceDataForTypeNSEC3HasAnOverflowingSaltLength(usize),

	/// Resource data for resource record type `NSEC3` has an overflowing hash length.
	ResourceDataForTypeNSEC3HasAnOverflowingHashLength(usize),

	/// Resource data for resource record type `RRSIG` has an incorrect length (value in tuple).
	ResourceDataForTypeRRSIGHasAnIncorrectLength(usize),

	/// Resource data for resource record type `RRSIG` has more than 126 labels (including root, only 127 labels are allowed and root is not allowed to be counted in this instance).
	ResourceDataForTypeRRSIGHasMoreThan126Labels(u8),

	/// Resource data for resource record type `HIP` has an incorrect length (value in tuple).
	ResourceDataForTypeHIPHasAnIncorrectLength(usize),

	/// Resource data for resource record type `HINFO` has too short a length (value in tuple).
	ResourceDataForTypeHINFOHasTooShortALength(usize),

	/// Resource data for resource record type `HINFO` has too short a length after checking length of CPU field (value in tuple).
	ResourceDataForTypeHINFOWouldHaveCpuDataOverflow(usize),

	/// Resource data for resource record type `HINFO` has too short a length after checking length of OS field (value in tuple).
	ResourceDataForTypeHINFOWouldHaveOsDataOverflow(usize),

	/// After parsing resource data in a record of type `HINFO`, there is unattributed data remaining.
	ResourceDataForTypeHINFOWouldHaveUnusuedDataRemaining,

	/// Resource data for resource record type `MX` has too short a length (value in tuple).
	ResourceDataForTypeMXHasTooShortALength(usize),

	/// Resource data for resource record type `TXT` has not text strings (and thus has a length of zero).
	ResourceRecordForTypeTXTHasNoCharacterStrings,

	/// After parsing resource data in a record of type `TXT`, there is unattributed data remaining.
	ResourceDataForTypeTXTWouldHaveUnusuedDataRemaining,

	/// The security alogrithm DS-Delete should not be used for this resource record.
	SecurityAlgorithmShouldNotBeUsedForThisResourceRecordType(u8),

	/// The security alogrithm type is reserved (number in tuple).
	SecurityAlgorithmTypeIsReservedByRfc6725(u8),

	/// A reserved security algorithm type (number in tuple).
	SecurityAlgorithmTypeIsReservedByRfc6014(u8),

	/// Reserved.
	SecurityAlgorithmTypeIsReservedByRfc4034,

	/// Reserved.
	DigestAlgorithmTypeIsReservedByRfc3658,

	/// A `DS` resource record has digest data that has an incorrect length for the digest type.
	ResourceDataForTypeDSHasADigestLengthThatIsIncorrectForTheDigestType(usize),

	/// Resource data for resource record type `SOA` has an invalid length after parsing `MNAME` and `RNAME`.
	StartOfAuthorityIsIncorrectSizeAfterParsingMNAMEAndRNAME,

	/// A resource record of the psuedo-type `OPT` is present other than in the additional record section.
	ExtendedDnsOptRecordOutsideOfAdditionalDataSection,

	/// More than one resource record of the psuedo-type `OPT` is present in the additional record section.
	MoreThanOneExtendedDnsOptResourceRecord,

	/// The UDP payload size is less than 512 bytes (actual value in tuple).
	ExtendedDnsOptRecordUdpPayloadSizeIsLessThan512Bytes(u16),

	/// A resource record of the psuedo-type `OPT` is present with a name other than ''.
	ExtendedDnsOptRecordNameTooLong,

	/// A resource record of the psuedo-type `OPT` is present with a name other than ''.
	ExtendedDnsOptRecordNameNotRoot,

	/// An unsupported EDNS version; unsupported version in tuple.
	UnsupportedExtendedDnsVersion(u8),

	/// EDNS(0) `Z`field not zero.
	ExtendedDnsZFieldNotZero,

	/// EDNS(0) Option field has a length less than 4.
	ExtendedDnsOptionTooShort,

	/// EDNS(0) Option code was in the reserved set (0, 65001-65534 and 65535); code is in tuple.
	///
	/// Code 4 is ignored as the draft it pertains sees occasionaly progress; it might come into being.
	ExtendedDnsOptionCodeWasReserved(u16),

	/// EDNS(0) Option length field indicates an option data field whose length would exceed that remaining in the resource data of the `OPT` resource record.
	ExtendedDnsOptionDataOverflows,

	/// EDNS(0) Option `DAU` must only be set in a request.
	ExtendedDnsOptionDAUMustOnlyBeSetInARequest,

	/// EDNS(0) Option `DHU` must only be set in a request.
	ExtendedDnsOptionDHUMustOnlyBeSetInARequest,

	/// EDNS(0) Option `N3U` must only be set in a request.
	ExtendedDnsOptionN3UMustOnlyBeSetInARequest,

	/// The name was not long enough.
	///
	/// Typically this occurs when a name is shorter than the `RLEN/RDATA` space allocated for it in, say, a `CNAME` resource record.
	NameWasNotLongEnough,

	/// The name occupies no bytes at all.
	NameIsEmpty,

	/// The extended name labels are unused.
	ExtendedNameLabelsAreUnused,

	/// The unallocated name labels are unused.
	UnallocatedNameLabelsAreUnused,

	/// Compressed name labels are disallowed in this resource record.
	///
	/// See RFC 3597, Section 4 for some confusing rules.
	CompressedNameLabelsAreDisallowedInThisResourceRecord,

	/// The label pointer offset does not point to a previously parsed label.
	///
	/// Note that this includes pointers to pointers.
	///
	/// The tuple contains the offset.
	InvalidLabelPointerOffset(usize),

	/// There is not a terminal root label in a Name.
	NoTerminalRootLabel,

	/// A label length would cause overflow (ie it is too long).
	LabelLengthOverflows,

	/// A label pointer overflows (ie there isn't another byte for bottom 8 bits).
	LabelPointerOverflows,

	/// A label pointer is beyond the current location.
	LabelPointerOffsetPointsForwardToUnparsedData,
}

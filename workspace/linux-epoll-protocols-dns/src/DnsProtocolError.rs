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

	/// An obsolete data type was present.
	ObsoleteResourceRecordType(DataType),

	/// An unknown query or meta type was present.
	UnknownQueryTypeOrMetaType(DataType),

	/// A reserved record type was present.
	ReservedRecordType(DataType),

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

	/// Resource data for resource record type `TLSA` has an incorrect length (value in tuple).
	ResourceDataForTypeTLSAHasAnIncorrectLength(usize),

	/// Resource data for resource record type `SRV` has an incorrect length (value in tuple).
	ResourceDataForTypeSRVHasAnIncorrectLength(usize),

	/// Resource data for resource record type `NAPTR` has an incorrect length (value in tuple).
	ResourceDataForTypeNAPTRHasAnIncorrectLength(usize),

	/// Resource data for resource record type `NAPTR` has data left over.
	ResourceDataForTypeNAPTRHasDataLeftOver,

	/// Resource data for resource record type `NAPTR` has both a regular expression and a domain name.
	ResourceDataForTypeNAPTRHasBothARegularExpressionAndADomainName,

	/// Resource data for resource record type `SSHFP` has an incorrect length (value in tuple).
	ResourceDataForTypeSSHFPHasAnIncorrectLength(usize),

	/// Resource data for resource record type `SSHFP` has an unrecognised public key algorithm (value in tuple).
	ResourceDataForTypeSSHFPHasAnUnrecognisedPublicKeyAlgorithm(u8),

	/// Resource data for resource record type `SSHFP` has an unrecognised fingerprint type (value in tuple).
	ResourceDataForTypeSSHFPHasAnUnrecognisedFingerprintType(u8),

	/// Resource data for resource record type `SSHFP` has a digest size which is incorrect for the fingerprint type.
	///
	/// Tuple contains the fingerprint type and the actual digest size.
	ResourceDataForTypeSSHFPHasADigestOfIncorrectSizeForTheFingerprintType(FingerprintType, usize),

	/// Resource data for resource record type `TLSA` has an unrecognised certificate usage (value in tuple).
	ResourceDataForTypeTLSAHasAnUnrecognisedCertificateUsage(u8),

	/// Resource data for resource record type `TLSA` has an unrecognised selector (value in tuple).
	ResourceDataForTypeTLSAHasAnUnrecognisedSelector(u8),

	/// Resource data for resource record type `TLSA` has an unrecognised matching type (value in tuple).
	ResourceDataForTypeTLSAHasAnUnrecognisedMatchingType(u8),

	/// Resource data for resource record type `TLSA` has an unrecognised matching type (value in tuple).
	ResourceDataForTypeTLSAHasADigestLengthThatIsIncorrectForTheMatchingType(MatchingType, usize),

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

	/// Resource data for resource record type `SOA` has an invalid length after parsing `MNAME` and `RNAME`.
	StartOfAuthorityIsIncorrectSizeAfterParsingMNAMEAndRNAME,

	/// A resource record of the psuedo-type `OPT` is present other than in the additional record section.
	ExtendedDnsOptRecordOutsideOfAdditionalDataSection,

	/// More than one resource record of the psuedo-type `OPT` is present in the additional record section.
	MoreThanOneExtendedDnsOptRecord,

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

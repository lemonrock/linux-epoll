// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


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

	/// Resource record type is asterisk in a resource record.
	ResourceRecordTypeAsteriskShouldNotOccurOutsideOfAQuestionSectionEntry,

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

	/// Resource data for resource record type `MX` has too short a length (value in tuple).
	ResourceDataForTypeMXHasTooShortALength(usize),

	/// Resource data for resource record type `TXT` has not text strings (and thus has a length of zero).
	ResourceRecordForTypeTXTHasNoTextStrings,

	/// Resource data for resource record type `SOA` has an invalid length after parsing `MNAME` and `RNAME`.
	StartOfAuthorityIsIncorrectSizeAfterParsingMNAMEAndRNAME,

	/// A resource record of the psuedo-type `OPT` is present other than in the additional record section.
	EdnsOptRecordOutsideOfAdditionalDataSection,

	/// A resource record of the psuedo-type `OPT` is present with a name other than ''.
	EdnsOptRecordNameTooLong,

	/// A resource record of the psuedo-type `OPT` is present with a name other than ''.
	EdnsOptRecordNameNotRoot,

	/// An unsupported EDNS version; unsupported version in tuple.
	UnsupportedExtendedDnsVersion(u8),

	/// EDNS `Z`field not zero.
	ExtendedDnsZFieldNotZero,

	/// More than one resource record of the psuedo-type `OPT` is present in the additional record section.
	MoreThanOneEdnsOptRecord,

	/// The name was not long enough.
	///
	/// Typically this occurs when a name is shorter than the `RLEN/RDATA` space allocated for it in, say, a `CNAME` resource record.
	NameWasNotLongEnough,

	/// The name occupies no bytes at all.
	NameIsEmpty,

	/// The unsupported name label (2 bits, ie `u2`).
	UnsupportedNameLabelKind(u8),

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

	/// A label pointer is beyond the current location.
	LabelPointerOffsetPointsForwardToUnparsedData,
}

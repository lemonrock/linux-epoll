// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Why was a security algorithm rejected ignored?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SecurityAlgorithmRejectedBecauseReason
{
	/// The security alogrithm `RSA-MD5` is deprecated.
	DeprecatedSecurityAlgorithm_RSA_MD5,

	/// DH came about well before version 3 of DNSSEC and is not used.
	EffectivelyObsoleteSecurityAlgorithm_Diffie_Hellman,

	/// DSA is probably vulnerable.
	ProbablyVulnerableSecurityAlgorithm_DSA,

	/// SHA-1 is broken; RSA-SHA-1 should be considered to be broken.
	MayBeBrokenSecurityAlgorithm_RSA_SHA_1,

	/// The security alogrithm `GOST R 34.10-2001` is effectively obsolete, as RFC 7091 permitted only a 5-year window for its replacement by the newer `GOST R 34.10-2012`.
	///
	/// ## Details of the `GOST R 34.10-2001` algorithm
	///
	/// Defined in RFC 5933.
	///
	/// An elliptic curve public key using the `GOST R 34.10-2001` standard; the underlying cryptographic implementation was defined in RFC 5832 which was updated by RFC 7091 which defines the newer `GOST R 34.10-2012` algorithm.
	/// It was created by the Russian state.
	/// A C++ implementation of the later standard `GOST R 34.10-2012` is at <https://github.com/nevkontakte/GOST-R-34.10-2012>.
	///
	/// The RFCs defining the alogrithm variants are literal translations of Russian (mandatory) national standards and contain language unusual for RFCs.
	EffectivelyObsoleteSecurityAlgorithm_GOST_R_34_10_2001,

	/// Unassigned.
	Unassigned(u8),

	/// `INDIRECT`.
	ReservedByRfc4034ForIndirectKeys,

	/// `PRIVATE`.
	ReservedByRfc4034ForPrivateAlgorithms,

	/// `PRIVATEOID`.
	ReservedByRfc4034ForPrivateAlgorithmsOids,
}

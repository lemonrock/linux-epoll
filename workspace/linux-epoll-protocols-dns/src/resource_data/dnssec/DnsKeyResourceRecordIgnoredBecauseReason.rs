// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Why was a `DNSKEY` record ignored?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DnsKeyResourceRecordIgnoredBecauseReason
{
	/// The protocol was not `3`.
	ProtocolWasNot3(u8),

	/// Unassigned flags.
	UnassignedFlags(u16),

	/// Revoked.
	Revoked,

	/// The Secure Entry Point (SEP) flag is set but the zone key flag is not.
	///
	/// RFC 4034, Section 2.1.1, Paragraph 2 describes this combination: "A DNSKEY RR with the SEP set and the Zone Key flag not set MUST NOT be used to verify RRSIGs that cover RRsets".
	SecureEntryPointFlagSetButNotZoneKeyFlag,

	/// Security algorithm was rejected.
	SecurityAlgorithmRejected(SecurityAlgorithmRejectedBecauseReason),
}

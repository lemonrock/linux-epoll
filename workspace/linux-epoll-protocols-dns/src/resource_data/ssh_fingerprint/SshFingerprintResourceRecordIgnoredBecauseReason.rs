// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Why was a `SSHFP` record ignored?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SshFingerprintResourceRecordIgnoredBecauseReason
{
	/// The public key algorithm DSA is effectively obsolete.
	PublicKeyAlgorithmDsaIsEffectivelyObsolete,

	/// The public key algorithm was unassigned.
	PublicKeyAlgorithmUnassigned(u8),

	/// The digest algorithm SHA-1 is broken.
	DigestAlgorithmSha1IsBroken,

	/// The digest algorithm was unassigned.
	DigestAlgorithmUnassigned(u8),
}

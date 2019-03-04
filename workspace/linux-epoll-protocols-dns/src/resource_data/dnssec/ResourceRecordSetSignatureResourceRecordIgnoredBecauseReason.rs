// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Why was a `RRSIG` record ignored?
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Hash)]
pub enum ResourceRecordSetSignatureResourceRecordIgnoredBecauseReason
{
	/// Security algorithm was rejected.
	SecurityAlgorithmRejected(SecurityAlgorithmRejectedBecauseReason),

	/// Difference in signature expiration and inception is too great for wrapping serial number mathematics.
	///
	/// This is the fault of a badly configured server.
	DifferenceInSignatureExpirationAndInceptionIsTooGreatForWrappingSerialNumberMathematics
	{
		/// Signature inception timestamp.
		signature_inception_timestamp: SignatureTimestamp,

		/// Signature expiration timestamp.
		signature_expiration_timestamp: SignatureTimestamp,
	},

	/// The difference between the signature inception and expiration was negative (ie the signature expired before its inception) or zero (it expired at inception).
	///
	/// This is the fault of a badly configured server.
	DifferenceInSignatureInceptionAndExpirationWasNegativeOrZero
	{
		/// Signature inception timestamp.
		signature_inception_timestamp: SignatureTimestamp,

		/// Signature expiration timestamp.
		signature_expiration_timestamp: SignatureTimestamp,
	},

	/// The signature is not yet valid.
	InceptionIsInTheFuture
	{
		/// Signature inception timestamp.
		signature_inception_timestamp: SignatureTimestamp,

		/// Signature expiration timestamp.
		signature_expiration_timestamp: SignatureTimestamp,
	},

	/// The signature has expired.
	Expired
	{
		/// Signature inception timestamp.
		signature_inception_timestamp: SignatureTimestamp,

		/// Signature expiration timestamp.
		signature_expiration_timestamp: SignatureTimestamp,
	},
}

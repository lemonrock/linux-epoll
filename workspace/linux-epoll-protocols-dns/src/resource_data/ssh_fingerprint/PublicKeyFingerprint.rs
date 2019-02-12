// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A SSH public key fingerprint.
#[derive(Debug)]
pub struct PublicKeyFingerprint<'a>
{
	/// Public key algorithm.
	pub public_key_algorithm: PublicKeyAlgorithm,

	/// Digest algorithm used to produced disgest of public key.
	pub digest_algorithm: FingerprintType,

	/// Digest of public key.
	///
	/// Length has already been checked to make sure it is correct for the `digest_algorithm`.
	pub digest_bytes: &'a [u8],
}

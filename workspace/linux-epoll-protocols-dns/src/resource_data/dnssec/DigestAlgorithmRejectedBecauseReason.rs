// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Why was a digest algorithm rejected ignored?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DigestAlgorithmRejectedBecauseReason
{
	/// SHA-1, although mandatory, is broken.
	Sha1IsBroken,

	/// GOST R 34.11-94 may been broken; it has been replaced in Russian standards with more a modern algorithm
	Gost94MayBeBroken,

	/// It is impossible to use unassigned values.
	Unassigned(u8),
}

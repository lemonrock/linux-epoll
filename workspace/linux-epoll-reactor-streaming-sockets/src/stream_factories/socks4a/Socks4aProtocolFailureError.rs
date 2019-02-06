// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// SOCKS4a protocol failure error.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Socks4aProtocolFailureError
{
	/// Version was not `4` (actual value in tuple).
	VersionInvalid(u8),

	/// Command code was `91`.
	RequestRejectedOrFailed,

	/// Command code was `92`.
	RequestRejectedBecauseSocksServerCanNotConnectToIdentdOnTheClient,

	/// Command code was `93`.
	RequestRejectedBecauseTheClientProgramAndIdentdReportDifferentUserIdentifiers,

	/// Command code was invalid (actual value in tuple).
	CommandCodeWasInvalid(u8),
}

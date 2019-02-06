// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// SOCKS5 protocol failure error.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Socks5ProtocolFailureError
{
	/// Version was not `5` (actual value in tuple).
	VersionInvalid(u8),

	/// No acceptable authentication methods were supplied by the client.
	NoAcceptableAuthenticationMethodsSupplied,

	/// Credential code which was never sent by client (actual value in tuple).
	CredentialCodeInReplyWasNeverSentByClient(Socks5AuthenticationCredentialCode),

	/// User name was empty.
	EmptyUserName,

	/// Password was empty.
	EmptyPassword,

	/// Version was not `1` (actual value in tuple).
	UserNamePasswordVersionInvalid(u8),

	/// Failed; non-zero status code from the server is in the tuple.
	UserNamePasswordAuthenticationFailed(u8),
}

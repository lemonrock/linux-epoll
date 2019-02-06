// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// An Internet Protocol version 4 address (for SOCKS4) or host name to be resolved by the SOCKS proxy server (SOCKS4a).
///
/// Note that a Host Name should never exceed 253 bytes (254 including trailing ASCII NUL).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InternetProtocolVersion4AddressOrHostName
{
	/// Internet Protocol version 4 address.
	InternetProtocolVersion4Address(IpAddrV4),

	/// Ask SOCKS to resolve the host name to an Internet Protocol version 4 address for us.
	///
	/// Note that a Host Name should never exceed 253 bytes (254 including trailing ASCII NUL).
	HostName(CString),
}

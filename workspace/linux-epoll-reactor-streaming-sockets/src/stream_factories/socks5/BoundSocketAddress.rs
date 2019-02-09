// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A SOCKS5 bound socket address.
///
/// Conceptually similar to a `SocketAddr`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoundSocketAddress
{
	/// An address.
	///
	/// Normally an Internet Protocol Version 4 or Version 6 address, but the SOCKS5 protocol permits DNS domain names and host names.
	pub address: InternetProtocolAddressOrHostName,

	/// A port number.
	pub port: u16,
}

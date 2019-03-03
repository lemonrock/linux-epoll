// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A gateway associated with an IPsec public key.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Gateway<'a>
{
	/// As an Internet Protocol version 6 address.
	InternetProtocolVersion4(&'a Ipv4Addr),

	/// As an Internet Protocol version 6 address.
	InternetProtocolVersion6(&'a Ipv6Addr),

	/// As a domain name.
	DomainName(WithoutCompressionParsedNameIterator<'a>)
}

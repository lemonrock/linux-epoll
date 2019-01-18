// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


trait IpLookupTableMatch
{
	fn is_match<A>(&self, address: A) -> bool;
}

impl IpLookupTableMatch for IpLookupTable
{
	#[inline(always)]
	fn is_match<A>(&self, address: A) -> bool
	{
		self.longest_match(remote_peer_address).is_some()
	}
}

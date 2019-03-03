// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Key exchange data.
#[derive(Debug)]
pub struct KeyExchange<'a>
{
	/// Preference.
	pub preference: u16,

	/// Key exchange server name.
	pub key_exchange_server_name: WithoutCompressionParsedNameIterator<'a>,
}

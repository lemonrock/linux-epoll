// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// `A` is either an Internet Protocol Version 4 address (`Ipv4Addr`) or an Internet Protocol Version 6 address (`Ipv6Addr`).
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PermittedInternetProtocolSubnets<A>(HashMap<A, u8>);

impl<A> Deref for PermittedInternetProtocolSubnets<A>
{
	type Target = HashMap<A, u8>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<A> DerefMut for PermittedInternetProtocolSubnets<A>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl<A> PermittedInternetProtocolSubnets<A>
{
	#[inline(always)]
	pub(crate) fn to_ip_lookup_table(self) -> IpLookupTable<A, ()>
	{
		let mut internet_protocol_address_access_control_list = IpLookupTable::with_capacity(self.0.len());
		for (address, mask) in self.0.drain()
		{
			internet_protocol_address_access_control_list.insert(address, mask as u32, ());
		}
		internet_protocol_address_access_control_list
	}
}

impl PermittedInternetProtocolSubnets<Ipv4Addr>
{
	/// * `address`: An Internet Protocol version 4 address.
	/// * `mask`: A bit mask from 0 to 32 for Internet Protocol Version 4 addresses and from 0 to 128 for Internet Protocol Version 6 addresses.
	pub fn add_subnet(&mut self, address: Ipv4Addr, mask: u8)
	{
		debug_assert!(mask <= 32, "mask is greater than 32");

		self.insert(address, mask);
	}
}

impl PermittedInternetProtocolSubnets<Ipv6Addr>
{
	/// * `address`: An Internet Protocol version 6 address.
	/// * `mask`: A bit mask from 0 to 32 for Internet Protocol Version 4 addresses and from 0 to 128 for Internet Protocol Version 6 addresses.
	pub fn add_subnet(&mut self, address: Ipv6Addr, mask: u8)
	{
		debug_assert!(mask <= 128, "mask is greater than 128");

		self.insert(address, mask);
	}
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Remote peer address-based access control.
///
/// Holds deny and permitted address lists (black lists and white lists) for remote Internet Protocol version 4 and version 6 subnets; the deny list is checked first, and, if the address is not present, the allow list then checked.
/// This allows for generic white listing rules (eg all of the regular internet) and then for explicit exemptions (eg these message-dispatchs in this country).
///
/// For unix domain sockets, there is a deny list of user identifiers and a permitted list of (primary) group identifiers.
/// The deny list is checked first, and, if the user identifier is not present, the allow list is then checked.
/// This allows for generic white listing rules (eg for all administrators) and then for explicit exemptions (eg a recently departed administrator).
pub struct RemotePeerAddressBasedAccessControl
{
	denied_protocol_version_4_subnets: IpLookupTable<Ipv4Addr, ()>,
	permitted_protocol_version_4_subnets: Option<IpLookupTable<Ipv4Addr, ()>>,
	denied_protocol_version_6_subnets: IpLookupTable<Ipv6Addr, ()>,
	permitted_protocol_version_6_subnets: Option<IpLookupTable<Ipv6Addr, ()>>,
	denied_unix_domain_user_identifierentifiers: HashSet<uid_t>,
	permitted_unix_domain_group_identifiers: Option<HashSet<gid_t>>,
}

impl Debug for RemotePeerAddressBasedAccessControl
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "RemotePeerAddressBasedAccessControl {{ denied_protocol_version_4_subnets: _, permitted_protocol_version_4_subnets: _, denied_protocol_version_6_subnets: _, permitted_protocol_version_6_subnets: _, denied_unix_domain_user_identifierentifiers: {:?}, permitted_unix_domain_group_identifiers: {:?} }}", self.denied_unix_domain_user_identifierentifiers, self.permitted_unix_domain_group_identifiers)
	}
}

impl RemotePeerAddressBasedAccessControl
{
	/// Creates a new instance.
	///
	/// Permitted lists are `Option`s.
	/// If they are `None`, then the permitted list is not checked and all possible values are permitted (as long as the accompanying deny list does not deny them).
	#[inline(always)]
	pub fn new(denied_protocol_version_4_subnets: InternetProtocolSubnets<Ipv4Addr>, permitted_protocol_version_4_subnets: Option<InternetProtocolSubnets<Ipv4Addr>>, denied_protocol_version_6_subnets: InternetProtocolSubnets<Ipv6Addr>, permitted_protocol_version_6_subnets: Option<InternetProtocolSubnets<Ipv6Addr>>, denied_unix_domain_user_identifierentifiers: HashSet<uid_t>, permitted_unix_domain_group_identifiers: Option<HashSet<gid_t>>) -> Self
	{
		Self
		{
			denied_protocol_version_4_subnets: denied_protocol_version_4_subnets.to_ip_lookup_table(),
			permitted_protocol_version_4_subnets: permitted_protocol_version_4_subnets.map(|value| value.to_ip_lookup_table()),
			denied_protocol_version_6_subnets: denied_protocol_version_6_subnets.to_ip_lookup_table(),
			permitted_protocol_version_6_subnets: permitted_protocol_version_6_subnets.map(|value| value.to_ip_lookup_table()),
			denied_unix_domain_user_identifierentifiers,
			permitted_unix_domain_group_identifiers,
		}
	}
}

impl AccessControl<sockaddr_in> for RemotePeerAddressBasedAccessControl
{
	#[inline(always)]
	fn is_remote_peer_allowed(&self, remote_peer_address: sockaddr_in, _streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<sockaddr_in>) -> bool
	{
		let remote_peer_address: Ipv4Addr = unsafe { transmute(remote_peer_address.sin_addr) };

		if unlikely!(self.denied_protocol_version_4_subnets.is_match(remote_peer_address))
		{
			return false
		}

		match self.permitted_protocol_version_4_subnets
		{
			None => true,
			Some(ref ip_lookup_table) => ip_lookup_table.is_match(remote_peer_address)
		}
	}
}

impl AccessControl<sockaddr_in6> for RemotePeerAddressBasedAccessControl
{
	#[inline(always)]
	fn is_remote_peer_allowed(&self, remote_peer_address: sockaddr_in6, _streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<sockaddr_in6>) -> bool
	{
		let remote_peer_address: Ipv6Addr = unsafe { transmute(remote_peer_address.sin6_addr) };

		if unlikely!(self.denied_protocol_version_6_subnets.is_match(remote_peer_address))
		{
			return false
		}

		match self.permitted_protocol_version_6_subnets
		{
			None => true,
			Some(ref ip_lookup_table) => ip_lookup_table.is_match(remote_peer_address)
		}
	}
}

impl AccessControl<sockaddr_un> for RemotePeerAddressBasedAccessControl
{
	#[inline(always)]
	fn is_remote_peer_allowed(&self, _remote_peer_address: sockaddr_un, streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<sockaddr_un>) -> bool
	{
		let credentials = streaming_socket_file_descriptor.remote_peer_credentials();

		if unlikely!(self.denied_unix_domain_user_identifierentifiers.contains(&credentials.user_identifierentifier))
		{
			return false
		}

		match self.permitted_unix_domain_group_identifiers
		{
			None => true,
			Some(ref group_identifiers) => group_identifiers.contains(&credentials.group_identifier),
		}
	}
}

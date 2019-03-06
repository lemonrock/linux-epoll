// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Host Identity Protocol (`HIP`) resource record data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HostIdentityProtocol<'a>
{
	/// Host identitiy tag (HIT).
	pub host_identity_tag: &'a [u8],

	/// Public key.
	pub public_key: Option<PublicKey<'a>>,

	/// At least one rendezvous server is present.
	pub first_rendezvous_server_domain_name: WithoutCompressionParsedName<'a>,

	/// May be empty.
	///
	/// Recipients *SHOULD* parse this to make sure the names are valid.
	pub remaining_rendezvous_server_domain_names: &'a [u8],
}

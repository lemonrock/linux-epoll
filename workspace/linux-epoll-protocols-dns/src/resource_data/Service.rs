// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A `SRV` record.
#[derive(Debug)]
pub struct Service<'a>
{
	/// Priority.
	///
	/// RFC 2782: "A client MUST attempt to contact the target host with the lowest-numbered priority it can reach; target hosts with the same priority SHOULD be tried in an order defined by the weight field".
	pub priority: u16,

	/// Weight.
	///
	/// Indicative of load on the server at a point in time, or, more crudely, relative performance of different servers.
	///
	/// RFC 2782: "Larger weights SHOULD be given a proportionately higher probability of being selected".
	///
	/// Larger weights imply less loading.
	pub weight: u16,

	/// TCP, UDP or SCTP port for the service..
	pub port: u16,

	/// Must not be an alias; should not use name compression; a value of '.' (ie Root) means the service is unavailable.
	///
	/// The interaction with round-robin `A` or `AAAA` records is unclear.
	pub target: WithCompressionParsedNameIterator<'a>,
}

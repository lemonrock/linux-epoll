// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Edge-triggered status.
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReactEdgeTriggeredStatus
{
	/// Input (read) or output (write) is now available.
	InputOrOutputNowAvailable
	{
		/// Read is now ready.
		read_now_ready: bool,

		/// Write is now ready.
		write_now_ready: bool,
	},

	/// The connection is closing with an error.
	ClosedWithError,

	/// The remote peer closed cleanly.
	///
	/// This includes TCP 'half-close' shutdowns (which are near useless on modern socket protocols that use TLS).
	RemotePeerClosedCleanly,
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Settings to apply to a connection established by `accept()`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StreamingServerListenerSocketSettings
{
	/// Send buffer size (in bytes).
	pub send_buffer_size_in_bytes: usize,

	/// Receive buffer size (in bytes).
	pub receive_buffer_size_in_bytes: usize,

	/// Idle for how many seconds before TCP keep-alive is started?
	pub idles_before_keep_alive_seconds: u16,

	/// Interval in seconds between TCP keep-alive probes.
	pub keep_alive_interval_seconds: u16,

	/// Maximum number of TCP keep-alive probes to send.
	pub maximum_keep_alive_probes: u16,

	/// How long to linger (in seconds).
	pub linger_seconds: u16,

	/// How long to linger in the TCP FIN-WAIT2 state (in seconds).
	pub linger_in_FIN_WAIT2_seconds: u16,

	/// Zero is rounded up to one.
	///
	/// How many TCP SYN segments to try to transmit before giving up on a connection attempt?
	pub maximum_SYN_transmits: u16,

	/// Back log of prior connections to honour (typically capped by Linux to 128).
	pub back_log: u32,
}

impl Default for StreamingServerListenerSocketSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		StreamingServerListenerSocketSettings
		{
			send_buffer_size_in_bytes: 64 * 1024,
			receive_buffer_size_in_bytes: 64 * 1024,
			idles_before_keep_alive_seconds: 60,
			keep_alive_interval_seconds: 5,
			maximum_keep_alive_probes: 5,
			linger_seconds: 60,
			linger_in_FIN_WAIT2_seconds: 0,
			maximum_SYN_transmits: 1,
			back_log: 128,
		}
	}
}

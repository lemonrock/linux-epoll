// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Holds data that determines if a connection is permitted.
pub trait AccessControl<SD: SocketData>
{
	/// Is the remote peer allowed based on its address or credentials?
	fn is_remote_peer_allowed(&self, remote_peer_address: SD, streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<SD>) -> bool;
}

impl<A: AccessControl<SD>, SD: SocketData> AccessControl<SD> for Rc<A>
{
	#[inline(always)]
	fn is_remote_peer_allowed(&self, remote_peer_address: SD, streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<SD>) -> bool
	{
		self.deref().is_remote_peer_allowed(remote_peer_address, streaming_socket_file_descriptor)
	}
}

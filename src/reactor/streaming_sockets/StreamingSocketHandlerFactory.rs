// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Creates instances of `StreamingSocketHandler`.
pub trait StreamingSocketHandlerFactory<SD: SocketData>: Sized
{
	/// Type returned.
	type StreamingSocketHandler: StreamingSocketHandler<SD>;

	/// Creates a new instance if possible.
	///
	/// Errors will cause a newly accepted connection to be closed.
	fn create(&mut self) -> Result<Self::StreamingSocketHandler, ()>;
}

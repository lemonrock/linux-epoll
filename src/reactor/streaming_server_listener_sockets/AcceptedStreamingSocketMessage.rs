// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug)]
struct AcceptedStreamingSocketMessage<SD: SocketData>
{
	streaming_socket_file_descriptor: StreamingSocketFileDescriptor<SD>,
	streaming_socket_service_identifier: u8,
}

impl<SD: SocketData> AcceptedStreamingSocketMessage<SD>
{
	#[inline(always)]
	pub fn initialize(receiver: NonNull<Self>, streaming_socket_file_descriptor: StreamingSocketFileDescriptor<SD>, streaming_socket_service_identifier: u8)
	{
		unsafe
		{
			write
			(
				receiver.as_mut(),
				Self
				{
					streaming_socket_file_descriptor,
					streaming_socket_service_identifier,
				}
			)
		}
	}
}

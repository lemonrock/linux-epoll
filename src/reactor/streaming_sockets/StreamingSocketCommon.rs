// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug)]
struct StreamingSocketCommon<SSH: StreamingSocketHandler<SD>, SD: SocketData>
{
	streaming_socket_handler: SSH,
}

impl<SSH: StreamingSocketHandler<SD>, SD: SocketData> StreamingSocketCommon<SSH, SD>
{
	#[inline(always)]
	fn do_initial_input_and_output_and_register_with_epoll_if_necesssary(event_poll: &EventPoll<impl Arenas>, (mut streaming_socket_handler, streaming_socket_file_descriptor): (SSH, StreamingSocketFileDescriptor<SD>)) -> Result<(), EventPollRegistrationError>
	{
		let close_as_all_input_and_output_completed = streaming_socket_handler.initial_input_and_output(streaming_socket_file_descriptor);

		if unlikely!(close_as_all_input_and_output_completed)
		{
			drop(streaming_socket_file_descriptor);
			Ok(())
		}

		const AddFlags: EPollAddFlags = EPollAddFlags::Input | EPollAddFlags::InputPriority | EPollAddFlags::Output | EPollAddFlags::ReadShutdown | EPollFlags::EdgeTriggered;

		event_poll.register(streaming_socket_file_descriptor, AddFlags, |uninitialized_this|
		{
			unsafe
			{
				write(&mut uninitialized_this.streaming_socket_handler, streaming_socket_handler);
			}
			Ok(())
		})
	}

	#[inline(always)]
	fn react(&mut self, event_poll: &EventPoll<impl Arenas>, file_descriptor: &StreamingSocketFileDescriptor<SD>, event_flags: EPollEventFlags, _terminate: &impl Terminate) -> Result<bool, String>
	{
		const ClosingWithError: EPollEventFlags = EPollEventFlags::InputPriority | EPollEventFlags::OutOfBandDataCanBeRead | EPollEventFlags::Error | EPollEventFlags::Error | EPollEventFlags::OtherErrorOrNoBuffersQueued;
		const RemotePeerClosedCleanly: EPollEventFlags = EPollEventFlags::ReadShutdown | EPollEventFlags::HangUp;
		if event_flags.intersects(ClosingWithError)
		{
			self.streaming_socket_handler.react_closing_with_error();
			Ok(true)
		}
		else if event_flags.intersects(RemotePeerClosedCleanly)
		{
			self.streaming_socket_handler.react_remote_peer_closed_cleanly();
			Ok(true)
		}
		else
		{
			let read_now_ready = event_flags.contains(EPollEventFlags::Input);
			let write_now_ready = event_flags.contains(EPollEventFlags::Output);

			debug_assert!(read_now_ready || write_now_ready, ("Spurious event with neither read nor write available; flags were `{:?}`", event_flags.bits()));
			Ok(self.streaming_socket_handler.react_input_and_output(file_descriptor, read_now_ready, write_now_ready))
		}
	}
}

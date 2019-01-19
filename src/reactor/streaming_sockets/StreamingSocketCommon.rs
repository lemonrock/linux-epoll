// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug)]
struct StreamingSocketCommon<'a, SSH: StreamingSocketHandler<SD>, SD: SocketData>
{
	marker: PhantomData<&'a SSH>,
}

impl<'a, SSH: StreamingSocketHandler<SD>, SD: SocketData> Coroutine for StreamingSocketCommon<SSH, SD>
{
	type StartArguments = (&'a StreamingSocketFileDescriptor<SD>);

	type ResumeArguments = ReactEdgeTriggeredStatus;

	type Yields = ();

	type Complete = Result<(), CompleteError>;

	fn coroutine(start_arguments: Self::StartArguments, yielder: Yielder<Self::ResumeArguments, Self::Yields, Self::Complete>) -> Self::Complete
	{
		let (all_wrapped_up) = start_arguments;

		let mut byte_counter = ByteCounter::default();

		tls_server_session.complete_handshaking(&streaming_socket_file_descriptor, &mut yielder, &mut byte_counter)?;

		Ok(())
	}
}

impl<SSH: StreamingSocketHandler<SD>, SD: SocketData> StreamingSocketCommon<SSH, SD>
{
	#[inline(always)]
	fn do_initial_input_and_output_and_register_with_epoll_if_necesssary(event_poll: &EventPoll<impl Arenas>, (mut streaming_socket_handler, streaming_socket_file_descriptor): (SSH, StreamingSocketFileDescriptor<SD>)) -> Result<(), EventPollRegistrationError>
	{
		// TODO: pre-allocate and check for allocation failures!
		let	coroutine_stack_size: usize = XXXX;
		let coroutine_stack = ProtectedFixedSizeStack::new(coroutine_stack_size);

		// needs to be at least &streaming_socket_handler and a (TLS) session and higher-level logic...
		let start_data = XXX;


		let started_coroutine = match StackAndTypeSafeTransfer::new(coroutine_stack).start(start_data)
		{
			Right(completed) => return completed.map_err(|complete_error| complete_error.into()),

			Left((), started_coroutine) => started_coroutine,
		};

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
		const ClosedWithError: EPollEventFlags = EPollEventFlags::InputPriority | EPollEventFlags::OutOfBandDataCanBeRead | EPollEventFlags::Error | EPollEventFlags::Error | EPollEventFlags::OtherErrorOrNoBuffersQueued;

		const RemotePeerClosedCleanly: EPollEventFlags = EPollEventFlags::ReadShutdown | EPollEventFlags::HangUp;

		if event_flags.intersects(ClosedWithError)
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

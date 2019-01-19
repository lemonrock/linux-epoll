// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug)]
struct StreamingSocketCommon<'a, SSF: ServerStreamFactory<'a, SD>, SU: StreamUser<SD>, SD: SocketData>
{
	started_coroutine: StartedStackAndTypeSafeTransfer<S, Self>,
	marker: &'a (SSF, SU, SD),
}

impl<'a, SSF: ServerStreamFactory<'a, SD>, SU: StreamUser<SD>, SD: SocketData> Coroutine for StreamingSocketCommon<'a, SSF, SU, SD>
{
	type StartArguments = (&'a StreamingSocketFileDescriptor<SD>, &'a SSF, SSF::AdditionalArguments, &'a SU);

	type ResumeArguments = ReactEdgeTriggeredStatus;

	type Yields = ();

	type Complete = Result<(), CompleteError>;

	#[inline(always)]
	fn coroutine<'yielder>(start_arguments: Self::StartArguments, yielder: Yielder<'yielder, Self::ResumeArguments, Self::Yields, Self::Complete>) -> Self::Complete
	{
		let (streaming_socket_file_descriptor, server_stream_factory, additional_arguments, stream_user) = start_arguments;

		let stream = server_stream_factory.new_stream_and_handshake(streaming_socket_file_descriptor, yielder, additional_arguments)?;

		stream_user.use_stream(stream)
	}
}

impl<'a, SSF: ServerStreamFactory<'a, SD>, SU: StreamUser<SD>, SD: SocketData> StreamingSocketCommon<'a, SSF, SU, SD>
{
	#[inline(always)]
	fn do_initial_input_and_output_and_register_with_epoll_if_necesssary(event_poll: &EventPoll<impl Arenas>, (streaming_socket_file_descriptor, server_stream_factory, additional_arguments, stream_user): (StreamingSocketFileDescriptor<SD>, &'a SSF, SSF::AdditionalArguments, &'a SU)) -> Result<(), EventPollRegistrationError>
	{
		// TODO: pre-allocate and check for allocation failures!
		let	coroutine_stack_size: usize = XXXX;
		let coroutine_stack = ProtectedFixedSizeStack::new(coroutine_stack_size);
		let start_data = (&streaming_socket_handler, server_stream_factory, additional_arguments, stream_user);

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
				write(&mut uninitialized_this.started_coroutine, started_coroutine);
			}
			Ok(())
		})
	}

	#[inline(always)]
	fn react(&mut self, event_poll: &EventPoll<impl Arenas>, file_descriptor: &StreamingSocketFileDescriptor<SD>, event_flags: EPollEventFlags, _terminate: &impl Terminate) -> Result<bool, String>
	{
		const ClosedWithError: EPollEventFlags = EPollEventFlags::InputPriority | EPollEventFlags::OutOfBandDataCanBeRead | EPollEventFlags::Error | EPollEventFlags::Error | EPollEventFlags::OtherErrorOrNoBuffersQueued;

		const RemotePeerClosedCleanly: EPollEventFlags = EPollEventFlags::ReadShutdown | EPollEventFlags::HangUp;

		use self::ReactEdgeTriggeredStatus::*;

		if event_flags.intersects(ClosedWithError)
		{
			match self.started_coroutine.resume(ClosedWithError)
			{
				Left(_yields @ ()) => if cfg!(debug_assertions)
				{
					panic!("Should have terminated")
				}
				else
				{
					unreachable!()
				},
				Right(_complete) => Ok(true),
			}
		}
		else if event_flags.intersects(RemotePeerClosedCleanly)
		{
			match self.started_coroutine.resume(ClosedWithError)
			{
				Left(_yields @ ()) => if cfg!(debug_assertions)
				{
					panic!("Should have terminated")
				}
				else
				{
					unreachable!()
				},
				Right(_complete) => Ok(true),
			}
		}
		else
		{
			let read_now_ready = event_flags.contains(EPollEventFlags::Input);
			let write_now_ready = event_flags.contains(EPollEventFlags::Output);
			debug_assert!(read_now_ready || write_now_ready, ("Spurious event with neither read nor write available; flags were `{:?}`", event_flags.bits()));

			match self.started_coroutine.resume(InputOrOutputNowAvailable { read_now_ready, write_now_ready })
			{
				Left(_yields @ ()) => Ok(false),
				Right(complete) => Ok(complete.is_err()),
			}
		}
	}
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


struct StreamingSocketCommon<'a, SF: 'a + StreamFactory<'a, SD>, SU: 'a + StreamUser<'a, SF::S>, SD: 'a + SocketData>
{
	started_coroutine: StartedStackAndTypeSafeTransfer<'a, SimpleStack, Self>,
}

impl<'a, SF: 'a + StreamFactory<'a, SD>, SU: 'a + StreamUser<'a, SF::S>, SD: 'a + SocketData> Debug for StreamingSocketCommon<'a, SF, SU, SD>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "StreamingSocketCommon {{ started_coroutine: {:?} }}", self.started_coroutine)
	}
}

impl<'a, SF: 'a + StreamFactory<'a, SD>, SU: 'a + StreamUser<'a, SF::S>, SD: 'a + SocketData> Coroutine<'a> for StreamingSocketCommon<'a, SF, SU, SD>
{
	type StartArguments = (&'a StreamingSocketFileDescriptor<SD>, &'a SF, SF::AdditionalArguments, &'a SU);

	type ResumeArguments = ReactEdgeTriggeredStatus;

	type Yields = ();

	type Complete = Result<(), CompleteError>;

	#[inline(always)]
	fn coroutine<'yielder: 'a>(start_arguments: Self::StartArguments, yielder: Yielder<'yielder, Self::ResumeArguments, Self::Yields, Self::Complete>) -> Self::Complete
	{
		let (streaming_socket_file_descriptor, server_stream_factory, additional_arguments, stream_user) = start_arguments;

		let stream = server_stream_factory.new_stream_and_handshake(streaming_socket_file_descriptor, yielder, additional_arguments)?;

		stream_user.use_stream(stream)
	}
}

impl<'a, SF: 'a + StreamFactory<'a, SD>, SU: 'a + StreamUser<'a, SF::S>, SD: 'a + SocketData> StreamingSocketCommon<'a, SF, SU, SD>
{
	#[inline(always)]
	fn do_initial_input_and_output_and_register_with_epoll_if_necesssary<SSR: StreamingSocketReactor<'a, SF, SU, SD>>(event_poll: &EventPoll<impl Arenas>, (streaming_socket_file_descriptor, server_stream_factory, additional_arguments, stream_user): (SSR::FileDescriptor, &'a SF, SF::AdditionalArguments, &'a SU)) -> Result<(), EventPollRegistrationError>
	{
		let start_data = (&streaming_socket_file_descriptor, server_stream_factory, additional_arguments, stream_user);

		let started_coroutine = match StackAndTypeSafeTransfer::new(SimpleStack).start(start_data)
		{
			Right(completed) => return { completed?; Ok(()) },

			Left(((), started_coroutine)) => started_coroutine,
		};

		const AddFlags: EPollAddFlags = EPollAddFlags::Input | EPollAddFlags::InputPriority | EPollAddFlags::Output | EPollAddFlags::ReadShutdown | EPollAddFlags::EdgeTriggered;

		event_poll.register::<SSR>(streaming_socket_file_descriptor, AddFlags, |uninitialized_reactor|
		{
			uninitialized_reactor.initialize
			(
				Self
				{
					started_coroutine,
				}
			);
			Ok(())
		})
	}

	#[inline(always)]
	fn react(&mut self, event_poll: &EventPoll<impl Arenas>, file_descriptor: &StreamingSocketFileDescriptor<SD>, event_flags: EPollEventFlags, _terminate: &impl Terminate) -> Result<bool, String>
	{
		use self::ReactEdgeTriggeredStatus::*;

		if event_flags.intersects(EPollEventFlags::InputPriority | EPollEventFlags::OutOfBandDataCanBeRead | EPollEventFlags::Error | EPollEventFlags::Error | EPollEventFlags::OtherErrorOrNoBuffersQueued)
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
		else if event_flags.intersects(EPollEventFlags::ReadShutdown | EPollEventFlags::HangUp)
		{
			match self.started_coroutine.resume(RemotePeerClosedCleanly)
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

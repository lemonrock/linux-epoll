// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


struct StreamingSocketCommon<SF: StreamFactory<SD>, SU: StreamUser<SF::S>, SD: SocketData>
{
	started_coroutine: StartedStackAndTypeSafeTransfer<SimpleStack, Self>,
}

impl<SF: StreamFactory<SD>, SU: StreamUser<SF::S>, SD: SocketData> Debug for StreamingSocketCommon<SF, SU, SD>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "StreamingSocketCommon {{ started_coroutine: {:?} }}", self.started_coroutine)
	}
}

impl<SF: StreamFactory<SD>, SU: StreamUser<SF::S>, SD: SocketData> Coroutine for StreamingSocketCommon<SF, SU, SD>
{
	type StartArguments = (ManuallyDrop<StreamingSocketFileDescriptor<SD>>, NonNull<SF>, NonNull<SF::AdditionalArguments>, Rc<SU>);

	type ResumeArguments = ReactEdgeTriggeredStatus;

	type Yields = ();

	type Complete = Result<(), CompleteError>;

	#[inline(always)]
	fn coroutine<'yielder>(start_arguments: Self::StartArguments, yielder: Yielder<'yielder, Self::ResumeArguments, Self::Yields, Self::Complete>) -> Self::Complete
	{
		let (streaming_socket_file_descriptor, server_stream_factory_non_null, additional_arguments, stream_user) = start_arguments;

		let server_stream_factory = unsafe { server_stream_factory_non_null.as_ref() };

		let stream = server_stream_factory.new_stream_and_handshake(streaming_socket_file_descriptor, yielder, additional_arguments)?;

		stream_user.use_stream(stream)
	}
}

impl<SF: StreamFactory<SD>, SU: StreamUser<SF::S>, SD: SocketData> StreamingSocketCommon<SF, SU, SD>
{
	#[inline(always)]
	fn do_initial_input_and_output_and_register_with_epoll_if_necesssary<SSR: StreamingSocketReactor<SF, SU, SD, AS, A>, AS: Arenas, A: Arena<SSR, AS>>(event_poll: &EventPoll<AS>, (streaming_socket_file_descriptor, server_stream_factory, additional_arguments, stream_user): (SSR::FileDescriptor, &SF, SF::AdditionalArguments, Rc<SU>)) -> Result<(), EventPollRegistrationError>
	{
		// TODO: Sort out types...


		// TODO: We have to hold references to server_stream_factory, additional_arguments, stream_user otherwise they can go out of scope (after start()).


		XXXXXX


		let start_arguments = unsafe
		{
			(
				NonNull::new_unchecked(&streaming_socket_file_descriptor as *const _ as *mut _),
				NonNull::new_unchecked(server_stream_factory as *const _ as *mut _),

			)
		};

		let started_coroutine = match StackAndTypeSafeTransfer::new(SimpleStack).start((&streaming_socket_file_descriptor, server_stream_factory, additional_arguments, stream_user))
		{
			Right(completed) => return { completed?; Ok(()) },

			Left(((), started_coroutine)) => started_coroutine,
		};

		event_poll.register::<SSR, A, _>(streaming_socket_file_descriptor, EPollAddFlags::Streaming, |uninitialized_reactor|
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
	fn react(&mut self, _event_poll: &EventPoll<impl Arenas>, _file_descriptor: &StreamingSocketFileDescriptor<SD>, event_flags: EPollEventFlags, _terminate: &impl Terminate) -> Result<bool, String>
	{
		use self::ReactEdgeTriggeredStatus::*;

		if event_flags.intersects(EPollEventFlags::CloseWithError)
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
		else if event_flags.intersects(EPollEventFlags::RemotePeerClosedCleanly)
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

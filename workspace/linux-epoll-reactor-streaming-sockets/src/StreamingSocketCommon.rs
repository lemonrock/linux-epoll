// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.
//


#[doc(hidden)]
pub struct StreamingSocketCommon<SF: StreamFactory<SD>, SU: StreamUser<SF::S>, SD: SocketData>
{
	started_coroutine: StartedStackAndTypeSafeTransfer<SimpleStack, Self>,
}

#[doc(hidden)]
impl<SF: StreamFactory<SD>, SU: StreamUser<SF::S>, SD: SocketData> Debug for StreamingSocketCommon<SF, SU, SD>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "StreamingSocketCommon {{ started_coroutine: {:?} }}", self.started_coroutine)
	}
}

#[doc(hidden)]
impl<SF: StreamFactory<SD>, SU: StreamUser<SF::S>, SD: SocketData> Coroutine for StreamingSocketCommon<SF, SU, SD>
{
	type StartArguments = (StreamingSocketFileDescriptor<SD>, Rc<SF>, SF::AdditionalArguments, Rc<SU>);

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

#[doc(hidden)]
impl<SF: StreamFactory<SD>, SU: StreamUser<SF::S>, SD: SocketData> StreamingSocketCommon<SF, SU, SD>
{
	#[inline(always)]
	fn do_initial_input_and_output_and_register_with_epoll_if_necesssary<A: Arena<SSR>, SSR: StreamingSocketReactor<SF, SU, SD>, EPR: EventPollRegister>(event_poll_register: &EPR, arena: &A, reactor_compressed_type_identifier: CompressedTypeIdentifier, (streaming_socket_file_descriptor, server_stream_factory, additional_arguments, stream_user): (SSR::FileDescriptor, Rc<SF>, SF::AdditionalArguments, Rc<SU>)) -> Result<(), EventPollRegistrationError>
	{
		let start_arguments =
		(
			unsafe { transmute_copy(&streaming_socket_file_descriptor) },
			server_stream_factory,
			additional_arguments,
			stream_user,
		);

		use self::StartOutcome::*;

		let started_coroutine = match StackAndTypeSafeTransfer::new(SimpleStack).start(start_arguments)
		{
			Complete(Ok(())) => return Ok(()),

			Complete(Err(complete_error)) => return Err(EventPollRegistrationError::InitialInputOrOutputFailed(Box::new(complete_error))),

			WouldLikeToResume((), started_coroutine) => started_coroutine,
		};

		event_poll_register.register::<A, SSR, _>(arena, reactor_compressed_type_identifier, streaming_socket_file_descriptor, EPollAddFlags::Streaming, |uninitialized_reactor, streaming_socket_file_descriptor|
		{
			forget(streaming_socket_file_descriptor);

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
	fn react(&mut self, event_flags: EPollEventFlags, _terminate: &impl Terminate) -> Result<bool, String>
	{
		use self::ReactEdgeTriggeredStatus::*;

		use self::ResumeOutcome::*;

		if event_flags.intersects(EPollEventFlags::CloseWithError)
		{
			match self.started_coroutine.resume(ClosedWithError)
			{
				WouldLikeToResume(_yields @ ()) => if cfg!(debug_assertions)
				{
					panic!("Should have terminated")
				}
				else
				{
					unreachable!()
				},

				Complete(_complete) => Ok(true),
			}
		}
		else if event_flags.intersects(EPollEventFlags::RemotePeerClosedCleanly)
		{
			match self.started_coroutine.resume(RemotePeerClosedCleanly)
			{
				WouldLikeToResume(_yields @ ()) => if cfg!(debug_assertions)
				{
					panic!("Should have terminated")
				}
				else
				{
					unreachable!()
				},

				Complete(_complete) => Ok(true),
			}
		}
		else
		{
			let read_now_ready = event_flags.contains(EPollEventFlags::Input);
			let write_now_ready = event_flags.contains(EPollEventFlags::Output);
			debug_assert!(read_now_ready || write_now_ready, ("Spurious event with neither read nor write available; flags were `{:?}`", event_flags.bits()));

			match self.started_coroutine.resume(InputOrOutputNowAvailable { read_now_ready, write_now_ready })
			{
				WouldLikeToResume(_yields @ ()) => Ok(false),

				Complete(_complete) => Ok(true),
			}
		}
	}
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug)]
pub struct TlsStreamingSocketHandler<USSH: StreamingSocketHandler<SD>, SD: SocketData, S: Sized + Deref<Stack>>
{
	underlying_streaming_socket_handler: USSH,
	coroutine: StackAndTypeSafeTransfer<S, Self>,
	tls_server_session: ServerSession,
}

impl<USSH: StreamingSocketHandler<SD>, SD: SocketData> Coroutine for TlsStreamingSocketHandler<USSH, SD>
{
	/// Type of the arguments the coroutine is initially called with, eg `(usize, String)`.
	type StartArguments = (AllWrappedUp<SD>);

	/// Type of the arguments the coroutine is resumed with, eg `(u8, Vec<f64>)`.
	type ResumeArguments = ReactEdgeTriggeredArguments;

	/// Type of the result from a yield of the coroutine.
	type Yields = ();

	/// Type of the final result from the coroutine.
	type Complete = Result<(), TlsInputOutputError>;

	/// Implement this for the coroutine's behaviour.
	///
	/// Panics inside the coroutine are transferred to the calling thread and raised.
	fn coroutine(start_arguments: Self::StartArguments, yielder: Yielder<Self::ResumeArguments, Self::Yields, Self::Complete>) -> Self::Complete
	{
		let (all_wrapped_up) = start_arguments;

		// TODO: Identify connections that are idle and taking up resources; we only really need to kill them if the arena is full.
		let mut byte_counter = ByteCounter::default();

		tls_server_session.complete_handshaking(&streaming_socket_file_descriptor, &mut yielder, &mut byte_counter)?;

		struct AllWrappedUp<SD: SocketData>
		{
		}

		impl<SD: SocketData> AllWrappedUp<SD>
		{
			#[inline(always)]
			pub(crate) fn complete_handshaking(&mut self, yielder: &mut TlsYielder) -> Result<(), TlsInputOutputError>
			{
				self.tls_server_session.complete_handshaking::<SD>(&self.streaming_socket_file_descriptor, yielder, &mut self.byte_counter, buf)
			}

			/// Similar to `io::Read.read()`.
			#[inline(always)]
			pub fn read(&mut self, yielder: &mut TlsYielder, buf: &mut [u8]) -> Result<usize, TlsInputOutputError>
			{
				self.tls_server_session.stream_read::<SD>(&self.streaming_socket_file_descriptor, yielder, &mut self.byte_counter, buf)
			}

			/// Similar to `io::Write.write()`.
			#[inline(always)]
			pub fn write(&mut self, yielder: &mut TlsYielder, buf: &[u8]) -> Result<usize, TlsInputOutputError>
			{
				self.tls_server_session.stream_write::<SD>(&self.streaming_socket_file_descriptor, yielder, &mut self.byte_counter, buf)
			}
		}




		Ok(())

	}
}

impl XXXXXXX
{
}

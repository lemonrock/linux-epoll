// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug)]
pub struct TlsStreamingSocketHandlerFactory<USSHF: StreamingSocketHandlerFactory<SD>, SD: SocketData>
{
	underlying_streaming_socket_handler_factory: USSHF,
	coroutine_stack_size: usize,
	tls_server_configuration: Arc<ServerConfig>,
	tls_session_buffer_limit: usize,
}

impl<USSHF: StreamingSocketHandlerFactory<SD>, SD: SocketData> StreamingSocketHandlerFactory<SD> for TlsStreamingSocketHandlerFactory<USSHF, SD>
{
	type StreamingSocketHandler = TlsStreamingSocketHandler<USSHF::StreamingSocketHandler, SD>;

	#[inline(always)]
	fn create(&mut self) -> Result<Self::StreamingSocketHandler, ()>
	{

		// TODO: We don't have access to streaming_socket_file_descriptor: StreamingSocketFileDescriptor<SD> - it is provided on each react by the connection.


		struct AllWrappedUp<SD: SocketData, S: Session>
		{
			streaming_socket_file_descriptor: StreamingSocketFileDescriptor<SD>,
			tls_session: S,
			byte_counter: ByteCounter,
		}

		let all_wrapped_up = AllWrappedUp
		{
			streaming_socket_file_descriptor,
			tls_session:
			{
				let mut session = ServerSession::new(&self.tls_server_configuration);
				session.set_buffer_limit(self.tls_session_buffer_limit);
				session
			},
			byte_counter: ByteCounter::default(),
		};

		Ok
		(
			TlsStreamingSocketHandler
			{
				underlying_streaming_socket_handler: self.underlying_streaming_socket_handler_factory.new()?,
				coroutine: StackAndTypeSafeTransfer::new(coroutine_stack, TlsStreamingSocketHandler::<USSHF::StreamingSocketHandler, SD>::coroutine, all_wrapped_up),
			}
		)
	}
}

impl<USSHF: StreamingSocketHandlerFactory<SD>, SD: SocketData> TlsStreamingSocketHandlerFactory<USSHF, SD>
{
	#[inline(always)]
	pub fn new(underlying_streaming_socket_handler_factory: USSHF, coroutine_stack_size: usize, tls_configuration: &TlsServerConfiguration) -> Result<Self, TlsServerConfigurationError>
	{
		Ok
		(
			Self
			{
				underlying_streaming_socket_handler_factory,
				coroutine_stack_size,
				tls_server_configuration: tls_configuration.server_configuration()?,
				tls_session_buffer_limit: tls_configuration.session_buffer_limit,
			}
		)
	}
}

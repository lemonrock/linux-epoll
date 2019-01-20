// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A factory to create instances of `TlsClientStream`.
pub struct TlsClientStreamFactory
{
	tls_configuration: Arc<ClientConfig>,
	session_buffer_limit: usize,
}

impl Debug for TlsClientStreamFactory
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "TlsClientStreamFactory {{ tls_configuration: _, session_buffer_limit: {:?} }}", self.session_buffer_limit)
	}
}

impl<'a, SD: 'a + SocketData> StreamFactory<'a, SD> for TlsClientStreamFactory
{
	type S = TlsClientStream<'a, SD>;

	/// The ASCII host name.
	type AdditionalArguments = &'a str;

	#[inline(always)]
	fn new_stream_and_handshake(&self, streaming_socket_file_descriptor: &'a StreamingSocketFileDescriptor<SD>, yielder: Yielder<'a, ReactEdgeTriggeredStatus, (), Result<(), CompleteError>>, additional_arguments: Self::AdditionalArguments) -> Result<Self::S, CompleteError>
	{
		let ascii_host_name = additional_arguments;

		let generic_stream = GenericStream::wrap(streaming_socket_file_descriptor, yielder);
		TlsClientStream::new(generic_stream, &self.tls_configuration, self.session_buffer_limit, ascii_host_name)
	}
}

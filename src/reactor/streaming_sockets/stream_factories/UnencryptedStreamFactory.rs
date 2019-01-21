// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A factory to create instances of `UnencryptedStream`.
#[derive(Debug)]
pub struct UnencryptedStreamFactory;

impl<SD: SocketData> StreamFactory<SD> for UnencryptedStreamFactory
{
	/// This is a lie; the lifetime is ***NOT*** `'static` but actually `'yielder` in `new_stream_and_handshake()`.
	/// Rust's lack of associated type constructors makes this impossible to express; `yielder` is ***NOT*** a parameter to put on `UnencryptedStreamFactory` (eg `UnencryptedStreamFactory<'yielder>`) because the lifetime has nothing to do with the lifetime of the factory (indeed, factories live far longer than the coroutines they are invovled in instantiating).
	type S = UnencryptedStream<'static, SD>;

	type AdditionalArguments = ();

	#[inline(always)]
	fn new_stream_and_handshake<'yielder>(&self, streaming_socket_file_descriptor: ManuallyDrop<StreamingSocketFileDescriptor<SD>>, yielder: Yielder<'yielder, ReactEdgeTriggeredStatus, (), Result<(), CompleteError>>, additional_arguments: Self::AdditionalArguments) -> Result<Self::S, CompleteError>
	{
		let generic_stream = GenericStream::wrap(streaming_socket_file_descriptor, yielder);
		UnencryptedStream::new(generic_stream)
	}
}

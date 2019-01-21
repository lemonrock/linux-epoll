// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A factory to abstract the creation of `Stream` instances.
pub trait StreamFactory<SD: SocketData>
{
	/// The type of the `Stream` being created.
	type S: Stream;

	/// Any additional data needed to instantiate a new stream.
	///
	/// For example, for TLS client sessions, one has to know the DNS host name of the destination server.
	///
	/// These are passed as a pointer.
	type AdditionalArguments;

	/// Creates a new stream, initiates handshaking on it if required, then returns it or an error.
	///
	/// Always called within a coroutine.
	fn new_stream_and_handshake<'yielder>(&self, streaming_socket_file_descriptor: ManuallyDrop<StreamingSocketFileDescriptor<SD>>, yielder: Yielder<'yielder, ReactEdgeTriggeredStatus, (), Result<(), CompleteError>>, additional_arguments: Self::AdditionalArguments) -> Result<Self::S, CompleteError>;
}

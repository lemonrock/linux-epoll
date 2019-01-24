// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Represents a handler for the contents of a distributed file descriptor message.
///
/// Sadly, we can not use an enumeration as we do not know the implementations of this trait at compile time of this library.
///
/// (We can potentially know it at compile time of a downstream dependent binary).
pub trait DistributedFileDescriptorMessageContents
{
	/// Size of the message contents, ie can be implemented as `::std::mem::size_of::<Self>()` unless the struct implementing this contains a DST or extern type.
	fn size_of(&self) -> usize;
}

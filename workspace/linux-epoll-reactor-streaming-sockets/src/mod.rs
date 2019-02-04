// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.
//


use super::*;


/// Stream abstracts to make common the difference between TLS and non-TLS sockets.
#[macro_use] pub mod streams;


/// Factories to create streams.
pub mod stream_factories;


include!("streaming_socket_reactor.rs");


include!("StreamingSocketCommon.rs");
include!("StreamingSocketInternetProtocolVersion4Reactor.rs");
include!("StreamingSocketInternetProtocolVersion6Reactor.rs");
include!("StreamingSocketReactor.rs");
include!("StreamingSocketUnixDomainReactor.rs");

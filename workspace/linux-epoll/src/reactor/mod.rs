// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


use super::*;


/// Streaming socket reactors and supporting logic.
#[macro_use] pub mod streaming_sockets;


/// Streaming server listener socket reactors and supporting logic.
pub mod streaming_server_listener_sockets;


include!("AdaptedReactorsRegistrar.rs");
include!("CompleteError.rs");
include!("InputOutputYielder.rs");
include!("ReactEdgeTriggeredStatus.rs");
include!("Reactor.rs");
include!("ReactorsRegistrar.rs");
include!("ReactorsRegistration.rs");

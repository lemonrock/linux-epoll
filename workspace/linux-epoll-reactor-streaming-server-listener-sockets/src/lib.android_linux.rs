// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[macro_use] extern crate likely;
extern crate linux_epoll;
extern crate treebitmap;


use self::access_control::*;
use ::linux_epoll::libc::gid_t;
use ::linux_epoll::libc::uid_t;
use ::linux_epoll::*;
use ::linux_epoll::arena::*;
use ::linux_epoll::cpu_affinity::LogicalCores;
use ::linux_epoll::file_descriptors::epoll::*;
use ::linux_epoll::file_descriptors::socket::*;
use ::linux_epoll::file_descriptors::socket::syscall::sockaddr_in;
use ::linux_epoll::file_descriptors::socket::syscall::sockaddr_in6;
use ::linux_epoll::file_descriptors::socket::syscall::sockaddr_un;
use ::linux_epoll::hashbrown::*;
use ::linux_epoll::message_dispatch::QueuePerThreadQueuesPublisher;
use ::linux_epoll::reactor::*;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::mem::transmute;
use ::std::hash::Hash;
use ::std::net::Ipv4Addr;
use ::std::net::Ipv6Addr;
use ::std::net::SocketAddrV4;
use ::std::net::SocketAddrV6;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
use ::std::path::PathBuf;
use ::std::ptr::NonNull;
use ::std::ptr::write;
use ::std::rc::Rc;
use ::std::sync::Arc;
use ::treebitmap::IpLookupTable;


/// Access control for streaming sockets.
pub mod access_control;


include!("AcceptedStreamingSocketMessage.rs");
include!("streaming_server_listener_reactor.rs");
include!("StreamingServerListenerSocketCommon.rs");
include!("StreamingServerListenerSocketInternetProtocolVersion4Reactor.rs");
include!("StreamingServerListenerSocketInternetProtocolVersion6Reactor.rs");
include!("StreamingServerListenerSocketReactor.rs");
include!("StreamingServerListenerSocketSettings.rs");
include!("StreamingServerListenerSocketUnixDomainReactor.rs");
include!("UnixDomainSocketAddress.rs");

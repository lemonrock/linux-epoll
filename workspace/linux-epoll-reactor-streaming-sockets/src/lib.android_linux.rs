// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


extern crate arrayvec;
extern crate context_coroutine;
extern crate hashbrown;
#[macro_use] extern crate likely;
extern crate linux_epoll;
extern crate rustls_extra;


use self::stream_factories::*;
use self::streams::*;
use ::arrayvec::ArrayVec;
use ::context_coroutine::*;
use ::hashbrown::HashMap;
use ::hashbrown::hash_map::Entry;
use ::linux_epoll::*;
use ::linux_epoll::arena::*;
#[allow(unused_imports)] use ::linux_epoll::file_descriptors::VectoredWrite;
use ::linux_epoll::file_descriptors::epoll::*;
use ::linux_epoll::file_descriptors::socket::*;
use ::linux_epoll::file_descriptors::socket::syscall::sockaddr_in;
use ::linux_epoll::file_descriptors::socket::syscall::sockaddr_in6;
use ::linux_epoll::file_descriptors::socket::syscall::sockaddr_un;
use ::linux_epoll::reactor::*;
use ::rustls_extra::*;
pub use ::rustls_extra::supported_cipher_suites;
use ::std::error;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::mem::transmute;
use ::std::io;
use ::std::io::ErrorKind;
use ::std::io::Initializer;
use ::std::io::Read;
use ::std::io::Write;
use ::std::mem::forget;
use ::std::mem::transmute_copy;
use ::std::mem::uninitialized;
use ::std::net::IpAddr;
use ::std::net::Ipv4Addr;
use ::std::net::Ipv6Addr;
use ::std::net::SocketAddrV4;
use ::std::ops::Deref;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::write;
use ::std::rc::Rc;
use ::std::sync::Arc;


/// Stream factories.
pub mod stream_factories;


/// Streams.
pub mod streams;


include!("streaming_socket_reactor.rs");


include!("CompleteError.rs");
include!("ReactEdgeTriggeredStatus.rs");
include!("StreamingSocketCommon.rs");
include!("StreamingSocketInternetProtocolVersion4Reactor.rs");
include!("StreamingSocketInternetProtocolVersion6Reactor.rs");
include!("StreamingSocketReactor.rs");
include!("StreamingSocketUnixDomainReactor.rs");
include!("TlsInputOutputError.rs");

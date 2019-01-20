// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


extern crate context;
extern crate cpu_affinity;
extern crate ct_logs;
extern crate either;
extern crate file_descriptors;
extern crate indexmap;
extern crate libc;
#[macro_use] extern crate likely;
extern crate lock_free_multi_producer_single_consumer_ring_buffer;
extern crate rustls;
extern crate treebitmap;
extern crate webpki;


use self::arena::*;
use self::arenas::*;
use self::coroutine::*;
use self::reactor::*;
use self::reactor::distribution::*;
use self::reactor::streaming_sockets::*;
#[macro_use] use self::reactor::streaming_sockets::streams::*;
use self::reactor::streaming_sockets::stream_factories::*;
use self::reactor::streaming_server_listener_sockets::access_control::*;
use self::terminate::*;
use self::tls::*;
use ::context::context::*;
use ::context::stack::*;
use ::cpu_affinity::*;
use ::ct_logs::LOGS as GooglesKnownListOfCertificateTransparencyLogs;
use ::either::*;
use ::file_descriptors::*;
use ::file_descriptors::character_device::CharacterDeviceFileDescriptor;
use ::file_descriptors::epoll::*;
use ::file_descriptors::epoll::syscall::*;
use ::file_descriptors::eventfd::EventFileDescriptor;
use ::file_descriptors::fanotify::FanotifyFileDescriptor;
use ::file_descriptors::inotify::InotifyFileDescriptor;
use ::file_descriptors::pipes_and_fifos::ReceivePipeFileDescriptor;
use ::file_descriptors::pipes_and_fifos::SendPipeFileDescriptor;
use ::file_descriptors::posix_message_queues::ReceivePosixMessageQueueFileDescriptor;
use ::file_descriptors::posix_message_queues::SendPosixMessageQueueFileDescriptor;
use ::file_descriptors::posix_message_queues::SendAndReceivePosixMessageQueueFileDescriptor;
use ::file_descriptors::socket::*;
use ::file_descriptors::socket::syscall::*;
use ::file_descriptors::signalfd::*;
use ::file_descriptors::signalfd::syscall::signalfd_siginfo;
use ::file_descriptors::timerfd::TimerFileDescriptor;
use ::file_descriptors::terminal::TerminalFileDescriptor;
use ::indexmap::IndexSet;
use ::libc::gid_t;
use ::libc::sched_getcpu;
use ::libc::uid_t;
use ::lock_free_multi_producer_single_consumer_ring_buffer::*;
use ::rustls::ALL_CIPHERSUITES;
use ::rustls::AllowAnyAnonymousOrAuthenticatedClient;
use ::rustls::AllowAnyAuthenticatedClient;
use ::rustls::Certificate;
use ::rustls::ClientCertVerifier;
use ::rustls::ClientConfig;
use ::rustls::ClientSession;
use ::rustls::ClientSessionMemoryCache;
use ::rustls::NoClientAuth;
use ::rustls::NoClientSessionStorage;
use ::rustls::NoServerSessionStorage;
use ::rustls::PrivateKey;
use ::rustls::ProtocolVersion;
use ::rustls::RootCertStore;
use ::rustls::ServerConfig;
use ::rustls::ServerSession;
use ::rustls::ServerSessionMemoryCache;
use ::rustls::Session;
use ::rustls::SupportedCipherSuite;
use ::rustls::Ticketer;
use ::rustls::TLSError;
use ::rustls::WriteV;
use ::rustls::internal::pemfile::*;
use ::std::cell::Cell;
use ::std::cell::UnsafeCell;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::error;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fs::File;
use ::std::hash::Hash;
use ::std::io;
use ::std::io::BufReader;
use ::std::io::ErrorKind;
use ::std::io::Initializer;
use ::std::io::Read;
use ::std::io::Write;
use ::std::marker::PhantomData;
use ::std::mem::forget;
use ::std::mem::ManuallyDrop;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::net::Ipv4Addr;
use ::std::net::Ipv6Addr;
use ::std::net::SocketAddrV4;
use ::std::net::SocketAddrV6;
use ::std::ops::BitAnd;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
use ::std::os::unix::io::AsRawFd;
use ::std::os::unix::io::FromRawFd;
use ::std::os::unix::io::RawFd;
use ::std::panic::AssertUnwindSafe;
use ::std::panic::catch_unwind;
use ::std::panic::PanicInfo;
use ::std::panic::resume_unwind;
use ::std::path::PathBuf;
use ::std::ptr::drop_in_place;
use ::std::ptr::NonNull;
use ::std::ptr::write;
use ::std::rc::Rc;
use ::std::sync::Arc;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::thread;
use ::treebitmap::IpLookupTable;
use ::webpki::DNSNameRef;


/// Implementations of the `Arena` trait.
pub mod arena;


/// Structures to ease use of coroutines (continuations) used for handling sockets and the like.
pub mod coroutine;


/// Implementations of the `Arenas` trait.
pub mod arenas;


/// Implementations of the `Ractor` trait.
#[macro_use] pub mod reactor;


/// Implementations of the `Terminate` trait.
pub mod terminate;


/// Supporting logic for TLS.
pub mod tls;


/// Horrible hack to make public a constant from rustls.
pub const TLS13_CHACHA20_POLY1305_SHA256: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[0];

/// Horrible hack to make public a constant from rustls.
pub const TLS13_AES_256_GCM_SHA384: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[1];

/// Horrible hack to make public a constant from rustls.
pub const TLS13_AES_128_GCM_SHA256: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[2];

/// Horrible hack to make public a constant from rustls.
pub const TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[3];

/// Horrible hack to make public a constant from rustls.
pub const TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[4];

/// Horrible hack to make public a constant from rustls.
pub const TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[5];

/// Horrible hack to make public a constant from rustls.
pub const TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[6];

/// Horrible hack to make public a constant from rustls.
pub const TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[7];

/// Horrible hack to make public a constant from rustls.
pub const TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[8];


include!("EventPoll.rs");
include!("EventPollRegistrationError.rs");
include!("EventPollToken.rs");
include!("FileDescriptorKind.rs");
include!("Unused.rs");

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


extern crate context;
extern crate cpu_affinity;
extern crate ct_logs;
extern crate either;
extern crate file_descriptors;
extern crate indexmap;
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
use ::file_descriptors::signalfd::*;
use ::file_descriptors::signalfd::syscall::signalfd_siginfo;
use ::file_descriptors::timerfd::TimerFileDescriptor;
use ::file_descriptors::terminal::TerminalFileDescriptor;
use ::indexmap::IndexSet;
use ::lock_free_multi_producer_single_consumer_ring_buffer::*;
use ::rustls::Certificate;
use ::rustls::ClientConfig;
use ::rustls::ClientSession;
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
use ::std::collections::HashSet;
use ::std::error;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fs::File;
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
use ::std::ops::BitAnd;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
use ::std::os::unix::io::AsRawFd;
use ::std::os::unix::io::FromRawFd;
use ::std::os::unix::io::RawFd;
use ::std::panic::catch_unwind;
use ::std::panic::PanicInfo;
use ::std::panic::resume_unwind;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::ptr::drop_in_place;
use ::std::ptr::NonNull;
use ::std::ptr::write;
use ::std::sync::Arc;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::thread;
use ::treebitmap::IpLookupTable;
use ::webpki::DNSNameRef;
use ::webpki::ECDSA_P256_SHA256;
use ::webpki::ECDSA_P256_SHA384;
use ::webpki::ECDSA_P384_SHA256;
use ::webpki::ECDSA_P384_SHA384;
use ::webpki::RSA_PKCS1_2048_8192_SHA256;
use ::webpki::RSA_PKCS1_2048_8192_SHA384;
use ::webpki::RSA_PKCS1_2048_8192_SHA512;
use ::webpki::RSA_PKCS1_3072_8192_SHA384;
use ::webpki::RSA_PSS_2048_8192_SHA256_LEGACY_KEY;
use ::webpki::RSA_PSS_2048_8192_SHA384_LEGACY_KEY;
use ::webpki::RSA_PSS_2048_8192_SHA512_LEGACY_KEY;
use ::webpki::SignatureAlgorithm;


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


include!("EventPoll.rs");
include!("EventPollRegistrationError.rs");
include!("EventPollToken.rs");
include!("FileDescriptorKind.rs");
include!("Unused.rs");

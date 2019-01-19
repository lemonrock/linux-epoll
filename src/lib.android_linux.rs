// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


extern crate context;
extern crate cpu_affinity;
extern crate file_descriptors;
extern crate indexmap;
#[macro_use] extern crate likely;
extern crate lock_free_multi_producer_single_consumer_ring_buffer;
extern crate rustls;
extern crate treebitmap;


use self::arena::*;
use self::arenas::*;
use self::coroutine::*;
use self::reactor::*;
use self::reactor::streaming_sockets::*;
use self::reactor::streaming_server_listener_sockets::*;
use self::reactor::streaming_server_listener_sockets::access_control::*;
use self::terminate::*;
use self::tls::*;
use ::context::context::*;
use ::context::stack::*;
use ::cpu_affinity::*;
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
use ::rustls::*;
use ::rustls::internal::pemfile::*;
use ::rustls::TLSError::FailedToGetCurrentTime;
use ::rustls::TLSError::NoCertificatesPresented;
use ::rustls::TLSError::WebPKIError;
use ::std::any::Any;
use ::std::cell::Cell;
use ::std::cell::UnsafeCell;
use ::std::collections::HashSet;
use ::std::error;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fs::File;
use ::std::marker::PhantomData;
use ::std::mem::forget;
use ::std::mem::ManuallyDrop;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::mem::transmute_copy;
use ::std::mem::uninitialized;
use ::std::ops::BitAnd;
use ::std::ops::Deref;
use ::std::os::unix::io::AsRawFd;
use ::std::os::unix::io::FromRawFd;
use ::std::os::unix::io::RawFd;
use ::std::panic::PanicInfo;
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


/// Implementations of the `Arena` trait.
pub mod arena;


/// Structures to ease use of coroutines (continuations) used for handling sockets and the like.
pub mod coroutine;


/// Implementations of the `Arenas` trait.
pub mod arenas;


/// Implementations of the `Ractor` trait.
pub mod reactor;


/// Implementations of the `Terminate` trait.
pub mod terminate;


/// Supporting logic for TLS.
pub mod tls;


include!("EventPoll.rs");
include!("EventPollRegistrationError.rs");
include!("EventPollToken.rs");
include!("FileDescriptorKind.rs");
include!("loop_or_await_or_error.rs");
include!("Unused.rs");

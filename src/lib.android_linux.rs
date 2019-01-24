// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


extern crate coroutine;
extern crate cpu_affinity;
extern crate file_descriptors;
extern crate libc;
#[macro_use] extern crate likely;
extern crate lock_free_multi_producer_single_consumer_ring_buffer;
extern crate rustls_extra;
extern crate terminate;
extern crate treebitmap;


use self::arena::*;
use self::arenas::*;
use self::reactor::*;
use self::reactor::distribution::*;
use self::reactor::streaming_sockets::*;
use self::reactor::streaming_sockets::streams::*;
use self::reactor::streaming_sockets::stream_factories::*;
use self::reactor::streaming_server_listener_sockets::access_control::*;
use ::coroutine::*;
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
use ::file_descriptors::socket::syscall::*;
use ::file_descriptors::signalfd::*;
use ::file_descriptors::signalfd::syscall::signalfd_siginfo;
use ::file_descriptors::timerfd::TimerFileDescriptor;
use ::file_descriptors::terminal::TerminalFileDescriptor;
use ::libc::gid_t;
use ::libc::sched_getcpu;
use ::libc::uid_t;
use ::lock_free_multi_producer_single_consumer_ring_buffer::*;
use ::rustls_extra::*;
use ::std::any::TypeId;
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
use ::std::mem::transmute_copy;
use ::std::mem::uninitialized;
use ::std::net::Ipv4Addr;
use ::std::net::Ipv6Addr;
use ::std::net::SocketAddrV4;
use ::std::net::SocketAddrV6;
use ::std::ops::BitAnd;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
use ::std::ops::Index;
use ::std::ops::IndexMut;
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
use ::std::ptr::null_mut;
use ::std::ptr::read;
use ::std::ptr::write;
use ::std::raw::TraitObject;
use ::std::rc::Rc;
use ::std::sync::Arc;
use ::terminate::*;
use ::treebitmap::IpLookupTable;


/// Implementations of the `Arena` trait.
pub mod arena;


/// Implementations of the `Arenas` trait.
pub mod arenas;


/// Structures to produce, consume and handle dynamically typed messages passed between threads using a ring buffer.
pub mod message_dispatch;


/// Implementations of the `Ractor` trait.
#[macro_use] pub mod reactor;


/// Implementations of the `Terminate` trait.
pub mod terminate;


include!("file_descriptor_kind_dispatch.rs");


include!("EventPoll.rs");
include!("EventPollRegistrationError.rs");
include!("EventPollToken.rs");
include!("FileDescriptorKind.rs");

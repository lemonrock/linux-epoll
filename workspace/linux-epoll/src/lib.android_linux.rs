// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


extern crate arrayvec;
pub extern crate cpu_affinity;
extern crate dpdk_unix;
pub extern crate file_descriptors;
pub extern crate hashbrown;
pub extern crate libc;
#[macro_use] extern crate likely;
pub extern crate message_dispatch;
#[macro_use] extern crate serde_derive;
extern crate terminate;


use self::arena::*;
use self::reactor::*;
use ::arrayvec::ArrayVec;
use ::cpu_affinity::*;
use ::dpdk_unix::android_linux::*;
use ::dpdk_unix::hyper_thread::*;
use ::dpdk_unix::scheduling::*;
use ::dpdk_unix::signals::*;
use ::file_descriptors::*;
use ::file_descriptors::epoll::*;
use ::file_descriptors::epoll::syscall::*;
use ::file_descriptors::socket::*;
use ::libc::SIGHUP;
use ::libc::SIGINT;
use ::libc::SIGQUIT;
use ::libc::sigset_t;
use ::libc::SIGTERM;
use ::message_dispatch::*;
pub use ::message_dispatch::erased_boxed_functions::CompressedTypeIdentifier;
use ::std::cell::Cell;
use ::std::cell::UnsafeCell;
use ::std::collections::BTreeSet;
use ::hashbrown::HashMap;
use ::hashbrown::HashSet;
use ::std::any::TypeId;
use ::std::error;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::mem::ManuallyDrop;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::mem::zeroed;
use ::std::ops::Deref;
use ::std::os::unix::io::AsRawFd;
use ::std::panic::*;
use ::std::ptr::drop_in_place;
use ::std::ptr::NonNull;
use ::std::sync::Arc;
use ::std::thread::Builder;
use ::std::thread::JoinHandle;
pub use ::terminate::*;


/// Implementations of the `Arena` trait.
pub mod arena;


/// Implementations of the `Reactor` trait.
#[macro_use] pub mod reactor;


include!("EventPoll.rs");
include!("EventPollRegister.rs");
include!("EventPollRegistrationError.rs");
include!("EventPollToken.rs");
include!("JoinHandles.rs");
include!("Process.rs");
include!("ProcessConfiguration.rs");
include!("Registration.rs");

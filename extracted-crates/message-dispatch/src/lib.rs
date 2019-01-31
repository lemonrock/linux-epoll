// This file is part of message-dispatch. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT. No part of message-dispatch, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of message-dispatch. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(integer_atomics)]
#![feature(raw)]


//! #message-dispatch
//! 
//! This provides dynamic dispatch support for messages of different types and sizes sent from one thread to another (or back to the same thread) without the need to use trait objects.
//!
//! As such, the only cost involved in dispatch is the cost of an indirect call.
//!
//! It could even be used to send messages across POSIX message queues if so desired.


extern crate arrayvec;
extern crate cpu_affinity;
extern crate libc;
extern crate file_descriptors;
extern crate errno;
#[macro_use] extern crate likely;
extern crate terminate;


use self::magic_ring_buffer::*;
use self::erased_boxed_functions::*;
use self::virtual_method_tables::*;
use ::arrayvec::ArrayVec;
use ::cpu_affinity::*;
use ::std::collections::HashMap;
use ::std::cell::UnsafeCell;
use ::std::any::Any;
use ::std::any::TypeId;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::mem::align_of;
use ::std::mem::forget;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::ops::Deref;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
use ::std::ptr::write;
use ::std::raw::TraitObject;
use ::std::sync::Arc;
use ::terminate::Terminate;


/// A magic ring buffer suitable for backing a queue.
pub mod magic_ring_buffer;


/// Erased, boxed functions can be used as generic message dispatchers.
pub mod erased_boxed_functions;


/// Various wrappers around virtual method tables (vtables) which allow for them to be tagged.
///
/// A tagged pointer to a vtable allows one to mix multiple `dyn Trait` (fat pointers), using the tag to differentiated the trait type.
#[allow(dead_code)]
mod virtual_method_tables;


include!("Dequeue.rs");
include!("Enqueue.rs");
include!("Message.rs");
include!("MessageHandlersRegistration.rs");
include!("MessageHeader.rs");
include!("PerThreadQueueSubscriber.rs");
include!("round_up_to_alignment.rs");
include!("Queue.rs");
include!("QueuePerThreadQueuesPublisher.rs");
include!("VariablySized.rs");

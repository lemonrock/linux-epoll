// This file is part of message-dispatch. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT. No part of message-dispatch, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of message-dispatch. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/message-dispatch/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(extern_types)]


//! #message-dispatch
//! 
//! This provides dynamic dispatch support for messages of different types and sizes sent from one thread to another (or back to the same thread) without the need to use trait objects.
//!
//! As such, the only cost involved in dispatch is the cost of an indirect call.
//!
//! It could even be used to send messages across POSIX message queues if so desired.


extern crate arrayvec;
extern crate terminate;


use self::erased_boxed_functions::*;
use ::arrayvec::ArrayVec;
use ::std::collections::HashMap;
use ::std::any::TypeId;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::mem::align_of;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
use ::std::ptr::write;
use ::terminate::Terminate;


/// Erased, boxed functions can be used as generic message dispatchers.
pub mod erased_boxed_functions;


/// Various wrappers around virtual method tables (vtables) which allow for them to be tagged.
///
/// A tagged pointer to a vtable allows one to mix multiple `dyn Trait` (fat pointers), using the tag to differentiated the trait type.
pub mod virtual_method_tables;


include!("Message.rs");
include!("MessageHeader.rs");
include!("VariablySized.rs");

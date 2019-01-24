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


use ::std::collections::HashMap;
use ::std::any::TypeId;
use ::std::mem::transmute;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;


include!("BoxedFunctionPointer.rs");
include!("CallArguments.rs");
include!("VariablySized.rs");
include!("ImmutableTypeErasedBoxedFunction.rs");
include!("ImmutableTypeErasedBoxedFunctionMap.rs");

// TODO: Probably not needed but still very interesting.
include!("TaggedVirtualMethodTablePointer.rs");
include!("VirtualMethodTablePointer.rs");



// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![cfg_attr(any(target_os = "android", target_os = "linux"), feature(core_intrinsics))]
#![cfg_attr(any(target_os = "android", target_os = "linux"), feature(extern_types))]
#![cfg_attr(any(target_os = "android", target_os = "linux"), feature(nll))]
#![cfg_attr(any(target_os = "android", target_os = "linux"), feature(read_initializer))]


//! #linux-epoll
//! 
//! This is a rust library.
//!
//!
//! ## Conditional Compilation Features
//!
//! ### `assume-file-descriptors-are-never-duplicated`
//!
//! Enable the feature `assume-file-descriptors-are-never-duplicated` to eliminate syscalls to remove file descriptors from an epoll instance before `close`-ing them.
//! This feature is disabled by default.
//!
//! If file descriptors are duplicated (eg in FFI code) then it is possible that closing a file descriptor may result in an epoll instance still generating events for it, causing a memory leak (or worse; it rapidly gets confusing).
//! Ordinarily, modern applications do not need to duplicate file descriptors that are attached to an epoll instance and so enabling this feature produces a performance win.


#[cfg(any(target_os = "android", target_os = "linux"))] include!("lib.android_linux.rs");

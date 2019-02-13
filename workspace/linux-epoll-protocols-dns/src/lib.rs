// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(core_intrinsics)]
#![feature(extern_types)]


//! #linux-epoll-protocols-dns
//!
//! A simple implementation of a secure DNS client.


extern crate arrayvec;
extern crate hashbrown;
#[macro_use] extern crate likely;


use self::extended_dns::*;
use self::extended_dns::dnssec::*;
use self::name::*;
use self::resource_data::*;
use self::resource_data::location::*;
use self::resource_data::ssh_fingerprint::*;
use self::resource_data::start_of_authority::*;
use self::resource_data::text_strings::*;
use self::resource_data::tls_dane::*;
use ::arrayvec::ArrayVec;
use ::hashbrown::HashMap;
use ::std::cell::Cell;
use ::std::cmp::min;
use ::std::marker::PhantomData;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::net::Ipv4Addr;
use ::std::net::Ipv6Addr;
use ::std::ptr::NonNull;
use ::std::slice::from_raw_parts;


/// Extended DNS (EDNS).
pub(crate) mod extended_dns;


/// DNS name handling.
pub mod name;


/// DNS resource data (`RDATA`) related types.
pub mod resource_data;


include!("DnsProtocolError.rs");
include!("Message.rs");
include!("MessageBitField1.rs");
include!("MessageBitField2.rs");
include!("MessageBody.rs");
include!("MessageHeader.rs");
include!("MessageIdentifer.rs");
include!("MessageOpcode.rs");
include!("MessageResponseCode.rs");
include!("MessageType.rs");
include!("ResourceData.rs");
include!("ResourceRecord.rs");
include!("ResourceRecordClass.rs");
include!("ResourceRecordFooter.rs");
include!("ResourceRecordType.rs");
include!("QueryClass.rs");
include!("QuerySectionEntry.rs");
include!("QuerySectionEntryFooter.rs");
include!("TcpMessage.rs");
include!("TimeToLiveInSeconds.rs");

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(core_intrinsics)]
#![feature(proc_macro_hygiene)]


//! #linux-epoll-protocols-dns
//!
//! A simple implementation of a secure DNS client.


extern crate arrayvec;
extern crate either;
extern crate hashbrown;
#[macro_use] extern crate likely;
extern crate phf;
#[macro_use] extern crate phf_macros;
extern crate time;


use self::DnsProtocolError::*;
use self::character_strings::*;
use self::extended_dns::*;
use self::message::*;
use self::name::*;
use self::resource_data::*;
use self::resource_data::certificate::*;
use self::resource_data::certification_authority_authorization::*;
use self::resource_data::dhcid::*;
use self::resource_data::dns_based_authentication_of_named_entities::*;
use self::resource_data::dnssec::*;
use self::resource_data::host_identity_protocol::*;
use self::resource_data::identifier_locator_network_protocol::*;
use self::resource_data::ipsec::*;
use self::resource_data::location::*;
use self::resource_data::naming_authority_pointer::*;
use self::resource_data::ssh_fingerprint::*;
use self::resource_data::start_of_authority::*;
use self::response_parsing::*;
use self::support::*;
use ::arrayvec::ArrayVec;
use ::either::*;
use ::hashbrown::HashMap;
use ::phf::Map;
use ::std::cmp::min;
use ::std::cmp::Ordering;
use ::std::cmp::PartialOrd;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::marker::PhantomData;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::mem::zeroed;
use ::std::net::Ipv4Addr;
use ::std::net::Ipv6Addr;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::write_bytes;
use ::std::slice::from_raw_parts;
use ::std::slice::from_raw_parts_mut;
use ::time::get_time;
use ::time::Timespec;


/// Character strings support.
pub mod character_strings;


pub(crate) mod extended_dns;


pub(crate) mod message;


/// DNS name handling.
pub mod name;


/// DNS resource data (`RDATA`) related types.
pub mod resource_data;


pub(crate) mod response_parsing;


pub(crate) mod support;


include!("DnsProtocolError.rs");
include!("MessageIdentifier.rs");
include!("SerialNumber.rs");
include!("TimeInSeconds.rs");
include!("TimeToLiveInSeconds.rs");

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


use super::*;


/// `TLSA` and `SMIME` record support.
pub mod dns_based_authentication_of_named_entities;


/// DNSSEC.
pub mod dnssec;


/// `LOC` record support.
pub mod location;


/// `NAPTR` record support.
pub mod naming_authority_pointer;


/// `IPSECKEY` (and potentially the obsolete `KEY`) record support.
pub mod ipsec;


/// `SSHFP` record support.
pub mod ssh_fingerprint;


/// `SOA` record support.
pub mod start_of_authority;


include!("HostInformation.rs");
include!("KeyExchange.rs");
include!("MailExchange.rs");
include!("OpenPgpRfc4880TransferablePublicKey.rs");
include!("ResourceRecordVisitor.rs");
include!("Service.rs");
include!("SliceExt.rs");

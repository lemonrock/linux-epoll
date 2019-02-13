// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


use super::*;


/// `LOC` record support.
pub mod location;


/// `SSHFP` record support.
pub mod ssh_fingerprint;


/// `SOA` record support.
pub mod start_of_authority;


/// `TXT` record support.
pub mod text_strings;


/// `TLSA` record support.
pub mod tls_dane;


include!("MailExchange.rs");
include!("OpenPgpRfc4880TransferablePublicKey.rs");
include!("ResourceRecordVisitor.rs");
include!("Service.rs");

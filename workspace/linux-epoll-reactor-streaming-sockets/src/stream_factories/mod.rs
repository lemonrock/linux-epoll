// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


use super::*;


/// HTTP CONNECT proxy wrapping factories.
pub mod http_connect;


/// SOCKS4a proxy wrapping factories.
pub mod socks4a;


/// SOCKS5 proxy wrapping factories.
pub mod socks5;


include!("send_packet.rs");
include!("StreamFactory.rs");
include!("TlsClientStreamFactory.rs");
include!("TlsServerStreamFactory.rs");
include!("UnencryptedStreamFactory.rs");

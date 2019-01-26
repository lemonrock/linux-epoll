// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


use super::*;
#[cfg(target_arch = "x86")] use ::std::arch::x86::_mm_lddqu_si128;
#[cfg(target_arch = "x86")] use ::std::arch::x86::__m128i;
#[cfg(target_arch = "x86_64")] use ::std::arch::x86_64::_mm_lddqu_si128;
#[cfg(target_arch = "x86_64")] use ::std::arch::x86_64::__m128i;


include!("AtomicPairOfU64.rs");
include!("CompareExchangeQueueMemoryPointers.rs");
include!("PairOfU64.rs");

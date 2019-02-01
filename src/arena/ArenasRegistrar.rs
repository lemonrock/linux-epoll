// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Registrar for arenas.
pub trait ArenasRegistrar
{
	/// Register an arena.
	///
	/// It is not permissible to register multiple arenas for the same type of `R`.
	fn register_arena<A: Arena<R>, R: Reactor>(&mut self, arena: A) -> CompressedTypeIdentifier;
}
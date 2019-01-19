// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


macro_rules! write_loop_or_await_or_error
{
	($io_error: ident, $yielder: ident, $complete_error_kind_wrapping_io_error: ident) =>
	{
		{
			use self::ErrorKind::*;

			match $io_error.kind()
			{
				Interrupted => continue,

				WouldBlock => await_further_input_or_output_to_become_available!($yielder),

				_ => return Err(CompleteError::$complete_error_kind_wrapping_io_error($io_error))
			}
		}
	}
}

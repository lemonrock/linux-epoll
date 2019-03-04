// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


macro_rules! guard_next_label_starts_at_pointer
{
	($current_label_starts_at_pointer: ident, $length: ident, $maximum_for_end_of_name_pointer: ident) =>
	{
		{
			let next_label_starts_at_pointer = $current_label_starts_at_pointer + LabelKind::LabelKindSize + $length;
			if unlikely!(next_label_starts_at_pointer > $maximum_for_end_of_name_pointer)
			{
				return Err(LabelLengthOverflows)
			}

			next_label_starts_at_pointer
		}
	}
}

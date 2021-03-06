// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


macro_rules! parse_and_register_bytes_label
{
	($label: ident, $current_label_starts_at_pointer: ident, $maximum_for_end_of_name_pointer: ident, $labels_register_reference: ident, $number_of_labels: ident, $name_length: ident) =>
	{
		{
			let length = $label.length();

			debug_assert!(length <= MaximumSizeOfLabelExcludingTrailingPeriod, "label `{}` exceeds MaximumSizeOfLabelExcludingTrailingPeriod `{}`", length, MaximumSizeOfLabelExcludingTrailingPeriod);

			let label_bytes_length_including_trailing_period = (length as u8) + SizeOfTrailingPeriod;
			$labels_register_reference.push(($current_label_starts_at_pointer, label_bytes_length_including_trailing_period));

			parse_bytes_label!(length, $current_label_starts_at_pointer, $maximum_for_end_of_name_pointer, $number_of_labels, $name_length)
		}
	}
}

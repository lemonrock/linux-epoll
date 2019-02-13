// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ParsedLabel<'a>
{
	label_bytes: LabelBytes<'a>,
	next: Cell<Option<NonNull<Self>>>,
}

impl<'a> ParsedLabel<'a>
{
	const Fake: Self = Self
	{
		label_bytes: b"",
		next: Cell::new(None),
	};

	#[inline(always)]
	fn new(upTo63Bytes: &UpTo63Bytes, length: usize) -> Self
	{
		Self
		{
			label_bytes: unsafe { from_raw_parts(upTo63Bytes as *const UpTo63Bytes as *const u8, length) },
			next: Cell::new(None),
		}
	}

	#[inline(always)]
	fn label_bytes(&self) -> LabelBytes<'a>
	{
		self.label_bytes
	}

	#[inline(always)]
	fn this_label_starts_at_pointer(&self) -> usize
	{
		self.label_bytes.as_ptr() as usize
	}

	#[inline(always)]
	fn next_label_starts_at_pointer(&self) -> usize
	{
		self.this_label_starts_at_pointer() + self.label_bytes.len()
	}

	#[inline(always)]
	fn is_terminal_root_label(&self) -> bool
	{
		self.label_bytes.is_empty()
	}

	#[inline(always)]
	fn next(&mut self) -> Option<NonNull<Self>>
	{
		self.next.get()
	}

	#[inline(always)]
	fn set_next(&mut self, next_parsed_label: &Self)
	{
		self.next.set(Some(unsafe { NonNull::new_unchecked(next_parsed_label as *const Self as *mut Self) }))
	}
}


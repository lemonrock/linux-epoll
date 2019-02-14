// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// A naming authority pointer with a regular expression.
pub struct NamingAuthorityPointerWithRegularExpression<'a>
{
	/// Header.
	pub header: NamingAuthorityPointerHeader<'a>,

	/// Regular expression, up to 255 bytes long.
	///
	/// Will never be empty (0 bytes long).
	pub regular_expression: &'a [u8],
}

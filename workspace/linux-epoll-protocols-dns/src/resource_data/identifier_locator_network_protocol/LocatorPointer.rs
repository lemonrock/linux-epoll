// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Represents a locator pointer along with its preference.
///
/// Used in a `LP` record.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocatorPointer<'a>
{
	/// Indicates the owner name's relative preference for record among other records associated with this owner name.
	///
	/// Lower preference values are preferred over higher preference values.
	pub preference: u16,

	/// `Name`.
	///
	/// Must not be the same as the `Name` of the resource record it is associated with (this is not validated before being passed to `ResourceRecordVisitor.LP()`).
	pub domain_name: WithoutCompressionParsedNameIterator<'a>,
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Represents a Certificate Authority Authorization (CAA) record.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CertificateAuthorityAuthorization<'a>
{
	/// Is issuer critical?
	pub is_issuer_critical: bool,

	/// Property tag.
	///
	pub property_tag: CertificateAuthorityAuthorizationPropertyTag,

	/// Property value.
	///
	/// * `AuthorizationEntryByDomain and `AuthorizationEntryByWildcardDomain` have a subformat of name-value pairs.
	/// * `ReportIncidentByIodefReport` is a URL.
	/// * `AuthorizedEMailContactForDomainValidation` is a email address.
	pub property_value: &'a [u8],
}

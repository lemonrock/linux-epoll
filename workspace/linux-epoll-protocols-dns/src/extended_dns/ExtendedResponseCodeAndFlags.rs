// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
pub(crate) struct ExtendedResponseCodeAndFlags(pub(crate) [u8; 4]);

impl ExtendedResponseCodeAndFlags
{
	const DnsSecFlagUpper: u8 = 0b1000_0000;

	const KnownExtendedFlagsUpper: u8 = Self::DnsSecFlagUpper;

	#[inline(always)]
	pub(crate) fn extended_response_code_upper_8_bits(&self) -> u8
	{
		self.0.value(0)
	}

	#[inline(always)]
	pub(crate) fn version(&self) -> Result<ExtendedDnsVersion, DnsProtocolError>
	{
		let version = self.0.value(1);
		if likely!(version == 0x00)
		{
			Ok(ExtendedDnsVersion::Version0)
		}
		else
		{
			Err(UnsupportedExtendedDnsVersion(version))
		}
	}

	#[inline(always)]
	pub(crate) fn dnssec_ok(&self) -> bool
	{
		self.upper_flag_bits() & Self::DnsSecFlagUpper != 0
	}

	#[inline(always)]
	pub(crate) fn z(&self) -> Result<(), DnsProtocolError>
	{
		if likely!(self.upper_flag_bits() | !Self::KnownExtendedFlagsUpper == 0 && self.lower_flag_bits() == 0)
		{
			Ok(())
		}
		else
		{
			Err(ExtendedDnsZFieldNotZero)
		}
	}

	#[inline(always)]
	fn upper_flag_bits(&self) -> u8
	{
		self.0.value(2)
	}

	#[inline(always)]
	fn lower_flag_bits(&self) -> u8
	{
		self.0.value(3)
	}
}

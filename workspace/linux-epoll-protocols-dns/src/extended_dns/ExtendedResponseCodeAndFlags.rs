// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.
//


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
pub(crate) struct ExtendedResponseCodeAndFlags(pub(crate) [u8; 4]);

impl ExtendedResponseCodeAndFlags
{
	#[inline(always)]
	fn extended_response_code_upper_8_bits(&self) -> u8
	{
		unsafe { *self.0.get_unchecked(0) }
	}

	#[inline(always)]
	fn set_extended_response_code_upper_8_bits(&mut self, upper_8_bits: u8)
	{
		unsafe { *self.0.get_unchecked_mut(0) = upper_8_bits}
	}

	#[inline(always)]
	fn version(&self) -> Result<ExtendedDnsVersion, DnsProtocolError>
	{
		let version = unsafe { *self.0.get_unchecked(1) };
		if likely!(version == 0x00)
		{
			Ok(ExtendedDnsVersion::Version0)
		}
		else
		{
			Err(DnsProtocolError::UnsupportedExtendedDnsVersion(version))
		}
	}

	#[inline(always)]
	fn set_version(&mut self, extended_dns_version: ExtendedDnsVersion)
	{
		unsafe { *self.0.get_unchecked_mut(1) = extended_dns_version as u8 }
	}

	#[inline(always)]
	fn dnssec_ok(&self) -> bool
	{
		(unsafe { *self.0.get_unchecked(2) }) & 0b1000_0000 != 0
	}

	#[inline(always)]
	fn z(&self) -> Result<(), DnsProtocolError>
	{
		if likely!((unsafe { *self.0.get_unchecked(2) }) | 0b0111_1111 == 0 && (unsafe { *self.0.get_unchecked(3) }) == 0)
		{
			Ok(())
		}
		else
		{
			Err(DnsProtocolError::ExtendedDnsZFieldNotZero)
		}
	}
}

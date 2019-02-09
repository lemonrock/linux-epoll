// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Data required to establish a SOCKS4a client CONNECT.
#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub struct Socks4aConnect
{
	/// Address or Host Name.
	pub address_or_host_name: InternetProtocolVersion4AddressOrHostName,

	/// Destination Port.
	pub destination_port: u16,

	/// User Identifier.
	pub user_identifier: CString,
}

impl Socks4aConnect
{
	const SizeOfVN: usize = 1;

	const SizeOfCD: usize = 1;

	const SizeOfDSTPORT: usize = 2;

	const SizeOfDSTIP: usize = 4;

	const Version: u8 = 4;

	const PacketMaximumSize: usize = 384;

	#[inline(always)]
	pub(crate) fn send_request<'yielder, SD: SocketData>(&self, unencrypted_stream: UnencryptedStream<'yielder, SD>) -> Result<UnencryptedStream<'yielder, SD>, CompleteError>
	{
		let (buffer, packet_length) = self.write_packet(&mut buffer).map_err(|additional_length_required| CompleteError::InvalidData(format!("Either the user identifer or host name were so large that they could not be written; an extra {:?} bytes over {:?} would be required", additional_length_required, buffer.len())))?;
		send_packet(unencrypted_stream, &buffer[0 .. packet_length])
	}

	#[inline(always)]
	fn write_packet(&self) -> Result<([u8; Self::PacketMaximumSize], usize), usize>
	{
		let mut buffer: [u8; Self::PacketMaximumSize] = unsafe { uninitialized() };

		let user_identifier_length = self.user_identifier.to_bytes_with_nul().len();
		let minimum_buffer_length = Self::SizeOfVN + Self::SizeOfCD + Self::SizeOfDSTPORT + Self::SizeOfDSTIP + user_identifier_length;

		use self::InternetProtocolVersion4AddressOrHostName::*;

		let packet_length = match self.address_or_host_name
		{
			InternetProtocolVersion4Address(ref destination_internet_protocol_version_4_address) =>
			{
				if unlikely!(buffer.len() < minimum_buffer_length)
				{
					return Err(minimum_buffer_length - buffer.len())
				}

				unsafe
				{
					self.write_first_three_fields_unchecked(buffer, command);
					Self::write_DSTIP_unchecked(buffer, destination_internet_protocol_version_4_address.octets());
					self.write_USERID_unchecked(buffer);
					minimum_buffer_length
				}
			}

			HostName(ref host_name) =>
			{
				let host_name_length = host_name.to_bytes_with_nul().len();

				let offset_to_host_name = minimum_buffer_length;
				let minimum_buffer_length = minimum_buffer_length + host_name_length;
				if unlikely!(buffer.len() < minimum_buffer_length)
				{
					return Err(minimum_buffer_length - buffer.len())
				}

				unsafe
				{
					self.write_first_three_fields_unchecked(buffer, command);
					Self::write_DSTIP_unchecked(buffer, [0, 0, 0, 1]);
					self.write_USERID_unchecked(buffer);

					Self::write_DOMAIN_unchecked(buffer, offset_to_host_name, host_name)
				}
			},
		};

		Ok((buffer, packet_length))
	}

	#[inline(always)]
	unsafe fn write_first_three_fields_unchecked(&self, buffer: &mut [u8], command: u8)
	{
		Self::write_VN_unchecked(buffer);
		Self::write_CD(buffer);
		self.write_DSTPORT_unchecked(buffer);
	}

	#[inline(always)]
	unsafe fn write_VN_unchecked(buffer: &mut [u8])
	{
		*buffer.get_unchecked_mut(0) = Self::Version;
	}

	#[inline(always)]
	unsafe fn write_CD_unchecked(buffer: &mut [u8], command: u8)
	{
		*buffer.get_unchecked_mut(Self::SizeOfVN) = command;
	}

	#[inline(always)]
	unsafe fn write_DSTPORT_unchecked(&self, buffer: &mut [u8])
	{
		// NOTE: Works on ARM 32-bit as DSTPORT is 2-byte (16-bit) aligned.
		*(buffer.get_unchecked_mut(Self::SizeOfVN + Self::SizeOfCD) as *mut u8 as *mut [u8; 2]) = self.destination_port.to_be_bytes();
	}

	#[inline(always)]
	unsafe fn write_DSTIP_unchecked(&self, buffer: &mut [u8], octets: [u8; 4])
	{
		// NOTE: Works on ARM 32-bit as DSTIP is 4-byte (32-bit) aligned.
		*(buffer.get_unchecked_mut(Self::SizeOfVN + Self::SizeOfCD + Self::SizeOfDSTPORT) as *mut u8 as *mut [u8; 4]) = octets;
	}

	#[inline(always)]
	unsafe fn write_USERID_unchecked(&self, buffer: &mut [u8])
	{
		let user_identifier = self.user_identifier.to_bytes_with_nul();
		let user_identifier_length = user_identifier.len();
		copy_nonoverlapping(user_identifier.as_ptr(), buffer.get_unchecked_mut(Self::SizeOfVN + Self::SizeOfCD + Self::SizeOfDSTPORT + Self::SizeOfDSTIP), user_identifier_length)
	}

	#[inline(always)]
	unsafe fn write_DOMAIN_unchecked(&self, buffer: &mut [u8], offset: usize, host_name: &CString) -> usize
	{
		let host_name = host_name.to_bytes_with_nul();
		let host_name_length = host_name.len();
		copy_nonoverlapping(user_identifier.as_ptr(), buffer.get_unchecked_mut(offset), host_name_length);
		offset + host_name_length
	}
}

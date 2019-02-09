// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Data required to establish a SOCKS5 client CONNECT.
#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub struct Socks5Connect
{
	/// Address or Host Name.
	pub address_or_host_name: InternetProtocolAddressOrHostName,

	/// Destination Port.
	pub destination_port: u16,
}

impl Socks5Connect
{
	const SizeOfVER: usize = 1;

	const SizeOfCMD: usize = 1;

	const SizeOfRSV: usize = 1;

	const SizeOfATYP: usize = 1;

	const MaximumSizeOfHostName: usize = 253;

	const MaximumSizeOfDSTADDR: usize = Self::MaximumSizeOfHostName + 1;

	const SizeOfDSTPORT: usize = 2;

	const Version: u8 = 5;

	const ConnectCommand: u8 = 0x01;

	const PacketMaximumSize: usize = Self::SizeOfVER + Self::SizeOfCMD + Self::SizeOfRSV + Self::SizeOfATYP + Self::MaximumSizeOfDSTADDR + Self::SizeOfDSTPORT;

	#[inline(always)]
	pub(crate) fn send_request<'yielder, SD: SocketData>(&self, unencrypted_stream: UnencryptedStream<'yielder, SD>) -> Result<UnencryptedStream<'yielder, SD>, CompleteError>
	{
		let (buffer, packet_length) = self.write_packet(&mut buffer)?;
		send_packet(unencrypted_stream, &buffer[0 .. packet_length])
	}

	#[inline(always)]
	fn write_packet(&self) -> Result<([u8; Self::PacketMaximumSize], usize), CompleteError>
	{
		let mut buffer: [u8; Self::PacketMaximumSize] = unsafe { uninitialized() };

		Self::write_VER(&mut buffer);
		Self::write_CMD(&mut buffer, Self::ConnectCommand);
		Self::write_RSV(&mut buffer);

		use self::InternetProtocolAddressOrHostName::*;
		use self::IpAddr::*;
		let dst_addr_length = match self.address_or_host_name
		{
			&InternetProtocolAddress(V4(ref address)) =>
			{
				const dst_addr_length: usize = 4;
				Self::write_ATYP(&mut buffer, 0x01);

				// NOTE: Works on ARM 32-bit as DST.ADDR is 4-byte (32-bit) aligned.
				unsafe { *(buffer.get_unchecked_mut(Self::SizeOfVER + Self::SizeOfCMD + Self::SizeOfRSV + Self::SizeOfATYP) as *mut u8 as *mut [u8; dst_addr_length]) = address.octets() };

				dst_addr_length
			}

			&InternetProtocolAddress(V6(ref address)) =>
			{
				const dst_addr_length: usize = 16;
				Self::write_ATYP(&mut buffer, 0x04);

				// NOTE: Works on ARM 32-bit as DST.ADDR is 4-byte (32-bit) aligned.
				unsafe { *(buffer.get_unchecked_mut(Self::SizeOfVER + Self::SizeOfCMD + Self::SizeOfRSV + Self::SizeOfATYP) as *mut u8 as *mut [u8; dst_addr_length]) = address.octets() };

				dst_addr_length
			}

			&HostName(ref host_name) =>
			{
				let number_of_octets_in_host_name = host_name.len();

				if unlikely!(number_of_octets_in_host_name == 0)
				{
					return Err(CompleteError::InvalidData("The host name is empty".to_string()))
				}

				if unlikely!(number_of_octets_in_host_name > Self::MaximumSizeOfHostName)
				{
					return Err(CompleteError::InvalidData("The host name exceeds 253 bytes, the maximum for a DNS fully qualifed domain name (FQDN)".to_string()))
				}

				Self::write_ATYP(&mut buffer, 0x03);

				unsafe
				{
					*(buffer.get_unchecked_mut(Self::SizeOfVER + Self::SizeOfCMD + Self::SizeOfRSV + Self::SizeOfATYP)) = number_of_octets_in_host_name as u8;
					copy_nonoverlapping(host_name.as_ptr(), buffer.get_unchecked_mut(Self::SizeOfVER + Self::SizeOfCMD + Self::SizeOfRSV + Self::SizeOfATYP + 1), number_of_octets_in_host_name);
				}
				number_of_octets_in_host_name + 1
			}
		};

		let packet_length = self.write_DSTPORT(&mut buffer, dst_addr_length);

		Ok((buffer, packet_length))
	}

	#[inline(always)]
	fn write_VER(buffer: &mut [u8])
	{
		unsafe { *buffer.get_unchecked_mut(0) = Self::Version };
	}

	#[inline(always)]
	fn write_CMD(buffer: &mut [u8], command: u8)
	{
		unsafe { *buffer.get_unchecked_mut(Self::SizeOfVER) = command };
	}

	#[inline(always)]
	fn write_RSV(buffer: &mut [u8])
	{
		unsafe { *buffer.get_unchecked_mut(Self::SizeOfVER + Self::SizeOfCMD) = 0x00 };
	}

	#[inline(always)]
	fn write_ATYP(buffer: &mut [u8], atyp: u8)
	{
		unsafe { *buffer.get_unchecked_mut(Self::SizeOfVER + Self::SizeOfCMD + Self::SizeOfRSV) = atyp };
	}

	#[inline(always)]
	fn write_DSTPORT(&self, buffer: &mut [u8], dst_addr_length: usize)
	{
		let octets = self.destination_port.to_be_bytes();

		let offset = Self::SizeOfVER + Self::SizeOfCMD + Self::SizeOfRSV + Self::SizeOfATYP + dst_addr_length;

		// NOTE: Uses copy_nonoverlapping as the DST.PORT field may not be 16-byte aligned and a so a write will fail with ?SIGBUS on ARM 32-bit.
		unsafe { copy_nonoverlapping(octets.as_ptr(), buffer.get_unchecked_mut(offset), Self::SizeOfDSTPORT) };

		let packet_length = offset + Self::SizeOfDSTPORT;

		packet_length
	}










}

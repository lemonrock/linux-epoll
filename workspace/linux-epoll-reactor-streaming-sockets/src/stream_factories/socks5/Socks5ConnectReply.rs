// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


const SizeOfVER: usize = 1;

const SizeOfREP: usize = 1;

const SizeOfRSV: usize = 1;

const SizeOfATYP: usize = 1;

const MaximumSizeOfHostName: usize = 253;

const MaximumSizeOfBNDADDR: usize = Self::MaximumSizeOfHostName + 1;

const SizeOfBNDPORT: usize = 2;

struct Socks5ConnectReply<'yielder, SD: SocketData>
{
	unencrypted_stream: UnencryptedStream<'yielder, SD>,
	small_reply_packet_buffer: [u8; SizeOfVER + SizeOfREP + SizeOfRSV + SizeOfATYP + MaximumSizeOfBNDADDR + SizeOfBNDPORT],
	bytes_read_so_far: usize,
}

impl<'yielder, SD: SocketData> Socks5ConnectReply<'yielder, SD>
{
	#[inline(always)]
	pub(crate) fn read_reply(unencrypted_stream: UnencryptedStream<'yielder, SD>) -> Result<(UnencryptedStream<'yielder, SD>, BoundSocketAddress), CompleteError>
	{
		use self::Socks5ProtocolFailureError::*;

		#[inline(always)]
		fn error(error: Socks5ProtocolFailureError) -> Result<(), CompleteError>
		{
			Err(CompleteError::ProtocolViolation(Box::new(error)))
		}

		const SizeOfVER: usize = 1;
		const SizeOfREP: usize = 1;
		const SizeOfRSV: usize = 1;
		const SizeOfATYP: usize = 1;
		const SizeOfBNDPORT: usize = 2;

		let mut this = Self
		{
			unencrypted_stream,
			small_reply_packet_buffer: unsafe { uninitialized() },
			bytes_read_so_far: 0,
		};

		this.read_reply_bytes()?;

		let version = unsafe { *this.small_reply_packet_buffer.get_unchecked(0) } ;
		if version != Socks5Connect::Version
		{
			return error(VersionInvalid(version))
		}

		if unlikely!(this.bytes_read_so_far == SizeOfVER)
		{
			this.read_reply_bytes()?;
		}

		let rep = unsafe { *this.small_reply_packet_buffer.get_unchecked(SizeOfVER) };
		match rep
		{
			0x00 => (),
			0x01 => return error(GeneralSocksServerFailure),
			0x02 => return error(ConnectionNotAllowedByRuleset),
			0x03 => return error(NetworkUnreachable),
			0x04 => return error(HostUnreachable),
			0x05 => return error(ConnectionRefused),
			0x06 => return error(TimeToLiveExpired),
			0x07 => return error(CommandNotSupported),
			0x08 => return error(AddressTypeNotSupported),
			_ => return error(UnassignedError(rep)),
		}

		if unlikely!(this.bytes_read_so_far == SizeOfVER + SizeOfREP)
		{
			this.read_reply_bytes()?;
		}

		let rsv = unsafe { *this.small_reply_packet_buffer.get_unchecked(SizeOfVER + SizeOfREP) };

		if unlikely!(rsv != 0x00)
		{
			return error(ReplyRsvFieldWasNotZero)
		}

		if unlikely!(this.bytes_read_so_far == SizeOfVER + SizeOfREP + SizeOfRSV)
		{
			this.read_reply_bytes()?;
		}

		let atyp = unsafe { *this.small_reply_packet_buffer.get_unchecked(SizeOfVER + SizeOfREP + SizeOfRSV) };

		const SizeOfFixedHeader: usize = SizeOfVER + SizeOfREP + SizeOfRSV + SizeOfATYP;

		use self::InternetProtocolAddressOrHostName::*;

		let (bind_address, dst_addr_length) = match atyp
		{
			0x01 =>
			{
				const dst_addr_length: usize = 4;

				while(unlikely!(this.bytes_read_so_far < SizeOfFixedHeader + dst_addr_length))
				{
					this.read_reply_bytes()?
				}

				// NOTE: Works on ARM 32-bit as BIND.ADDR is 4-byte (32-bit) aligned.
				let bind_address = InternetProtocolAddress(IpAddr::from(unsafe { *(this.small_reply_packet_buffer.get_unchecked(SizeOfFixedHeader) as *mut u8 as *mut [u8; dst_addr_length]) }));

				(bind_address, dst_addr_length)
			}

			0x04 =>
			{
				const dst_addr_length: usize = 16;

				while(unlikely!(self.bytes_read_so_far < SizeOfFixedHeader + dst_addr_length))
				{
					this.read_reply_bytes()?
				}

				// NOTE: Works on ARM 32-bit as BIND.ADDR is 4-byte (32-bit) aligned.
				let bind_address = InternetProtocolAddress(IpAddr::from(unsafe { *(this.small_reply_packet_buffer.get_unchecked(SizeOfFixedHeader) as *mut u8 as *mut [u8; dst_addr_length]) }));

				(bind_address, dst_addr_length)
			}

			0x03 =>
			{
				const SizeOfNumberOfOctetsOfHostName: usize = 1;

				if unlikely!(this.bytes_read_so_far == SizeOfFixedHeader)
				{
					this.read_reply_bytes()?;
				}

				let number_of_octets_in_host_name = (unsafe { *this.small_reply_packet_buffer.get_unchecked(SizeOfFixedHeader) });

				if unlikely!(number_of_octets_in_host_name == 0)
				{
					return error(HostNameInReplyWasEmpty)
				}

				if unlikely!(number_of_octets_in_host_name > MaximumSizeOfHostName as u8)
				{
					return error(HostNameInReplyWasTooLarge(number_of_octets_in_host_name))
				}

				let number_of_octets_in_host_name = number_of_octets_in_host_name as usize;

				let dst_addr_length = SizeOfNumberOfOctetsOfHostName + number_of_octets_in_host_name;

				while(unlikely!(self.bytes_read_so_far < SizeOfFixedHeader + dst_addr_length))
				{
					this.read_reply_bytes()?
				}

				let mut host_name = ArrayVec::new();
				unsafe
				{
					copy_nonoverlapping(this.small_reply_packet_buffer.get_unchecked(SizeOfFixedHeader + SizeOfNumberOfOctetsOfHostName), host_name.as_mut_ptr(), number_of_octets_in_host_name);
					host_name.set_len(number_of_octets_in_host_name)
				}
				let bind_address = HostName(host_name);

				(bind_address, dst_addr_length)
			}

			_ => return error(ReplyContainedAnUnrecognisedAddressType(atyp)),
		};

		let offset_to_bndport = SizeOfFixedHeader + dst_addr_length;
		while(unlikely!(self.bytes_read_so_far < offset_to_bndport + SizeOfBNDPORT))
		{
			this.read_reply_bytes()?
		}

		// Uses copy_nonoverlapping as might not be 16-bit aligned on 32-bit ARM platforms, which will cause a ?SIGBUS.

		let mut bind_port_octets: [u8; SizeOfBNDPORT] = unsafe { uninitialized() };
		unsafe { copy_nonoverlapping(this.small_reply_packet_buffer.get_unchecked(offset_to_bndport), (&mut bind_port_octets) as *mut [u8; SizeOfBNDPORT] as *mut u8, SizeOfBNDPORT) }
		let bind_port = u16::from_be_bytes(bind_port_octets);

		let bound_socket = BoundSocketAddress
		{
			address: bind_address,
			port: bind_port,
		};

		Ok((this.unencrypted_stream, bound_socket))
	}

	#[inline(always)]
	fn read_reply_bytes(&mut self) -> Result<(), CompleteError>
	{
		let mut buffer = &mut self.small_reply_packet_buffer[self.bytes_read_so_far..];
		debug_assert_ne!(buffer.len(), 0, "should never try a read of zero bytes");

		let bytes_read = unencrypted_stream.read_data(buffer).map_err(|io_error| CompleteError::SocketRead(io_error))?;
		debug_assert_ne!(bytes_read, 0, "A read of zero should not be possible unless the buffer is zero");

		self.bytes_read_so_far += bytes_read;

		Ok(())
	}
}

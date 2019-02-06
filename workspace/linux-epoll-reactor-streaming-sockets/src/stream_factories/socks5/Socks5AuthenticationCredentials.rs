// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Socks5AuthenticationCredentials
{
	credentials: ArrayVec<[Socks5AuthenticationCredential; Socks5AuthenticationCredential::NumberOfMembers]>,
	presence: HashMap<Socks5AuthenticationCredentialCode, usize>,
}

impl Default for Socks5AuthenticationCredentials
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			credentials: ArrayVec::new(),
			presence: HashSet::with_capacity(Socks5AuthenticationCredential::NumberOfMembers),
		}
	}
}

impl Deref for Socks5AuthenticationCredentials
{
	type Target = ArrayVec<[Socks5AuthenticationCredential; Socks5AuthenticationCredential::NumberOfMembers]>;

	fn deref(&self) -> &Self::Target
	{
		&self.credentials
	}
}

impl Socks5AuthenticationCredentials
{
	const Version: u8 = 5;

	const PacketMaximumSize: usize = SizeOfVersion + SizeOfNumberOfMembers + Socks5AuthenticationCredential::NumberOfMembers;

	#[inline(always)]
	pub(crate) fn send_request<'yielder, SD: SocketData>(&self, unencrypted_stream: UnencryptedStream<'yielder, SD>) -> Result<UnencryptedStream<'yielder, SD>, CompleteError>
	{
		let (buffer, packet_length) = self.write_packet();
		send_packet(unencrypted_stream, &buffer[0 .. packet_length])
	}

	#[inline(always)]
	fn write_packet(&self) -> ([u8; Self:: VersionIdentifierMaximumMessageSize], usize)
	{
		let socks5_authentication_credentials_length = self.len();

		const SizeOfVersion: usize = 1;
		const SizeOfNumberOfMembers: usize = 1;
		const OffsetToCredentialCodes: usize = SizeOfVersion + SizeOfNumberOfMembers;

		let mut authentication_credentials_buffer: [u8; VersionIdentifierMaximumMessageSize] = unsafe { uninitialized() };

		*(unsafe { authentication_credentials_buffer.get_unchecked_mut(0) }) = Self::Version;

		*(unsafe { authentication_credentials_buffer.get_unchecked_mut(1) }) = socks5_authentication_credentials_length as u8;

		for index in 0 .. socks5_authentication_credentials_length
		{
			let code = (unsafe { socks5_authentication_credentials.get_unchecked(index) }).to_code() as u8;
			*(unsafe { authentication_credentials_buffer.get_unchecked_mut(OffsetToCredentialCodes + index) }) = code;
		}

		(authentication_credentials_buffer, OffsetToCredentialCodes + socks5_authentication_credentials_length)
	}

	/// Create for none.
	#[inline(always)]
	pub fn none() -> Self
	{
		let mut this = Self::default();
		this.add(Socks5AuthenticationCredential::None);
		this
	}

	/// Create from user name and password.
	#[inline(always)]
	pub fn user_name_and_password(user_name: ArrayVec<[u8; 255]>, password: ArrayVec<[u8; 255]>) -> Self
	{
		let mut this = Self::default();
		this.add(Socks5AuthenticationCredential::UserNameAndPassword { user_name, password });
		this
	}

	/// Add `credential`.
	///
	/// Only one credential of each type of member of `Socks5AuthenticationCredential` is permitted.
	///
	/// Returns `true` if `credential` not previously added (this will cause the old credential to be dropped and this credential to be put last).
	#[inline(always)]
	pub fn add(&mut self, credential: Socks5AuthenticationCredential) -> bool
	{
		let code = credential.to_code();

		use self::Entry::*;

		match self.credentials.entry(code)
		{
			Vacant(vacant) =>
			{
				self.credentials.push(credential);
				vacant.insert(self.credentials.len());
				false
			}

			Occupied(occupied) =>
			{
				let index = *occupied.get();
				self.credentials.remove(index);
				self.credentials.push(credential);
				*occupied.get_mut() = self.credentials.len();
				true
			}
		}
	}

	#[inline(always)]
	fn get_from_code(&self, code: Socks5AuthenticationCredentialCode) -> Option<&Socks5AuthenticationCredential>
	{
		self.presence.get(&code).map(|index| unsafe { self.credentials.get_unchecked(*index) } )
	}
}


// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// At this point there is no support for the GSS-API methods; however, GSS-API is only really effective when used with Kerberos.
///
/// A potential source of draft RFCs for other authentication methods is listed as part of [Dante](https://www.inet.no/dante/doc/).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Socks5AuthenticationCredential
{
	/// No authentication credentials.
	None,

	/// An user name and password, sent in the clear (as CLEARTEXT)!
	///
	/// The `user_name` and `password` are not permitted to be empty (zero length).
	UserNameAndPassword
	{
		/// User name; 1 to 255 bytes long inclusive.
		user_name: ArrayVec<[u8; 255]>,

		/// Password; 1 to 255 bytes long inclusive.
		password: ArrayVec<[u8; 255]>,
	}
}

impl Socks5AuthenticationCredential
{
	const NumberOfMembers: usize = 2;

	#[inline(always)]
	pub(crate) fn to_code(&self) -> Socks5AuthenticationCredentialCode
	{
		use self::Socks5AuthenticationCredential::*;

		match self
		{
			&None => Socks5AuthenticationCredentialCode::None,

			&UserNameAndPassword { .. } => Socks5AuthenticationCredentialCode::UserNameAndPassword,
		}
	}

	#[inline(always)]
	pub(crate) fn negotiate<'yielder, SD: SocketData>(&self, unencrypted_stream: UnencryptedStream<'yielder, SD>) -> Result<UnencryptedStream<'yielder, SD>, CompleteError>
	{
		use self::Socks5AuthenticationCredential::*;
		use self::Socks5ProtocolFailureError::*;

		let unencrypted_stream = match self
		{
			&None => unencrypted_stream,

			&UserNameAndPassword { ref user_name, ref password } => negotiate_user_name_and_password(unencrypted_stream, &user_name, &password)?,
		};

		Ok(unencrypted_stream)
	}

	#[inline(always)]
	fn negotiate_user_name_and_password<'yielder, SD: SocketData>(unencrypted_stream: UnencryptedStream<'yielder, SD>, user_name: &[u8], password: &[u8]) -> Result<UnencryptedStream<'yielder, SD>, CompleteError>
	{
		if unlikely!(user_name.is_empty())
		{
			return error(EmptyUserName)
		}

		if unlikely!(password.is_empty())
		{
			return error(EmptyPassword)
		}

		const SizeOfVersion: usize = 1;
		const SizeOfUserLength: usize = 1;
		const SizeOfUserMaximum: usize = 255;
		const SizeOfPasswordLength: usize = 1;
		const SizeOfPasswordMaximum: usize = 255;

		const MaximumBufferSize: usize = SizeOfVersion + SizeOfUserLength + SizeOfUserMaximum + SizeOfPasswordLength + SizeOfPasswordMaximum;

		#[inline(always)]
		fn write_packet(user_name: &[u8], password: &[u8]) -> ([u8; MaximumBufferSize], usize)
		{
			let mut buffer: [u8; MaximumBufferSize] = unsafe { uninitialized() };

			let user_name_length = user_name.len();
			let password_length = password.len();

			*(unsafe { buffer.get_unchecked_mut(0) }) = Self::Version;
			*(unsafe { buffer.get_unchecked_mut(SizeOfVersion) }) = user_name_length as u8;
			unsafe { copy_nonoverlapping(user_name.as_ptr(), buffer.get_unchecked_mut(SizeOfVersion + SizeOfUserLength), user_name_length) };

			*(unsafe { buffer.get_unchecked_mut(SizeOfVersion + SizeOfUserLength + user_name_length) }) = password_length as u8;
			unsafe { copy_nonoverlapping(password.as_ptr(), buffer.get_unchecked_mut(SizeOfVersion + SizeOfUserLength + user_name_length + SizeOfPasswordLength), password_length) };

			(buffer, SizeOfVersion + SizeOfUserLength + user_name_length + SizeOfPasswordLength + password_length)
		}

		let (buffer, packet_length) = write_packet(user_name, password);
		send_packet(unencrypted_stream, &buffer[0 .. packet_length])?;

		Socks5UserNamePasswordReply::read_reply(unencrypted_stream)?
	}
}

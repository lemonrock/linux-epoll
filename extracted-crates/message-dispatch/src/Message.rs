// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Represents a message to be enqueued to a buffer.
#[repr(C)]
pub struct Message
{
	message_header: MessageHeader,
	padding_to_align_message_contents_and_message_contents_and_padding_to_align_next_message_header: VariablySized,
}

impl Debug for Message
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "Message {{ compressed_type_identifier: {:?}, message_contents: _ }}", self.message_header)
	}
}

impl Message
{
	/// Enqueues a new message into the buffer if there is space available.
	///
	/// Assumes the buffer is correctly aligned for a MessageHeader.
	#[inline(always)]
	pub fn enqueue<MessageContents>(buffer: &mut [u8], compressed_type_identifier: CompressedTypeIdentifier, message_contents_constructor: impl FnOnce(NonNull<MessageContents>)) -> Result<usize, usize>
	{
		let buffer_pointer = buffer.as_ptr() as usize;
		debug_assert_eq!(buffer_pointer % align_of::<MessageHeader>(), 0, "buffer is not correctly aligned for a MessageHeader");

		let first_possible_pointer_to_message_contents = buffer_pointer + size_of::<MessageHeader>();

		let message_pointer = round_up_to_alignment::<MessageContents>(first_possible_pointer_to_message_contents);
		let message_contents_size = size_of::<MessageContents>();

		let end_of_message_pointer = message_pointer + message_contents_size;

		let next_message_pointer = round_up_to_alignment::<MessageHeader>(end_of_message_pointer);

		let total_message_size_including_message_header = next_message_pointer - buffer_pointer;
		debug_assert!(total_message_size_including_message_header < ::std::u16::MAX as usize, "total_message_size_including_message_header exceeds 65,535 bytes (MessageHeader users an u16 to store the message size)");
		if total_message_size_including_message_header > buffer.len()
		{
			return Err(total_message_size_including_message_header)
		}

		let number_of_bytes_padding_to_align_message_contents = message_pointer - first_possible_pointer_to_message_contents;
		debug_assert!(number_of_bytes_padding_to_align_message_contents < ::std::u8::MAX as usize, "number_of_bytes_padding_to_align_message_contents exceeds 255 bytes (MessageHeader users an u8 to store the alignment padding)");


		unsafe
		{
			let message_header = &mut * (buffer_pointer as *mut MessageHeader);
			write(&mut message_header.compressed_type_identifier, compressed_type_identifier);
			write(&mut message_header.number_of_bytes_padding_to_align_message_contents, number_of_bytes_padding_to_align_message_contents as u8);
			write(&mut message_header.message_contents_size, message_contents_size as u16);
		}

		message_contents_constructor(unsafe { NonNull::new_unchecked(message_pointer as *mut MessageContents) });

		Ok(total_message_size_including_message_header)
	}

	/// Stops on the first fatal error result (`Err(E)`) or if Terminate has become true.
	///
	/// Otherwise returns the number of bytes consumed.
	pub fn process_messages_in_buffer<E>(buffer: &mut [u8], message_handlers: &mut MutableTypeErasedBoxedFunctionCompressedMap<Result<(), E>>, terminate: &impl Terminate) -> Result<usize, E>
	{
		let original_length = buffer.len();
		let (mut remaining_buffer_pointer, mut remaining_buffer_length) = (buffer.as_mut_ptr(), original_length);

		while remaining_buffer_length >= MessageHeader::Size && terminate.should_continue()
		{
			match Self::process_next_message_in_buffer::<Result<(), E>>(remaining_buffer_pointer, remaining_buffer_length, message_handlers)
			{
				None => break,

				Some((outcome, total_message_size_including_message_header)) =>
				{
					outcome?;

					remaining_buffer_pointer = unsafe { remaining_buffer_pointer.add(total_message_size_including_message_header) };
					remaining_buffer_length -= total_message_size_including_message_header;
				}
			}
		}

		let consumed = original_length - remaining_buffer_length;
		Ok(consumed)
	}

	/// The handlers in `message_handlers` are responsible for logically dropping the message; it is recommended that messages and their constituent fields do not implement `Drop`.
	///
	/// Returns an outcome `R` and a `message_size` if successful.
	/// Returns `None` if the buffer is too small.
	#[inline(always)]
	fn process_next_message_in_buffer<R>(buffer_pointer: *mut u8, buffer_length: usize, message_handlers: &mut MutableTypeErasedBoxedFunctionCompressedMap<R>) -> Option<(R, usize)>
	{
		debug_assert!(buffer_length >= MessageHeader::Size, "Check buffer size before calling");

		debug_assert_eq!(buffer_pointer as usize % align_of::<MessageHeader>(), 0, "Buffer is not aligned on a MessageHeader");

		let message_header = unsafe { &mut * (buffer_pointer as *mut MessageHeader) };

		let total_message_size_including_message_header = message_header.total_message_size_including_message_header();

		if buffer_length < total_message_size_including_message_header
		{
			return None
		}

		let compressed_type_identifier = message_header.compressed_type_identifier;
		let arguments = message_header.message_contents();

		let outcome = message_handlers.call(compressed_type_identifier, arguments);
		Some((outcome, total_message_size_including_message_header))
	}
}

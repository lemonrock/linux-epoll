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
	#[inline(always)]
	pub(crate) fn enqueue<MessageContents>(magic_ring_buffer: &MagicRingBuffer, compressed_type_identifier: CompressedTypeIdentifier, message_contents_constructor: impl FnOnce(NonNull<MessageContents>))
	{
		let largest_possible_total_message_size_including_message_header = Self::largest_possible_total_message_size_including_message_header::<MessageContents>();

		magic_ring_buffer.write_some_data(largest_possible_total_message_size_including_message_header, |buffer_sized_as_for_maximum_possible|
		{
			Self::enqueue_once_buffer_allocated::<MessageContents, _>(buffer_sized_as_for_maximum_possible, compressed_type_identifier, message_contents_constructor)
		})
	}

	/// NOTE: In Rust, alignment is *always* a positive power of two (ie never zero), is 1 for packed structs and is never less than the struct's size, either.
	/// An empty struct by default has an alignment of 1 but it too can have any legal alignment.
	///
	/// Should, after monomorphization and compiler optimization, become nothing more than a constant value.
	#[inline(always)]
	pub(crate) fn largest_possible_total_message_size_including_message_header<MessageContents>() -> usize
	{
		const MessageHeaderSize: usize = size_of::<MessageHeader>();
		const MessageHeaderAlignment: usize = align_of::<MessageHeader>();
		let MessageContentsSize = size_of::<MessageContents>();
		let MessageContentsAlignment = align_of::<MessageContents>();

		if MessageContentsAlignment > MessageHeaderAlignment
		{
			let maximum_padding_after_message_header_but_before_message_contents = MessageContentsAlignment - MessageHeaderAlignment;
			let largest_possible_total_message_size_including_message_header = MessageHeaderSize + maximum_padding_after_message_header_but_before_message_contents + MessageContentsSize;

			largest_possible_total_message_size_including_message_header
		}
		else
		{
			let takes_up_no_space = MessageContentsSize == 0;

			if takes_up_no_space
			{
				MessageHeaderSize
			}
			else
			{
				MessageHeaderSize * 2
			}
		}
	}

	/// Enqueues a new message into the buffer_sized_as_for_maximum_possible if there is space available.
	///
	/// Assumes the `buffer_sized_as_for_maximum_possible` is correctly aligned for a `MessageHeader`.
	#[inline(always)]
	pub(crate) fn enqueue_once_buffer_allocated<MessageContents, MessageContentsConstructor: FnOnce(NonNull<MessageContents>)>(buffer_sized_as_for_maximum_possible: &mut [u8], compressed_type_identifier: CompressedTypeIdentifier, message_contents_constructor: MessageContentsConstructor)
	{
		let total_message_size_including_message_header_padding_to_align_before_message_contents_and_padding_to_align_after = buffer_sized_as_for_maximum_possible.len();
		debug_assert_eq!(Self::largest_possible_total_message_size_including_message_header::<MessageContents>(), total_message_size_including_message_header_padding_to_align_before_message_contents_and_padding_to_align_after, "buffer_sized_as_for_maximum_possible is not");
		debug_assert!(total_message_size_including_message_header_padding_to_align_before_message_contents_and_padding_to_align_after < ::std::u16::MAX as usize, "message is far too large");

		let buffer_pointer = buffer_sized_as_for_maximum_possible.as_ptr() as usize;
		debug_assert_eq!(buffer_pointer % align_of::<MessageHeader>(), 0, "buffer_sized_as_for_maximum_possible is not correctly aligned for a MessageHeader");

		const MessageHeaderSize: usize = size_of::<MessageHeader>();
		const MessageHeaderAlignment: usize = align_of::<MessageHeader>();
		let MessageContentsSize = size_of::<MessageContents>();
		let MessageContentsAlignment = align_of::<MessageContents>();

		let (message_contents_pointer, number_of_bytes_padding_to_align_message_contents) = if MessageContentsAlignment > MessageHeaderAlignment
		{
			let first_possible_message_contents_pointer = buffer_pointer + MessageHeaderSize;
			let message_contents_pointer = round_up_to_alignment::<MessageContents>(first_possible_message_contents_pointer);
			(message_contents_pointer, message_contents_pointer - first_possible_message_contents_pointer)
		}
		else
		{
			let takes_up_no_space = MessageContentsSize == 0;

			let actual_total_message_size = if takes_up_no_space
			{
				MessageHeaderSize
			}
			else
			{
				MessageHeaderSize * 2
			};
			(actual_total_message_size, 0)
		};

		unsafe
		{
			let message_header = &mut * (buffer_pointer as *mut MessageHeader);
			write(&mut message_header.compressed_type_identifier, compressed_type_identifier);
			write(&mut message_header.number_of_bytes_padding_to_align_message_contents, number_of_bytes_padding_to_align_message_contents as u8); // TODO: Could be stored as `SQRT(MessageContentsAlignment)`, thus allowing more alignments, at the cost of more processing when dequeued.
			write(&mut message_header.total_message_size_including_message_header_padding_to_align_before_message_contents_and_padding_to_align_after, total_message_size_including_message_header_padding_to_align_before_message_contents_and_padding_to_align_after as u16); // TODO: Could be stored less message header size and leading padding, thus allowing a little more data.
		}

		message_contents_constructor(unsafe { NonNull::new_unchecked(message_contents_pointer as *mut MessageContents) })
	}

	/// The handlers in `message_handlers` are responsible for logically dropping the message; it is recommended that messages and their constituent fields do not implement `Drop`.
	///
	/// Returns `(next_message_pointer, R)`.
	#[inline(always)]
	fn process_next_message_in_buffer<R>(buffer: &mut [u8], message_handlers: &mut MutableTypeErasedBoxedFunctionCompressedMap<R>) -> (usize, R)
	{
		const MessageHeaderSize: usize = size_of::<MessageHeader>();
		const MessageHeaderAlignment: usize = align_of::<MessageHeader>();

		let buffer_pointer = buffer.as_mut_ptr() as usize;
		let buffer_length = buffer.len();
		debug_assert_eq!(buffer_pointer % MessageHeaderAlignment, 0, "Buffer is not aligned on a MessageHeader");
		debug_assert!(buffer_length >= MessageHeaderSize, "Buffer is too small to contain a MessageHeader");

		let message_header = unsafe { &mut * (buffer_pointer as *mut MessageHeader) };

		let total_message_size_including_message_header_padding_to_align_before_message_contents_and_padding_to_align_after = message_header.total_message_size_including_message_header_padding_to_align_before_message_contents_and_padding_to_align_after();

		debug_assert!(buffer_length >= total_message_size_including_message_header_padding_to_align_before_message_contents_and_padding_to_align_after, "Buffer is too small to contain the Message");
		debug_assert_eq!((buffer_pointer + total_message_size_including_message_header_padding_to_align_before_message_contents_and_padding_to_align_after) % MessageHeaderAlignment, 0, "Message is not aligned such that the next MessageHeader is aligned");

		let compressed_type_identifier = message_header.compressed_type_identifier;
		let arguments = message_header.message_contents();

		let outcome = message_handlers.call(compressed_type_identifier, arguments);
		(total_message_size_including_message_header_padding_to_align_before_message_contents_and_padding_to_align_after, outcome)
	}
}

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


impl DistributedFileDescriptorMessage
{
	// TODO: May not be safely aligned...
	#[inline(always)]
	pub(crate) fn message_contents(&mut self, virtual_tables: &DistributedFileDescriptorMessageContentsVirtualMethodTablesPointerCompression) -> (RawFd, &mut dyn DistributedFileDescriptorMessageContents)
	{
		let data = &mut self.message_contents as *mut VariablySized as *mut ();
		let vtable = virtual_tables.get_virtual_method_table(self.index).to_ptr();

		let fat_pointer: &mut dyn DistributedFileDescriptorMessageContents = unsafe { transmute(TraitObject { data, vtable }) };

		(self.raw_file_descriptor, fat_pointer)
	}
}



struct ProducerMessage
{
	message_virtual_method_table: TaggedVirtualMethodTablePointer,
	message_contents: VariablySized,
}







trait ProducerMessageContents<I: 'static, P: 'static>: Any + 'static
{
	unsafe fn initialize(self: *mut Self, arguments: I);

	/// After this is called, `drop()` is ***NEVER*** called.
	unsafe fn process(&self, arguments: P);
}

impl<SF: StreamFactory<SD>, SU: StreamUser<SF::S>, SD: SocketData> ProducerMessageContents<SF, SU, SD> for StreamingSocketProducerMessageContents<SF, SU, SD>
{
	#[inline(always)]
	unsafe fn initialize(self: *mut Self, arguments: I)
	{

	}

	#[inline(always)]
	unsafe fn process(&mut self, arguments: P)
	{

	}
}

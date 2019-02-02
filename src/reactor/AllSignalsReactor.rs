// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// This object forces all signals to be handled using epoll.
#[derive(Debug)]
pub struct AllSignalsReactor<SH: SignalHandler>
{
	signal_file_descriptor: SignalFileDescriptor,
	signal_handler: SH,
}

impl<SH: SignalHandler> Reactor for AllSignalsReactor<SH>
{
	type FileDescriptor = SignalFileDescriptor;

	type RegistrationData = SH;

	// (arena: &A, compressed_type_identifier: CompressedTypeIdentifier) are really things that should belong to the event_poll

	/// Starts blocking signals at this point.
	#[inline(always)]
	fn do_initial_input_and_output_and_register_with_epoll_if_necesssary<A: Arena<Self>, T: Terminate>(event_poll: &EventPoll<T>, arena: &A, reactor_compressed_type_identifier: CompressedTypeIdentifier, registration_data: Self::RegistrationData) -> Result<(), EventPollRegistrationError>
	{
		let (signal_file_descriptor, _signal_mask) = SignalFileDescriptor::new_with_filled_signal_mask()?;

		event_poll.register::<A, Self, _>(arena, reactor_compressed_type_identifier, signal_file_descriptor, EPollAddFlags::EdgeTriggeredInput, |uninitialized_this, signal_file_descriptor|
		{
			unsafe
			{
				((&mut uninitialized_this.signal_file_descriptor) as *mut Self::FileDescriptor).write(signal_file_descriptor)
				((&mut uninitialized_this.signal_handler) as *mut SH).write(registration_data)
			}
			Ok(())
		})
	}

	fn react(&mut self, event_flags: EPollEventFlags, terminate: &impl Terminate) -> Result<bool, String>
	{
		debug_assert_eq!(event_flags, EPollEventFlags::Input, "flags contained a flag other than `Input`");

		let mut signals: [signalfd_siginfo; 32] = unsafe { uninitialized() };

		while terminate.should_continue()
		{
			use self::StructReadError::*;

			match self.signal_file_descriptor.read(&mut signals)
			{
				Err(WouldBlock) => break,

				Err(Cancelled) => panic!("Signal file descriptor was cancelled"),

				Err(Interrupted) => panic!("EINTR should not occur for read() et al when using a signalfd and blocking all signals on a thread"),

				Ok(signals) => for signal in signals
				{
					if terminate.should_continue()
					{
						if let Err(_) = signal.handle_signal(&self.0)
						{
							return Err(format!("Could not handle signal"))
						}
					}
				},
			}
		}

		Ok(false)
	}
}

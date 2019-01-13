// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


extern crate file_descriptors;
#[macro_use] extern crate likely;


use self::arenas::*;
use ::file_descriptors::character_device::CharacterDeviceFileDescriptor;
use ::file_descriptors::epoll::EPollFileDescriptor;
use ::file_descriptors::eventfd::EventFileDescriptor;
use ::file_descriptors::fanotify::FanotifyFileDescriptor;
use ::file_descriptors::inotify::InotifyFileDescriptor;
use ::file_descriptors::pipes_and_fifos::ReceivePipeFileDescriptor;
use ::file_descriptors::pipes_and_fifos::SendPipeFileDescriptor;
use ::file_descriptors::posix_message_queues::ReceivePosixMessageQueueFileDescriptor;
use ::file_descriptors::posix_message_queues::SendPosixMessageQueueFileDescriptor;
use ::file_descriptors::posix_message_queues::SendAndReceivePosixMessageQueueFileDescriptor;
use ::file_descriptors::socket::*;
use ::file_descriptors::signalfd::SignalFileDescriptor;
use ::file_descriptors::timerfd::TimerFileDescriptor;
use ::file_descriptors::terminal::TerminalFileDescriptor;
use ::std::cell::Cell;
use ::std::cell::UnsafeCell;
use ::std::collections::HashSet;
use ::std::mem::forget;
use ::std::mem::ManuallyDrop;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::ops::BitAnd;
use ::std::os::unix::io::FromRawFd;
use ::std::os::unix::io::RawFd;
use ::std::ptr::drop_in_place;
use ::std::ptr::NonNull;


/// Implementations of the `Arena` trait.
pub mod arenas;


include!("Arenas.rs");
include!("ArenaIndex.rs");
include!("SimpleArenas.rs");
include!("EventPollToken.rs");
include!("FileDescriptorKind.rs");
include!("FileDescriptorKindDispatch.rs");
include!("UsesFileDescriptor.rs");
include!("Unused.rs");

// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.







/*
	A rust fat pointer for a Box<dyn Trait> (or dyn Trait) consists of the following:-

		- a pointer to the actual allocated memory.
		- a pointer to a vtable;

	We want to use our own allocator if we're going to start smuggling such pointers around. It'll need to be thread safe.

	Presumably vtables are compiler-created and statically allocated. We can probably hack some sort of static-on-first-use thing does Box::new(dyn Trait), transmutes it, extracts the vtable pointer and shoves it into a global static.

	We can then allocate memory from an arena - which now need to be thread safe. Or we actually allocate it from the file descriptor queue going for distribution.

	The consumer reads the message off the queue
		- gets the file descriptor
		- gets the vtable pointer
		- has access to the actual bytes
			- vtable contains a method that effectively does 'size_of()'
			- we can memcpy this into the thread-local arena.

		- instead of a vtable pointer, we're currently passing an index
			- and use a pointer -> function pointers -> execute

			- a vtable approach is
				pointer to vtable -> select a function pointer -> execute


	You can only make object-safe traits into trait objects. Some complex rules govern all the properties that make a trait object safe, but in practice, only two rules are relevant. A trait is object safe if all the methods defined in the trait have the following properties:

    The return type isn’t Self.
    There are no generic type parameters.
	(There are no associated const, and all methods must take self)



	Alignment is interesting - look at https://github.com/archshift/dynstack/blob/master/src/lib.rs (push())


	For a message queue, alignment is not crucial, although good alignment will improve performance.

	We can compress vtable pointers down to an index if so desired, in exchange for a look-up in the consumer (probably worth doing).






	Multiple file descriptors problems
		- We own all the file descriptors, eg a pair to be used for a tcp proxy, or a socket, pipe and file fd (for efficiency of file reads)
			- We close one file descriptor, we need to close all file descriptors

		- We own one file descriptor, the other is shared, eg a socket and a shared pipe FD (as a file cache) or Posix MQ FD, etc.
			- We close ours; we may need to close the other


		- One is epoll, one is file
			- file will never block, but could be rather slow for a large read of 100s Mb
				- cache content in several pipe FDs and use tee()

		Problem: close and pending epoll events
			- we close ours in response to an epoll event to us - no problem
			- we close the other file descriptor - there may be pending events for it in the same epoll results loop

		Problem: TLS TCP proxy - two sockets linked back-to-back, both work via TLS (eg a QUIC -> TLS or TLS -> QUIC proxy)
			- if we have the same closure registered for 2 epoll events, when we yield from the closure, we could enter for IO being available on either; we may be waiting on IO for socket 1, but enter with IO for socket 2
				- current design does not tell us the file descriptor
			- we need to register the same closure for 2 sockets, or 2 different closures, in which case, we'll need a buffer shared by both (referenced via Rc<>).
				- if 2 different closures, we need to know that we failed to create the second closure because the 2nd connection 'completed', otherwise we're very stuck.
			- a shared buffer design is not efficient for a classic SOCKS proxy, but, since the initial connection isn't encrypted, a classic SOCKS proxy is probably a bad idea.

		Problem: We need to get an answer from a service - eg DNS, LDAP or a database - before carrying on
			- we yield, but there is already IO available so epoll will never cause us to re-enter
			- this is very much like a thread doing sync / wait


*/

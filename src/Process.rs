// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Represents a process about to start.
#[derive(Debug)]
pub struct Process<T: Terminate, R: Registration>
{
	process_configuration: ProcessConfiguration,
	terminate: Arc<T>,
	queue_per_threads_publisher: QueuePerThreadQueuesPublisher<(), String>,
	registration: Arc<R>,
	valid_logical_cores_for_the_current_process: LogicalCores,
}

impl<T: Terminate, R: Registration> Process<T, R>
{
	/// Create a new instance.
	pub fn new(process_configuration: ProcessConfiguration, registration: impl Registration) -> Self
	{
		// TODO: We should really be looking to use isolated CPU cores (isolcpus kernel setting).
		let valid_logical_cores_for_the_current_process = LogicalCores::valid_logical_cores_for_the_current_process();

		Self
		{
			process_configuration,
			terminate: SimpleTerminate::new(),
			queue_per_threads_publisher: QueuePerThreadQueuesPublisher::allocate(&valid_logical_cores_for_the_current_process),
			registration: Arc::new(registration),
			valid_logical_cores_for_the_current_process,
		}
	}

	/// Execute and return an exit number for `main()`.
	pub fn execute(self) -> i32
	{
		let terminate = SimpleTerminate::new();

		let load_kernel_modules = || {};
		let uses_enhanced_intel_speedstep_technology = false;
		let additional_kernel_command_line_validations = || {};
		process_common_configuration.execute(load_kernel_modules, uses_enhanced_intel_speedstep_technology, additional_kernel_command_line_validations, |_, _, _|
		{
			// TODO: InterruptRequest::force_all_interrupt_requests_to_just_these_hyper_threads(online_shared_hyper_threads, self.proc_path());
			let mut join_handles = Vec::with_capacity(logical_cores.len());

			for logical_core_identifier in self.valid_logical_cores_for_the_current_process.iter()
			{
				let logical_core_identifier = *logical_core_identifier;
				match self.spawn(logical_core_identifier)
				{
					Err(()) => break,
					Ok(join_handle) => join_handles.push(join_handle),
				}
			}

			ProcessCommonConfiguration::lock_down_security();
			ProcessCommonConfiguration::lock_down_raw_network_and_other_input_and_output();
			Scheduler::Idle.set_for_current_thread();
			// TODO: Set this thread's logical core affinities (probably same as shared with linux)
			ProcessCommonConfiguration::lock_down_thread_nice_value_setting();

			// TODO: Join handles aren't enough. We need to monitor signals, too, as we are the main thread.

			for join_handle in join_handles.into_iter()
			{
				if let Err(panic_information) = join_handle.join()
				{
					self.terminate.begin_termination_due_to_panic(panic_information)
				}
			}

			if self.terminate.terminated_due_to_panic_or_irrecoverable_error()
			{
				Err(())
			}
			else
			{
				Ok(())
			}
		})
	}

	/// Spawns a new thread.
	///
	/// If spawning a new thread failed, tells terminate to begin termination and returns an `Err(())`.
	#[inline(always)]
	fn spawn(&self, logical_core_identifier: LogicalCoreIdentifier) -> Result<JoinHandle<()>, ()>
	{
		let terminate = self.terminate.clone();
		let scheduler = self.process_configuration.per_thread_scheduler;
		let queue_per_threads_publisher = self.queue_per_threads_publisher.clone();
		let registration = self.registration.clone();

		let thread_initialization_outcome = self.new_builder(logical_core_identifier).spawn(move ||
		{
			block_all_signals_on_current_thread();

			ProcessCommonConfiguration::lock_down_security();
			ProcessCommonConfiguration::lock_down_raw_network_and_other_input_and_output();

			if let Err(could_not) = LogicalCores::set_current_thread_affinity_for_only_logical_core(logical_core_identifier)
			{
				terminate.begin_termination_due_to_irrecoverable_error(&could_not);
				return
			}

			if let Err(explanation) = scheduler.set_for_current_thread()
			{
				terminate.begin_termination_due_to_irrecoverable_error(format!("Could not set current thread priority: `{}`", explanation));
				return
			}

			ProcessCommonConfiguration::lock_down_thread_nice_value_setting();

			let event_poll = EventPoll::new(Arenas::default());
			registration.register_all_arenas(&mut event_poll);
			registration.register_any_reactors(&event_poll);
			let per_thread_subscriber = PerThreadQueueSubscriber::new(queue_per_threads_publisher, &terminate, registration, &event_poll);

			while terminate.should_continue()
			{
				if let Err(explanation) = event_poll.event_loop(terminate)
				{
					terminate.begin_termination_due_to_irrecoverable_error(&explanation);
					return
				}

				if let Err(explanation) = per_thread_subscriber.receive_and_handle_messages(())
				{
					terminate.begin_termination_due_to_irrecoverable_error(&explanation);
					return
				}
			}
		});

		thread_initialization_outcome.map_err(|io_error|
		{
			terminate.begin_termination_due_to_irrecoverable_error(&io_error);
			()
		})
	}

	#[inline(always)]
	fn new_builder(&self, logical_core_identifier: LogicalCoreIdentifier) -> Builder
	{
		let name = format!("LogicalCore-{:?}", logical_core_identifier);
		Builder::new().stack_size(self.process_configuration.per_thread_stack_size).name(name)
	}
}

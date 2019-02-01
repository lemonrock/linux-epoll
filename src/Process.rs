// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Represents a process about to start.
#[derive(Debug)]
pub struct Process<T: Terminate, R: Registration>
{
	process_configuration: ProcessConfiguration,
	terminate: Arc<T>,
	registration: Arc<R>,
}

impl<T: Terminate, R: Registration> Process<T, R>
{
	/// Create a new instance.
	pub fn new(process_configuration: ProcessConfiguration, registration: impl Registration) -> Self
	{
		Self
		{
			process_configuration,
			terminate: SimpleTerminate::new(),
			registration: Arc::new(registration),
		}
	}

	/// Executes from main.
	///
	/// Never returns, eventually exits the program with an exit code.
	#[inline(always)]
	pub fn execute(self) -> !
	{
		let load_kernel_modules = || {};

		let uses_enhanced_intel_speedstep_technology = false;

		let additional_kernel_command_line_validations = || {};

		let main_loop = |online_shared_hyper_threads, online_isolated_hyper_threads, master_logical_core| self.execute_internal(online_shared_hyper_threads, online_isolated_hyper_threads, master_logical_core);

		let execution_result = self.process_configuration.process_common_configuration.execute(load_kernel_modules, uses_enhanced_intel_speedstep_technology, additional_kernel_command_line_validations, main_loop);

		let exit_code = match execution_result
		{
			Err(process_common_configuration_execution_error) =>
			{
				eprintln!("Serious Failure: `{:?}`", process_common_configuration_execution_error);
				ProcessCommonConfiguration::EX_SOFTWARE
			}
			Ok(exit_code) => exit_code,
		};

		exit(exit_code)
	}

	// TODO: online_shared_hyper_threads and online_isolated_hyper_threads should have any cores subtracted that aren't in LogicalCores::valid_logical_cores_for_the_current_process()
	// let valid_logical_cores_for_the_current_process = LogicalCores::valid_logical_cores_for_the_current_process();


	#[inline(always)]
	fn execute_internal(&self, online_shared_hyper_threads: BTreeSet<HyperThread>, online_isolated_hyper_threads: BTreeSet<HyperThread>, _master_logical_core: HyperThread) -> Result<Option<SignalNumber>, String>
	{
		InterruptRequest::force_all_interrupt_requests_to_just_these_hyper_threads(&online_isolated_hyper_threads, self.process_configuration.process_common_configuration.proc_path()).map_err(|io_result| format!("Failed to force all interrupt requests to cores used for event poll threads because of `{:?}`"))?;

		let this_master_thread_logical_core_affinity = LogicalCores::from(online_shared_hyper_threads);
		let event_poll_threads_logical_cores = LogicalCores::from(online_isolated_hyper_threads);

		let join_handles = self.spawn_event_poll_threads(event_poll_threads_logical_cores).map_err(|_| "Could not spawn event poll threads".to_string())?;

		let result = catch_unwind(AssertUnwindSafe(||
		{
			ProcessCommonConfiguration::lock_down_security();

			ProcessCommonConfiguration::lock_down_raw_network_and_other_input_and_output();

			Scheduler::Idle.set_for_current_thread();
			this_master_thread_logical_core_affinity.set_current_thread_affinity();
			ProcessCommonConfiguration::lock_down_thread_nice_value_setting();

			self.wait_on_signals(self.process_configuration.running_interactively())
		}));

		let signal_to_re_raise = match result
		{
			Err(panic_information) =>
			{
				self.terminate.begin_termination_due_to_panic(panic_information);
				None
			}

			Ok(signal_to_re_raise) =>
			{
				self.terminate.begin_termination();
				signal_to_re_raise
			}
		};

		drop(join_handles);

		if self.terminate.terminated_due_to_panic_or_irrecoverable_error()
		{
			Err("Terminated due to panic or irrecoverable error".to_string())
		}
		else
		{
			Ok(signal_to_re_raise)
		}
	}

	fn wait_on_signals(&self, running_interactively: bool) -> Option<SignalNumber>
	{
		let signals_to_wait_for = Self::block_all_signals_bar_a_few(running_interactively);
		let mut signal_to_re_raise = None;
		while self.terminate.should_continue()
		{
			use self::TimedSignalWait::*;

			match one_millisecond_timed_wait_for_signals(&signals_to_wait_for)
			{
				TimedOut => continue,

				OtherSignalInterrupted => unreachable!("The wait was interrupted by a signal handler; this handler was for a signal other than one of those in the set `signals_to_wait_for`"),

				Signalled(signal_number) =>
				{
					if running_interactively
					{
						signal_to_re_raise = match signal_number
						{
							SIGTERM => None,
							SIGHUP => None,
							SIGINT => Some(SIGINT),
							SIGQUIT => Some(SIGQUIT),

							_ => unreachable!("Blocked signal '{}' received", signal_number),
						};
					}
					else
					{
						signal_to_re_raise = match signal_number
						{
							SIGTERM => None,

							_ => unreachable!("Blocked signal '{}' received", signal_number),
						};
					}

					break
				}
			}
		}
		signal_to_re_raise
	}

	#[inline(always)]
	fn spawn_event_poll_threads<'terminate>(&'terminate self, event_poll_threads_logical_cores: LogicalCores) -> Result<JoinHandles<'terminate, T>, ()>
	{
		let queue_per_threads_publisher = QueuePerThreadQueuesPublisher::allocate(&event_poll_threads_logical_cores);
		let mut join_handles = JoinHandles::new(&event_poll_threads_logical_cores);

		for logical_core_identifier in event_poll_threads_logical_cores.iter()
		{
			let logical_core_identifier = *logical_core_identifier;
			match self.spawn(logical_core_identifier, &queue_per_threads_publisher)
			{
				Err(()) => return Err(()),
				Ok(join_handle) => join_handles.push(join_handle),
			}
		}

		Ok(join_handles)
	}

	/// Spawns a new thread.
	///
	/// If spawning a new thread failed, tells terminate to begin termination and returns an `Err(())`.
	#[inline(always)]
	fn spawn_event_poll_thread(&self, logical_core_identifier: LogicalCoreIdentifier, queue_per_threads_publisher: &QueuePerThreadQueuesPublisher<(), String>) -> Result<JoinHandle<()>, ()>
	{
		let terminate = self.terminate.clone();
		let scheduler = self.process_configuration.per_thread_scheduler;
		let queue_per_threads_publisher = queue_per_threads_publisher.clone();
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

	#[inline(always)]
	fn block_all_signals_bar_a_few(running_interactively: bool) -> sigset_t
	{
		let signals_to_accept = if running_interactively
		{
			hashset!
			{
				SIGTERM,
				SIGHUP,
				SIGINT,
				SIGQUIT,
			}
		}
		else
		{
			hashset!
			{
				SIGTERM,
				// NOTE: `SIGHUP` has been used conventionally to force a daemon to re-read its configuration; we're probably better off using `SIGUSR1` or `SIGUSR2`.
			}
		};
		block_all_signals_on_current_thread_bar(&signals_to_accept);
		hash_set_to_signal_set(&signals_to_accept)
	}
}

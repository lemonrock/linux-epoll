// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Represents a multi-thread, multi-core process that handles event polling and other events for any set of file descriptors on Linux.
///
/// Example usage:-
///
/// ```
/// let process = Process::defaultish(registration);
///
/// let exit_code = match process.execute()
/// {
/// 	Ok(()) => 0,
/// 	Err(process_common_configuration_execution_error) =>
/// 	{
/// 		const EX_SOFTWARE: i32 = 70;
/// 		eprintln!("Serious Failure: `{:?}`", process_common_configuration_execution_error);
/// 		EX_SOFTWARE
/// 	}
/// };
///
/// exit(exit_code);
/// ```
///
/// The `registration` variable contains all the logic needed to run the process.
#[derive(Debug)]
pub struct Process<T: Terminate + 'static, R: Registration>
{
	process_configuration: ProcessConfiguration,
	terminate: Arc<T>,
	registration: Arc<R>,
}

impl<R: Registration> Process<SimpleTerminate, R>
{
	/// Create a new instance using defaut settings.
	///
	/// After calling this call `execute()`.
	#[inline(always)]
	pub fn defaultish(registration: R) -> Self
	{
		Self::new(ProcessConfiguration::default(), SimpleTerminate::new(), registration)
	}
}

impl<T: Terminate, R: Registration> Process<T, R>
{
	/// Create a new instance.
	///
	/// After calling this call `execute()`.
	#[inline(always)]
	pub fn new(process_configuration: ProcessConfiguration, terminate: Arc<T>, registration: R) -> Self
	{
		Self
		{
			process_configuration,
			terminate,
			registration: Arc::new(registration),
		}
	}

	/// Executes from main.
	///
	/// Returns successfully or an error.
	/// Be aware that daemonization may have occurred, and so `eprintln!` may not go to standard error but syslog instead.
	#[inline(always)]
	pub fn execute(self) -> Result<(), ProcessCommonConfigurationExecutionError>
	{
		const uses_enhanced_intel_speedstep_technology: bool = false;

		const isolated_cpus_required: bool = false;

		self.process_configuration.process_common_configuration.execute
		(
			|| Ok(()),
			uses_enhanced_intel_speedstep_technology,
			isolated_cpus_required,
			|_| Ok(()),
			|_online_shared_hyper_threads_for_os, online_shared_hyper_threads_for_process, online_isolated_hyper_threads_for_process, _master_logical_core| self.execute_internal(online_shared_hyper_threads_for_process, online_isolated_hyper_threads_for_process)
		)
	}

	#[inline(always)]
	fn execute_internal(&self, online_shared_hyper_threads_for_process: BTreeSet<HyperThread>, online_isolated_hyper_threads_for_process: BTreeSet<HyperThread>) -> Result<Option<SignalNumber>, String>
	{
		InterruptRequest::force_all_interrupt_requests_to_just_these_hyper_threads(&online_isolated_hyper_threads_for_process, self.process_configuration.process_common_configuration.proc_path()).map_err(|io_result| format!("Failed to force all interrupt requests to cores used for event poll threads because of `{:?}`", io_result))?;

		let this_master_thread_logical_core_affinity = LogicalCores::from(online_shared_hyper_threads_for_process);
		let event_poll_threads_logical_cores = if online_isolated_hyper_threads_for_process.is_empty()
		{
			this_master_thread_logical_core_affinity.clone()
		}
		else
		{
			LogicalCores::from(online_isolated_hyper_threads_for_process)
		};

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
				self.terminate.begin_termination_due_to_irrecoverable_error(&panic_information);
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
		let queue_per_threads_publisher = QueuePerThreadQueuesPublisher::allocate(&event_poll_threads_logical_cores, self.process_configuration.per_thread_message_queue_size_in_bytes);
		let mut join_handles = JoinHandles::new(&event_poll_threads_logical_cores, self.terminate.deref());

		for logical_core_identifier in event_poll_threads_logical_cores.iter()
		{
			let logical_core_identifier = *logical_core_identifier;
			match self.spawn_event_poll_thread(logical_core_identifier, &queue_per_threads_publisher)
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
	fn spawn_event_poll_thread(&self, logical_core_identifier: LogicalCoreIdentifier, queue_per_threads_publisher: &QueuePerThreadQueuesPublisher<AdaptedReactorsRegistrar, String>) -> Result<JoinHandle<()>, ()>
	{
		let terminate = self.terminate.clone();
		let scheduler = self.process_configuration.per_thread_scheduler;
		let per_thread_event_poll_time_out_milliseconds = self.process_configuration.per_thread_event_poll_time_out_milliseconds;
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
				terminate.begin_termination_due_to_irrecoverable_error(&format!("Could not set current thread priority: `{}`", explanation));
				return
			}

			ProcessCommonConfiguration::lock_down_thread_nice_value_setting();

			let mut event_poll = match EventPoll::new(Arenas::default(), per_thread_event_poll_time_out_milliseconds)
			{
				Err(creation_error) =>
				{
					terminate.begin_termination_due_to_irrecoverable_error(&format!("Could not create EventPoll: `{:?}`", creation_error));
					return
				}
				Ok(event_poll) => event_poll,
			};
			registration.register_all_arenas(&mut event_poll);
			registration.register_any_reactors(&event_poll);
			let arguments = AdaptedReactorsRegistrar::wrap(&event_poll);
			let per_thread_subscriber = PerThreadQueueSubscriber::new(queue_per_threads_publisher, terminate.clone(), registration.deref(), &arguments);

			while terminate.should_continue()
			{
				if let Err(explanation) = event_poll.event_loop_iteration::<T>(&terminate)
				{
					terminate.begin_termination_due_to_irrecoverable_error(&explanation);
					return
				}

				if let Err(explanation) = per_thread_subscriber.receive_and_handle_messages(arguments)
				{
					terminate.begin_termination_due_to_irrecoverable_error(&explanation);
					return
				}
			}
		});

		thread_initialization_outcome.map_err(|io_error|
		{
			self.terminate.begin_termination_due_to_irrecoverable_error(&io_error);
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
		let mut signals_to_accept = HashSet::with_capacity(1);
		signals_to_accept.insert(SIGTERM);

		if running_interactively
		{
			signals_to_accept.insert(SIGHUP);
			signals_to_accept.insert(SIGINT);
			signals_to_accept.insert(SIGQUIT);
		}
		block_all_signals_on_current_thread_bar(&signals_to_accept);
		hash_set_to_signal_set(&signals_to_accept)
	}
}

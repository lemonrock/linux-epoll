// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Data with an item per logical core in use by the process.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerLogicalCoreData<T>
{
	logical_cores_data: Box<[Option<T>]>,
}

impl<T> Deref for PerLogicalCoreData<T>
{
	type Target = [Option<T>];

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.logical_cores_data
	}
}
impl<T> DerefMut for PerLogicalCoreData<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.logical_cores_data
	}
}

impl<T> PerLogicalCoreData<T>
{
	/// `t_initializer` is called for each defined logical core in `logical_cores`; it is passed the logical core's identifier.
	#[inline(always)]
	pub fn new(logical_cores: &LogicalCores, t_initializer: FnMut(usize)) -> Self
	{
		let number_of_logical_cores = logical_cores.len();
		assert_ne!(number_of_logical_cores, 0, "There are no logical cores specified");

		Self
		{
			logical_cores_data:
			{
				// Since the highest logical core is not necessarily the same as the length, this could still be resized.
				let mut logical_cores_data = Vec::with_capacity(number_of_logical_cores);
				let mut current_logical_core = 0;
				for logical_core_index_reference in logical_cores.iter()
				{
					let logical_core_index = (*logical_core_index_reference);

					while current_logical_core < logical_core_index
					{
						logical_cores_data.push(None);
						current_logical_core += 1;
					}
					debug_assert_eq!(current_logical_core, logical_core_index);
					logical_cores_data.push(Some(t_initializer(logical_core_index as u16)));

					current_logical_core = logical_core_index + 1;
				}
				debug_assert_eq!(current_logical_core, logical_cores_data.len());

				logical_cores_data.into_boxed_slice()
			},
		}
	}

	/// Gets the data for a particular logical core.
	///
	/// If the logical core does not exist (or does not have assigned data), returns None; this can happen on Linux if using the SO_INCOMING_CPU socket option, which can map to a CPU not assigned to the process.
	#[inline(always)]
	pub fn get(&self, logical_core_index: u16) -> Option<&T>
	{
		let logical_core_index = logical_core_index as usize;
		if unlikely!(logical_core_index >= self.logical_cores_data.len())
		{
			return None
		}
		unsafe { self.logical_cores_data.get_unchecked(logical_core_index).as_ref() }
	}

	/// Gets the mutable data for a particular logical core.
	///
	/// If the logical core does not exist (or does not have assigned data), returns None; this can happen on Linux if using the` SO_INCOMING_CPU` socket option, which can return an index for a CPU not assigned to the process.
	#[inline(always)]
	pub fn get_mut(&mut self, logical_core_index: u16) -> Option<&mut T>
	{
		let logical_core_index = logical_core_index as usize;
		if unlikely!(logical_core_index >= self.logical_cores_data.len())
		{
			return None
		}
		unsafe { self.logical_cores_data.get_unchecked_mut(logical_core_index).as_mut() }
	}

	/// Takes the data for a particular logical core.
	///
	/// If the logical core does not exist (or does not have assigned data), returns None; this can happen on Linux if using the SO_INCOMING_CPU socket option, which can map to a CPU not assigned to the process.
	#[inline(always)]
	pub fn take(&mut self, logical_core_index: u16) -> Option<T>
	{
		let logical_core_index = logical_core_index as usize;
		if unlikely!(logical_core_index >= self.logical_cores_data.len())
		{
			return None
		}
		unsafe { self.logical_cores_data.get_unchecked_mut(logical_core_index).take() }
	}
}

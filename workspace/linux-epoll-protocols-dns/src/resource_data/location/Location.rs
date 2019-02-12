// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Location resource data.
#[repr(C, packed)]
pub struct Location
{
	/// Version.
	version: u8,

	/// Size of a sphere at this location.
	pub size: LocationCentimetres,

	/// Horizontal precision.
	pub horizontal_precision: LocationCentimetres,

	/// Vertical precision.
	pub vertical_precision: LocationCentimetres,

	/// The latitude of the center of the sphere described by `size()`, expressed as a 32-bit integer, most significant octet first (network standard byte order), in thousandths of a second of arc.
	///
	/// 2^31 represents the equator; numbers above that are north latitude.
	pub unsigned_latitude: [u8; 4],

	/// The longitude of the center of the sphere described by `size()`, expressed as a 32-bit integer, most significant octet first (network standard byte order), in thousandths of a second of arc.
	///
	/// 2^31 represents the equator; numbers above that are north latitude.
	pub unsigned_longitude: [u8; 4],

	/// The altitude of the center of the sphere described by by `size()`d, expressed as a 32-bit integer, most significant octet first (network standard byte order), in centimeters, from a base of 100,000m below the WGS 84 reference spheroid used by GPS.
	pub unsigned_altitude: [u8; 4],
}

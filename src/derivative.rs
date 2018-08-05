// Copyright Brandon Ly 2018.
// Copyright Jeron A. Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

/// This struct will be used to store the derivatives of position and velocity
/// of for the RK4 approach
pub struct Derivative {
	/// Velocity
	pub dx: f32,
	/// Acceleration
	pub dv: f32,
}

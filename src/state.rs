// The Cala Physics Engine
//
// Copyright Jeron A. Lau 2018.
// Copyright Brandon Ly 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use kinematics;
use Derivative;

/// This struct will be used to store the position and velocity of the state
/// values for the RK4 approach
pub struct State {
	/// Position
	pub x: f32,
	/// Velocity
	pub dx: f32,
}

impl State {
	/// Evaluates and updates the state and derivatives of an object and
	/// returns the derivative
	pub fn evaluate(&mut self, initial: State, d: Derivative,
		t: f32, dt: f32) -> Derivative
	{
		*self = State {
			x: initial.x + d.dx * dt,
			dx: initial.dx + d.dv * dt
		};

		Derivative {
			dx: self.dx,
			dv: kinematics::acceleration(self.x, self.dx, t + dt)
		}
	}
}

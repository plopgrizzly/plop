// "rust_physics_engine" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
//  * Copyright (c) 2018  Brandon Ly <wowbob396@gmail.com>

use kinematics;
use Derivative;

/// This struct will be used to store the position and velocity of the state
/// values for the RK4 approach
pub struct State {
	pub x: f32, // position
	pub dx: f32, // velocity
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

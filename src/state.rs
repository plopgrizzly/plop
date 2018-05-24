// "rust_physics_engine" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
//  * Copyright (c) 2018  Brandon Ly <wowbob396@gmail.com>

use Derivative;

/// This struct will be used to store the position and velocity of the state
/// values for the RK4 approach
pub struct State {
	x: f32, // position
	v: f32, // velocity
}

impl State {
	/// Evaluates and updates the state and derivatives of an object and
	/// returns the derivative
	pub fn evaluate(&mut self, _d: Derivative, _t: f32, _dt: f32)
		-> Derivative
	{
		let mut _state: State;
		unreachable!()
	}
}

// "rust_physics_engine" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
//  * Copyright (c) 2018  Brandon Ly <wowbob396@gmail.com>
use Derivative;
/// This struct will be used to store the position and velocity of the state
/// values for the RK4 approach
pub struct State {
	pub x: f32, // position
	pub v: f32, // velocity
}
pub fn acceleration(x: f32, v: f32, t: f32)
-> f32
    {
    let k:f32 = 15.0;
    let b:f32 = 0.1;

    return -k *x - b * v;
}
impl State {
	/// Evaluates and updates the state and derivatives of an object and
	/// returns the derivative
	pub fn evaluate(&mut self,mut initial: State, mut _d: Derivative,mut _t: f32,mut _dt: f32)
		-> Derivative
	{
		let mut _state: State;
		_state.x = 0.0;
		_state.v = 0.0;

		_state.x = initial.x + _d.dx*_dt;
		_state.v = initial.v + _d.dv*_dt;

		let mut _output: Derivative;
		_output.dx = _state.v;
		_output.dv = acceleration(_state.x,_state.v,_t+_dt);

		return _output;
		unreachable!()
	}


}

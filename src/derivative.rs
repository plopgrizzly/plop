// "rust_physics_engine" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
//  * Copyright (c) 2018  Brandon Ly <wowbob396@gmail.com>

/// This struct will be used to store the derivatives of position and velocity
/// of for the RK4 approach
pub struct Derivative {
	pub dx: f32, // velocity
	pub dv: f32, // acceleration
}

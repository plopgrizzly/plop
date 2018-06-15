// "rust_physics_engine" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
//  * Copyright (c) 2018  Brandon Ly <wowbob396@gmail.com>

/// acceleration due to spring dampering.
pub fn acceleration(x: f32, v: f32, _t: f32) -> f32 {
	let k: f32 = 15.0;
	let b: f32 = 0.1;

	return -k * x - b * v;
}

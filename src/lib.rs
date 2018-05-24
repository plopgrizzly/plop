// "rust_physics_engine" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
//  * Copyright (c) 2018  Brandon Ly <wowbob396@gmail.com>

extern crate adi_screen;

mod window;
mod dimensions;
mod derivative;
mod rigid_body;
mod state;

pub use window::Window;
pub use dimensions::Dimensions;
pub use derivative::Derivative;
pub use rigid_body::RigidBody;
pub use state::State;

mod constants {
	pub const GRAVITY: f32 = 9.81;
	pub const METER: f32 = 1.0;
	pub const DT: f32 = 1f32/100f32;
}

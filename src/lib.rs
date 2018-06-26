// The Cala Physics Engine
//
// Copyright Jeron A. Lau 2018.
// Copyright Brandon Ly 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)
//
//! The Cala Physics Engine.

#![warn(missing_docs)]
#![doc(
	html_logo_url = "http://plopgrizzly.com/cala/icon.png",
	html_favicon_url = "http://plopgrizzly.com/cala/icon.ico",
	html_root_url = "http://plopgrizzly.com/cala/"
)]

extern crate aci_png; // For loading default icon
extern crate adi_screen; // For creating a window and drawing
extern crate ami; // For octree
extern crate euler;

mod world;
mod derivative;
mod rigid_body;
mod state;
mod kinematics;

#[doc(hidden)]
pub mod prelude;

pub use adi_screen::{ Transform, Input };
pub use ami::{ BBox };
pub use world::{ World, WorldBuilder, Object };
pub use derivative::Derivative;
pub use rigid_body::RigidBody;
pub use state::State;

#[doc(hidden)]
pub use adi_screen::Sprite;

mod constants {
	pub const GRAVITY: f32 = 9.81;
	#[allow(unused)] // TODO
	pub const METER: f32 = 1.0;
	#[allow(unused)] // TODO
	pub const DT: f32 = 1.0 / 100.0;
}

/// The integration step of the RK4 method
pub fn integrate(_state: State, _t: f32, _dt: f32) {
	
}

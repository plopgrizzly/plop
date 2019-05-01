// Copyright Brandon Ly 2018.
// Copyright Jeron A. Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

//! "Plop": Plop Grizzly Physics Engine.

#![warn(missing_docs)]
#![doc(
	html_logo_url = "http://free.plopgrizzly.com/plop/icon.svg",
	html_favicon_url = "http://free.plopgrizzly.com/plop/icon.svg",
	html_root_url = "http://free.plopgrizzly.com/plop/"
)]

extern crate aci_png; // For loading default icon
extern crate ami; // For octree

// mod world;
mod derivative;
mod rigid_body;
mod state;
mod kinematics;
mod mass;

/// Prelude module.
pub mod prelude;

pub use ami::{ BBox };
// pub use world::{ World, Object };
pub use derivative::Derivative;
pub use rigid_body::RigidBody;
pub use state::State;
pub use mass::Mass;

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

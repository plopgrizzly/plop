// "rust_physics_engine" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
//  * Copyright (c) 2018  Brandon Ly <wowbob396@gmail.com>

use adi_screen::Sprite;

use Dimensions;

use constants;

/// The structure that represents physical objects in the gameworld
pub struct RigidBody {
	mass: f32,
	x: f32,
	y: f32,
	z: f32,
	fx: f32,
	fy: f32,
	fz: f32,
	sprite: Sprite,
}

impl RigidBody {
	/// This method specifically applies the force of gravity to a given
	/// `RigidBody`
	pub fn apply_gravity(&mut self) {
		self.apply_force(Dimensions::Y, constants::GRAVITY);
	}

	/// This function applies a force to a rigidbody with a given dimension
	pub fn apply_force(&mut self, dimension: Dimensions, force: f32) {
		match dimension {
			Dimensions::X => self.fx = self.fx - force,
			Dimensions::Y => self.fy = self.fy - force,
			Dimensions::Z => self.fz = self.fz - force,
		}
	}
}

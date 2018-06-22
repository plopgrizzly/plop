// The Cala Physics Engine
//
// Copyright Jeron A. Lau 2018.
// Copyright Brandon Ly 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use prelude::*;
use ami::{ BBox, Collider };
use adi_screen::{ Window, Sprite, Transform };

use constants;

/// The structure that represents physical objects in the gameworld.
pub struct RigidBody {
	mass: f32,
	force: Vec3, // (fx, fy, fz)
	spin: Vec3, // (rx, ry, rz)
	sprites: Vec<Sprite>,
	bbox: BBox,
	position: Vec3,
	rotation: Vec3,
}

impl Collider for RigidBody {
	fn bbox(&self) -> BBox {
		self.bbox + self.position
	}
}

impl RigidBody {
	/// Create a new RigidBody.
	pub(crate) fn new(mass: f32, bbox: BBox, sprites: Vec<Sprite>,
		position: Vec3, rotation: Vec3) -> Self
	{
		RigidBody {
			mass, force: Vec3::new(0.0, 0.0, 0.0), sprites, bbox,
			position, spin: Vec3::new(0.0, 0.0, 0.0), rotation
		}
	}

	/// This method specifically applies the force of gravity to a given
	/// `RigidBody`
	pub fn apply_gravity(&mut self) {
		self.apply_force(Vec3{ x:0.0, y:constants::GRAVITY, z:0.0 });
	}

	/// Apply a force vector to this `RigidBody`.
	pub fn apply_force(&mut self, force: Vec3) {
		self.force += force;
	}

	/// Apply a spin vector to this `RigidBody`.
	pub fn apply_spin(&mut self, spin: Vec3) {
		self.spin += spin;
	}

	/// Move `RigidBody` based on applied forces for a set period of time.
	pub fn update(&mut self, window: &mut Window, time: f32) {
		self.position.x += self.force.x * time;
		self.position.y += self.force.y * time;
		self.position.z += self.force.z * time;

		self.rotation.x += self.spin.x * time;
		self.rotation.y += self.spin.y * time;
		self.rotation.z += self.spin.z * time;

		let t = Transform::IDENTITY.rt(self.rotation,self.position);

		for sprite in &self.sprites {
			sprite.transform(window, t);
		}
	}
}

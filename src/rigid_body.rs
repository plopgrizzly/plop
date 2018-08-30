// Copyright Brandon Ly 2018.
// Copyright Jeron A. Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use prelude::*;
use ami::*;

use constants;

/// The structure that represents physical objects in the gameworld.
pub struct RigidBody {
	#[allow(unused)] // TODO
	mass: Mass,
	force: Vector, // position step (fx, fy, fz)
	spin: Rotation, // rotation step (quaternion)
	bbox: BBox,
	position: Vector, // XYZ 3D position
	rotation: Rotation, // quaternion rotation
}

impl Collider for RigidBody {
	fn bbox(&self) -> BBox {
		self.bbox + self.position
	}
}

// TODO: Add Friction Functions.
impl RigidBody {
	/// Create a new RigidBody.
	pub fn new(mass: Mass, bbox: BBox, position: Vector,
		rotation: Rotation) -> Self
	{
		RigidBody {
			mass, force: vector!(), bbox, position,
			spin: Rotation::identity(), rotation,
		}
	}

	/// This method specifically applies the force of gravity to a given
	/// `RigidBody`
	pub fn apply_gravity(&mut self) {
		self.apply_force(Vector{ x:0.0, y:constants::GRAVITY, z:0.0 });
	}

	/// Apply a force vector to this `RigidBody`.
	pub fn apply_force(&mut self, force: Vector) {
		self.force += force;
	}

	/// Add a spin to this `RigidBody`.
	pub fn apply_spin(&mut self, spin: Rotation) {
		self.spin = self.spin.then(spin);
	}

	/// Stop forces on this `RigidBody`.
	pub fn stop_force(&mut self) {
		self.force = vector!();
	}

	/// Stop spins on this `RigidBody`.
	pub fn stop_spin(&mut self) {
		self.spin = Rotation::identity();
	}

	/// Move `RigidBody` based on applied forces for a set period of time.
	///
	/// Returns a transformation Matrix for visualization.
	pub fn update(&mut self, time: f32) -> Matrix {
		self.position += self.force * time;
		self.rotation.then(self.spin * time);

		matrix!().rt(self.rotation,self.position)
	}
}

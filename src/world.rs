// Copyright Brandon Ly 2018.
// Copyright Jeron A. Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use std::rc::Rc;
use std::cell::{ RefCell, RefMut };

use ami::{ Octree, Id };
use adi_screen::{ Screen, Sprite };
use RigidBody;
use BBox;
use prelude::*;

/// An RigidBody in the octree.
pub struct Object {
	world: World,
	id: Id,
}

impl Object {
	/// This method specifically applies the force of gravity to a given
	/// `Object`.
	pub fn apply_gravity(&mut self) {
		(*self.world.0.borrow_mut()).octree[self.id]
			.apply_gravity();
	}

	/// Apply a force vector to an `Object`
	pub fn apply_force(&mut self, force: Vec3) {
		(*self.world.0.borrow_mut()).octree[self.id].apply_force(force)
	}

	/// Apply a spin vector to an `Object`
	pub fn apply_spin(&mut self, spin: Vec3) {
		(*self.world.0.borrow_mut()).octree[self.id].apply_spin(spin)
	}

	/// Move `Object` based on applied forces for a set period of time.
	pub fn update(&mut self, time: f32) {
		let mut object = self.world.0.borrow_mut().octree.remove(self.id);
		object.update(&mut self.world.0.borrow_mut().window, time);
		let _id = self.world.0.borrow_mut().octree.add(object);
		assert_eq!(self.id, _id);
	}
}

#[doc(hidden)]
pub struct WorldData<'a> {
	octree: Octree<RigidBody>,
}

impl WorldBuilder {
	/// Create the World and open a window.
	pub fn finish(self) -> World {
		let mut window = WindowBuilder::new()
			.background((self.bgc[0], self.bgc[1], self.bgc[2]));

		if let Some(fog) = self.fog {
			window = window.fog(fog.0, fog.1);
		}
		
		let window = window.finish();
		let octree = Octree::new();

		World()
	}
}

/// The 3D world space.
#[derive(Clone)] pub struct World {
	world: Rc<RefCell<WorldData<'static>>>,
}

impl World {
	pub fn new() -> World {
		
	}
}

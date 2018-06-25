// The Cala Physics Engine
//
// Copyright Jeron A. Lau 2018.
// Copyright Brandon Ly 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use std::rc::Rc;
use std::cell::{ RefCell, RefMut };

use aci_png;
use ami::{ Octree, Id };
use adi_screen::{ Window, WindowBuilder, Sprite };
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

/// A 2D GUI element.
pub struct Overlay {
	#[allow(unused)] // TODO
	sprite: Sprite,
}

/// Add a collection of sprites as a RigidBody to the world.
#[macro_export] macro_rules! add {
	($world:expr, $mass:expr, $bbox:expr, $pos:expr, $rot:expr,
		$( $x:expr ),*) =>
	{ {
		let t = $crate::Transform::IDENTITY.rt($rot, $pos);
		let sprites: Box<[$crate::Sprite]>
			= Box::new(sprites!(&mut $world.window().window,
				$(($x.0, $x.1, t.clone(), $x.2)),*));
		let sprites = sprites.into_vec();
		$world.add($mass, $bbox, sprites, $pos, $rot)
	} }
}

/// Macro to create multiple fog-affected sprites in an array.
/// # Example
/// See [`add!()`](macro.add.html)
#[macro_export] macro_rules! add_fog {
	($world:expr, $mass:expr, $bbox:expr, $pos:expr, $rot:expr,
		$( $x:expr ),*) =>
	{ {
		let t = $crate::Transform::IDENTITY.rt($rot, $pos);
		let sprites: Box<[Sprite]>
			= Box::new(sprites_fog!(&mut $world.window().window,
				$(($x.0, $x.1, t.clone(), $x.2)),*));
		let sprites = sprites.into_vec();
		$world.add($mass, $bbox, sprites, $pos, $rot)
	} }
}

/// Macro to create multiple non-camera affected sprites in an array.
/// # Example
/// See [`add!()`](macro.add.html)
#[macro_export] macro_rules! add_gui {
	($world:expr, $( $x:expr ),*) => { {
		let sprite: Sprite = sprites_gui!(&mut $world.window().window, $($x),*)[0];
		$world.overlay(sprite)
	} }
}

/// Macro to load textures from files for the window.
///
/// The first texture file listed is indexed 0, and each subsequent texture
/// increases by 1.  See: [`add!()`](macro.add.html) for example.
#[macro_export] macro_rules! set_textures {
	($world:expr, $decode:expr, $( $x:expr ),*) => { {
		let window = &mut $world.window().window;
		textures!(window, $decode, $($x),*)
	} }
}

/// Macro to load models from files for the window.
///
/// The first model file listed is indexed 0, and each subsequent model
/// increases by 1.  See: [`add!()`](macro.add.html) for example.
#[macro_export] macro_rules! set_models {
	($world:expr, $( $x:expr ),*) => { {
		let window = &mut $world.window().window;
		models!(window, $($x),*)
	} }
}

#[doc(hidden)]
pub struct WorldData {
	pub window: Window,
	octree: Octree<RigidBody>,
}

/// Builder for World
pub struct WorldBuilder {
	name: String,
	bgc: Vec3,
	icon: Option<Graphic>,
	fog: Option<(f32, f32)>,
}

impl WorldBuilder {
	/// Create a new WorldBuilder.
	pub fn new() -> Self {
		WorldBuilder {
			name: "Cala Program".to_string(),
			bgc: vec3!(0.1, 0.1, 0.1),
			icon: None,
			fog: None,
		}
	}

	/// Set the window name.
	pub fn name(mut self, name: &str) -> Self {
		self.name = name.to_string();
		self
	}

	/// Set the background color.
	pub fn background(mut self, color: Vec3) -> Self {
		self.bgc = color;
		self
	}

	/// Set the window icon
	pub fn icon(mut self, icon: Graphic) -> Self {
		self.icon = Some(icon);
		self
	}

	/// Set the fog, default is off.
	pub fn fog(mut self, fog: (f32, f32)) -> Self {
		self.fog = Some(fog);
		self
	}

	/// Create the World and open a window.
	pub fn finish(mut self) -> World {
		if self.icon.is_none() {
			let icon = aci_png::decode(
				include_bytes!("../icon.png")).unwrap();
			self.icon = Some(icon);
		}

		let mut window = WindowBuilder::new(&self.name, self.icon)
			.background(self.bgc.x, self.bgc.y, self.bgc.z);

		if let Some(fog) = self.fog {
			window = window.fog(fog.0, fog.1);
		}
		
		let window = window.finish();
		let octree = Octree::new();

		World(Rc::new(RefCell::new(WorldData { window, octree })))
	}
}

/// The 3D world space.
#[derive(Clone)] pub struct World(Rc<RefCell<WorldData>>);

impl World {
	/// Change the camera position.
	pub fn camera(&mut self, pos: Vec3, rot: Vec3) {
		self.window().window.camera(pos, rot);
	}

	/// Get the number of seconds since the previous frame.
	pub fn since(&mut self) -> f32 {
		self.window().window.since()
	}

	#[doc(hidden)]
	pub fn add(&mut self, mass: f32, bbox: BBox, sprites: Vec<Sprite>,
		position: Vec3, rotation: Vec3) -> Object
	{
		Object {
			world: self.clone(),
			id: (*self.0.borrow_mut()).octree
				.add(RigidBody::new(mass, bbox, sprites,
					position, rotation))
		}
	}

	#[doc(hidden)]
	pub fn overlay(sprite: Sprite) -> Overlay {
		Overlay { sprite }
	}

	/// Get the underlying adi_screen `Window`
	pub fn window(&mut self) -> RefMut<WorldData> {
		self.0.borrow_mut()
	}

	/// Update the display and get the input.
	pub fn update(&mut self) -> Option<::adi_screen::Input> {
		self.window().window.update()
	}
}

// The Cala Physics Engine
//
// Copyright Jeron A. Lau 2018.
// Copyright Brandon Ly 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

#[macro_use] extern crate cala;
extern crate aci_png;

use std::f32::consts::PI;
use cala::prelude::*;
use cala::{ World, Object };

const MOVE_SPEED : f32 = 6.0;
const LOOK_SPEED : f32 = 0.2 * PI;

struct Context {
	// The World Space
	world: World,
	// Camera position and rotation state
	pos: Vec3,
	rot: Vec3,
	// The sprites in the world
	falling: Object,
	rotator: Object,
}

fn read_input(context: &mut Context, input: Input) -> bool {
	let t = context.world.since();

	match input {
		Back | Quit => return true,
		W(Some(_)) => {
			let x = f32::sin(-context.rot.y) * t * MOVE_SPEED;
			let z = f32::cos(-context.rot.y) * t * MOVE_SPEED;
			context.pos.x += x;
			context.pos.z += z;
			context.world.camera(context.pos, context.rot);
		}
		S(Some(_)) => {
			let x = f32::sin(-context.rot.y) * -t * MOVE_SPEED;
			let z = f32::cos(-context.rot.y) * -t * MOVE_SPEED;
			context.pos.x += x;
			context.pos.z += z;
			context.world.camera(context.pos, context.rot);
		}
		D(Some(_)) => {
			let x = f32::sin(-(context.rot.y - (0.5 * PI))) * t * MOVE_SPEED;
			let z = f32::cos(-(context.rot.y - (0.5 * PI))) * t * MOVE_SPEED;
			context.pos.x += x;
			context.pos.z += z;
			context.world.camera(context.pos, context.rot);
		}
		A(Some(_)) => {
			let x = f32::sin(-(context.rot.y + (0.5 * PI))) * t * MOVE_SPEED;
			let z = f32::cos(-(context.rot.y + (0.5 * PI))) * t * MOVE_SPEED;
			context.pos.x += x;
			context.pos.z += z;
			context.world.camera(context.pos, context.rot);
		}
		Space(Some(_)) => {
			context.pos.y -= t * MOVE_SPEED;
			context.world.camera(context.pos, context.rot);
		}
		LShift(Some(_)) | RShift(Some(_)) => {
			context.pos.y += t * MOVE_SPEED;
			context.world.camera(context.pos, context.rot);
		}
		Q(Some(_)) => {
			context.rot.y += t * LOOK_SPEED;
			context.world.camera(context.pos, context.rot);
		}
		E(Some(_)) => {
			context.rot.y -= t * LOOK_SPEED;
			context.world.camera(context.pos, context.rot);
		}
		R(Some(_)) => {
			context.rotator.update(t);
		}
		F(Some(_)) => {
			context.falling.update(t);
		}
		Resize => {
			// TODO: resize GUIS
		}
		_ => {},
	}

	false
}

fn main() {
	let mut world = World::new();
	set_textures!(world, aci_png::decode,
		"res/box.png",
		"res/ball.png"
	);
	set_models!(world, "res/block.data");

	// Rotating object
	let mut rotator = add!(&mut world, 1.0,
		BBox::new(
			vec3!(-0.5, -0.5, -0.5),
			vec3!(0.5, 0.5, 0.5),
		),
		vec3!(0.0, -0.5, 2.0),
		vec3!(0.0, 0.0, 0.0),
		(0/*block model*/, Some(0/*box tex*/), false));
	rotator.apply_spin(vec3!(0.0, -0.2 * PI, 0.0)); // Add rotation.

	// Falling object
	let mut falling = add!(world, 1.0,
		BBox::new(
			vec3!(-0.5, -0.5, -0.5),
			vec3!(0.5, 0.5, 0.5),
		),
		vec3!(0.0, -4.5, 2.0),
		vec3!(0.0, 0.0, 0.0),
		(0/*block model*/, Some(0/*box tex*/), false));
	falling.apply_gravity(); // Add the force of gravity to this object.

	let mut context = Context {
		world,
		pos: vec3!(0.0, 0.0, 0.0),
		rot: vec3!(0.0, 0.0, 0.0),
		falling,
		rotator,
	};

	'program: loop {
		// Go through this frame's input.
		while let Some(input) = context.world.update() {
			if read_input(&mut context, input) {
				break 'program;
			}
		}
	}
}

extern crate rust_physics_engine;
extern crate aci_png;
#[macro_use]
extern crate adi_screen;

use rust_physics_engine::{
	Window,
};

use adi_screen::{
	Input, Transform, Key, Text, Msg, Sprite, IDENTITY
};

use std::f32::consts::PI;

const MOVE_SPEED : f32 = 6.0;
const LOOK_SPEED : f32 = 0.1;

struct Context {
	// Window
	window: Window,
	// Camera position and rotation state
	pos: (f32, f32, f32),
	rot: (f32, f32, f32),
	// Testing Text
	text: Text,
	// The sprites in the world
	sprites: [Sprite; 2],
	// Rotation of the box
	box_r: f32,
}

fn read_input(context: &mut Context, input: Input) -> bool {
	let t = context.window.window().since();

	match input {
		Input::Msg(Msg::Back) | Input::Msg(Msg::Quit) => return true,
		Input::KeyHold(Key::W) => {
			let x = f32::sin(-2.0 * PI * context.rot.1) * t * MOVE_SPEED;
			let z = f32::cos(-2.0 * PI * context.rot.1) * t * MOVE_SPEED;
			context.pos.0 += x;
			context.pos.2 += z;
			context.window.window().camera(context.pos, context.rot);
		},
		Input::KeyHold(Key::S) => {
			let x = f32::sin(-2.0 * PI * context.rot.1) * -t * MOVE_SPEED;
			let z = f32::cos(-2.0 * PI * context.rot.1) * -t * MOVE_SPEED;
			context.pos.0 += x;
			context.pos.2 += z;
			context.window.window().camera(context.pos, context.rot);
		},
		Input::KeyHold(Key::D) => {
			let x = f32::sin(-2.0 * PI * (context.rot.1 - 0.25)) * t * MOVE_SPEED;
			let z = f32::cos(-2.0 * PI * (context.rot.1 - 0.25)) * t * MOVE_SPEED;
			context.pos.0 += x;
			context.pos.2 += z;
			context.window.window().camera(context.pos, context.rot);
		},
		Input::KeyHold(Key::A) => {
			let x = f32::sin(-2.0 * PI * (context.rot.1 + 0.25)) * t * MOVE_SPEED;
			let z = f32::cos(-2.0 * PI * (context.rot.1 + 0.25)) * t * MOVE_SPEED;
			context.pos.0 += x;
			context.pos.2 += z;
			context.window.window().camera(context.pos, context.rot);
		},
		Input::KeyHold(Key::Space) => {
			context.pos.1 -= t * MOVE_SPEED;
			context.window.window().camera(context.pos, context.rot);
		},
		Input::KeyHold(Key::LShift) | Input::KeyHold(Key::RShift) => {
			context.pos.1 += t * MOVE_SPEED;
			context.window.window().camera(context.pos, context.rot);
		},
		Input::KeyHold(Key::Q) => {
			context.rot.1 += t * LOOK_SPEED;
			context.window.window().camera(context.pos, context.rot);
		},
		Input::KeyHold(Key::E) => {
			context.rot.1 -= t * LOOK_SPEED;
			context.window.window().camera(context.pos, context.rot);
		},
		Input::KeyHold(Key::R) => {
			context.box_r += t * LOOK_SPEED;
			Transform::new()
				.rotate(context.box_r, 0.0, 0.0)
				.translate(0.0, -0.5, 2.0)
				.apply(&mut context.window.window(),
					&mut context.sprites[0]);
		}
		Input::Resize => {
			context.text.update(context.window.window(), "Test", None);
//				"Physics Test\n", None);
		}
		_ => {},
	}

	false
}

fn main() {
	let mut window = rust_physics_engine::Window::new();

	textures!(textures, window.window(), aci_png::decode,
		"res/box.png",
		"res/ball.png"
	);
	
	models!(models, window.window(), "res/block.data");

	sprites!(sprites, window.window(), (&models[0], Some(&textures[0]),
		Transform::new().translate(0.0, -0.5, 2.0), false),
			(&models[0], Some(&textures[0]),
		Transform::new().translate(0.0, -4.5, 2.0), false));

	let mut context = Context {
		text: Text::new(&mut window.window(), (-1.0, -1.0), (0.25, 0.125)),
		window: window,
		pos: (0.0, 0.0, 0.0),
		rot: (0.0, 0.0, 0.0),
		sprites,
		box_r: 0.0
	};

	'program: loop {
		// Go through this frame's input.
		while let Some(input) = context.window.window().update() {
			if read_input(&mut context, input) {
				break 'program;
			}
		}
	}
}

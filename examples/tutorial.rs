extern crate aci_png;
#[macro_use]
extern crate adi_screen;

use adi_screen:: {
	Window, Input, Transform, Model, TexCoords, Key,  SpriteList, Text,
	DEFAULT_FONT, Msg, Sprite
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
	sprites: Vec<Sprite>,
	// Rotation of the box
	box_r: f32,
}

fn read_input(context: &mut Context, input: Input) -> bool {
	let t = context.window.since();

	match input {
		Input::Msg(Msg::Back) | Input::Msg(Msg::Quit) => return true,
		Input::KeyHold(Key::W) => {
			let x = f32::sin(-2.0 * PI * context.rot.1) * t * MOVE_SPEED;
			let z = f32::cos(-2.0 * PI * context.rot.1) * t * MOVE_SPEED;
			context.pos.0 += x;
			context.pos.2 += z;
			context.window.camera(context.pos, context.rot);
		},
		Input::KeyHold(Key::S) => {
			let x = f32::sin(-2.0 * PI * context.rot.1) * -t * MOVE_SPEED;
			let z = f32::cos(-2.0 * PI * context.rot.1) * -t * MOVE_SPEED;
			context.pos.0 += x;
			context.pos.2 += z;
			context.window.camera(context.pos, context.rot);
		},
		Input::KeyHold(Key::D) => {
			let x = f32::sin(-2.0 * PI * (context.rot.1 - 0.25)) * t * MOVE_SPEED;
			let z = f32::cos(-2.0 * PI * (context.rot.1 - 0.25)) * t * MOVE_SPEED;
			context.pos.0 += x;
			context.pos.2 += z;
			context.window.camera(context.pos, context.rot);
		},
		Input::KeyHold(Key::A) => {
			let x = f32::sin(-2.0 * PI * (context.rot.1 + 0.25)) * t * MOVE_SPEED;
			let z = f32::cos(-2.0 * PI * (context.rot.1 + 0.25)) * t * MOVE_SPEED;
			context.pos.0 += x;
			context.pos.2 += z;
			context.window.camera(context.pos, context.rot);
		},
		Input::KeyHold(Key::Space) => {
			context.pos.1 -= t * MOVE_SPEED;
			context.window.camera(context.pos, context.rot);
		},
		Input::KeyHold(Key::LShift) | Input::KeyHold(Key::RShift) => {
			context.pos.1 += t * MOVE_SPEED;
			context.window.camera(context.pos, context.rot);
		},
		Input::KeyHold(Key::Q) => {
			context.rot.1 += t * LOOK_SPEED;
			context.window.camera(context.pos, context.rot);
		},
		Input::KeyHold(Key::E) => {
			context.rot.1 -= t * LOOK_SPEED;
			context.window.camera(context.pos, context.rot);
		},
		Input::KeyHold(Key::R) => {
			context.box_r += t * LOOK_SPEED;
			Transform::new()
				.rotate(context.box_r, 0.0, 0.0)
				.translate(0.0, -0.5, 2.0)
				.apply(&mut context.window,
					&mut context.sprites[0]);
		}
		Input::Resize => {
			context.text.update(&mut context.window,
				"Physics Test\n", DEFAULT_FONT);
		}
		_ => {},
	}

	false
}

fn main() {
	let mut window = Window::new("Physics Test", &aci_png::decode(
		include_bytes!("res/plopgrizzly.png")).unwrap(),
		(0.05, 0.05, 1.0), None);

	let textures = textures!(&mut window, aci_png::decode,
		"res/box.png",
		"res/ball.png"
	);

	let block_model = Model::new(&mut window, include!("res/block.data"));
	let block_tc = TexCoords::new(&mut window, &include!("res/block.texc"));

	let sprites = SpriteList::new(block_model)
		.transform(Transform::new().translate(0.0, -0.5, 2.0))
		.texture(&mut window, textures[0], block_tc).to_vec();

	let mut context = Context {
		text: Text::new(&mut window, (-1.0, -1.0), (0.25, 0.125)),
		window,
		pos: (0.0, 0.0, 0.0),
		rot: (0.0, 0.0, 0.0),
		sprites,
		box_r: 0.0
	};

	'program: loop {
		// Go through this frame's input.
		while let Some(input) = context.window.update() {
			if read_input(&mut context, input) {
				break 'program;
			}
		}
	}
}

// "rust_physics_engine" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
//  * Copyright (c) 2018  Brandon Ly <wowbob396@gmail.com>

use adi_screen;
use adi_screen::WindowBuilder;

pub struct Window {
	window: adi_screen::Window,
}

impl Window {
	pub fn new() -> Self {
		let window = WindowBuilder::new("Physics Test", None)
			.background(0.1, 0.1, 0.1)
			.finish();

		Window {
			window
		}
	}

	pub fn window(&mut self) -> &mut adi_screen::Window {
		&mut self.window
	}
}

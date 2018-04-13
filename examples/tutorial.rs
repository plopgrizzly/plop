extern crate adi_screen;

use adi_screen:: {
	Window, Input, Transform, Texture, Model, TexCoords, Key, 
	SpriteList, Text, DEFAULT_FONT
};

use std::f32::consts::PI;

fn main() {
	let mut window = Window::new("Physics Test", &aci_png::decode(
		include_bytes!("res/plopgrizzly.png")).unwrap(),
		(0.25, 0.25, 1.0), None);

	
}

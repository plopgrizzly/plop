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

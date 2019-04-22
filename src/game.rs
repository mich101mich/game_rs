use super::{log, Backend, BackendStyle, Color};

mod interface;
pub use interface::*;
mod world;
pub use world::*;

pub struct Game {
	pub mouse: Mouse,
	line_start: (f32, f32),
	line_end: (f32, f32),
}

impl Game {
	pub fn new() -> Self {
		log!("Starting...");
		Game {
			mouse: Mouse::new(),
			line_start: (100.0, 200.0),
			line_end: (300.0, 300.0),
		}
	}

	pub fn draw(&mut self, backend: &mut Backend) {
		backend.fill(Color::rgb(128, 128, 128));

		self.line_start.0 += self.line_start.1 * 0.1;
		self.line_start.1 += -self.line_start.0 * 0.1;

		backend.draw_line(self.line_start, self.line_end, Color::rgb(255, 0, 0));

		backend.stroke_circle(20.0, 100.0, 10.0, 5.0, Color::rgb(0, 255, 0));
		backend.stroke_circle(20.0, 100.0, 4.0, 1.0, Color::rgb(0, 255, 0));
		backend.fill_circle(60.0, 100.0, 10.0, Color::rgb(0, 255, 0));

		backend.stroke_rect(300.0, 50.0, 50.0, 20.0, 5.0, Color::rgb(0, 0, 255));
		backend.fill_rect(300.0, 100.0, 50.0, 20.0, Color::rgb(0, 0, 255));

		backend.draw_text("Hello World", 50.0, 50.0, Color::rgb(0, 0, 0));
	}

	pub fn end(&mut self) {}

	pub fn on_key_press(&mut self, code: Option<KeyCode>, shift: bool, ctrl: bool) {
		self.mouse.set_shift(shift);
		self.mouse.set_ctrl(ctrl);
	}
}

use super::{Backend, Color, log, BackendStyle};

pub struct Game {
	line_start: (f32, f32),
	line_end: (f32, f32)
}

impl Game {
	pub fn new() -> Self {
		log!("Starting...");
		Game {
			line_start: (100.0, 200.0), line_end: (300.0, 300.0),
		}
	}

	pub fn draw(&mut self, backend: &mut Backend) {
		backend.fill(Color::rgb(128, 128, 128));

		self.line_start.0 += self.line_start.1 * 0.1;
		self.line_start.1 += -self.line_start.0 * 0.1;

		backend.draw_line(self.line_start, self.line_end, 3.0, Color::rgb(255, 0, 0));
	}
}

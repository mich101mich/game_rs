use graphics::*;

pub struct Game {
	line: [f64; 4],
}

impl Game {
	pub fn new() -> Self {
		Game {
			line: [100.0, 200.0, 300.0, 300.0],
		}
	}

	pub fn draw<T: Graphics>(&mut self, context: Context, backend: &mut T) {
		//log!("{:?}", self.line);

		clear([0.5, 0.5, 0.5, 1.0], backend);

		self.line[0] += self.line[1] * 0.1;
		self.line[1] += -self.line[0] * 0.1;

		let line = Line::new([0.0, 1.0, 0.0, 1.0], 10.0);
		line.draw(self.line, &context.draw_state, context.transform, backend);
	}
}

use super::{log, ui, world::World, Backend, BackendStyle, Color};

pub struct Game {
	pub mouse: ui::Mouse,
	pub world: World,
}

impl Game {
	pub fn new() -> Self {
		log!("Starting...");
		Game {
			mouse: ui::Mouse::new(),
			world: World::new(128, 128),
		}
	}

	pub fn draw(&mut self, backend: &mut Backend) {
		backend.fill(Color::rgb(128, 128, 128));
		self.world.draw(backend);

		backend.stroke_circle((20.0, 100.0), 20.0, 5.0, Color::rgb(0, 255, 0));
		backend.stroke_circle((20.0, 100.0), 4.0, 1.0, Color::rgb(0, 255, 0));
		backend.fill_circle((60.0, 100.0), 10.0, Color::rgb(0, 255, 0));

		backend.stroke_rect((300.0, 50.0), (50.0, 20.0), 5.0, Color::rgb(0, 0, 255));
		backend.fill_rect((300.0, 100.0), (50.0, 20.0), Color::rgb(0, 0, 255));

		backend.draw_text("Hello World", (50.0, 50.0), Color::rgb(0, 0, 0));
	}

	pub fn end(&mut self) {}

	pub fn on_key_press(&mut self, code: Option<ui::KeyCode>, shift: bool, ctrl: bool) {
		self.mouse.set_shift(shift);
		self.mouse.set_ctrl(ctrl);
	}
}

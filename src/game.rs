use super::{log, ui, world::World, Backend, BackendStyle, Color};

pub struct Game {
	pub mouse: ui::Mouse,
	pub world: World,
	background_dirty: bool,
}

impl Game {
	pub fn new() -> Self {
		log!("Starting...");
		Game {
			mouse: ui::Mouse::new(),
			world: World::new(128, 128),
			background_dirty: true,
		}
	}

	pub fn draw(&mut self, backend: &mut Backend) {
		backend.fill(Color::rgb(128, 128, 128));

		if self.background_dirty {
			backend.fill(Color::rgb(128, 128, 128));
			backend.clear_background();
			for row in 0..3 {
				for col in 0..16 {
					backend.draw_to_background(
						(row, col),
						((col * 20) as f32, (row * 16) as f32 + 200.0),
					);
				}
			}
		}
		backend.draw_background();

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

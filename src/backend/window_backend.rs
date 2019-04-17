use super::BackendStyle;
use crate::Game;

pub use sfml::graphics::Color;
use sfml::graphics::{RenderTarget, RenderWindow};

pub struct Backend {
	window: RenderWindow,
}

impl BackendStyle for Backend {

	fn start(mut game: Game) {
		let mut backend = Backend {
			window: RenderWindow::new((640, 480), "game", Default::default(), &Default::default()),
		};

		'game_loop: loop {
			// Process events
			while let Some(event) = backend.window.poll_event() {
				use sfml::window::Event::*;
				match event {
					Closed => {
						backend.window.close();
						break 'game_loop;
					}
					_ => {}
				}
			}

			game.draw(&mut backend);

			backend.window.display();
		}
	}

	fn get_width(&self) -> u32 {
		self.window.size().x
	}
	fn get_height(&self) -> u32 {
		self.window.size().y
	}

	fn fill(&mut self, color: Color) {
		self.window.clear(&color);
	}

	fn draw_line(&mut self, start: (f32, f32), end: (f32, f32), width: f32, color: Color) {}


	fn fill_rect(&mut self, left: f32, top: f32, right: f32, bottom: f32, color: Color) {}
	fn draw_rect(
		&mut self,
		left: f32,
		top: f32,
		right: f32,
		bottom: f32,
		width: f32,
		color: Color,
	) {

	}

	fn fill_ellipse(&mut self, x: f32, y: f32, rx: f32, ry: f32, color: Color) {}
	fn draw_ellipse(&mut self, x: f32, y: f32, rx: f32, ry: f32, width: f32, color: Color) {}


	fn draw_text(&mut self, text: &str, x: f32, y: f32, height: f32, color: Color) {}
}


#[macro_export]
macro_rules! log {
	( $( $x: expr ),* ) => {
		println!($( $x ),*)
	};
}

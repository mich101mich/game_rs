extern crate piston_window;

use piston_window::*;

macro_rules! log {
	( $( $x: expr ),* ) => {
		println!($( $x ),*)
	};
}

mod game;

fn main() {
	let mut window: PistonWindow = WindowSettings::new("game", (640, 480))
		.build()
		.unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

	let mut game = game::Game::new();

	println!("{:<02x}", (0.01 * 255.0) as usize);

	while let Some(e) = window.next() {
		window.draw_2d(&e, |c, g| {
			game.draw(c, g);
		});
	}
}

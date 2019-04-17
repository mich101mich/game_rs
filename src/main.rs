
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

mod backend;
pub use backend::{Color, Backend, BackendStyle};

mod game;
pub use game::Game;

fn main() {
	let game = Game::new();

	Backend::start(game);
}

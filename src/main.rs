
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate arithm_derive;

mod backend;
pub use backend::{Color, Backend, BackendStyle};

mod game;
pub use game::Game;

fn main() {
	let game = Game::new();

	Backend::start(game);
}

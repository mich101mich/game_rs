use super::{GamePos, TilePos};
use crate::{Backend, BackendStyle, Color};

pub struct Worker {
	pub pos: TilePos,
}

impl Worker {
	pub fn new(pos: TilePos) -> Self {
		Worker { pos }
	}
	pub fn draw(&self, backend: &mut Backend) {
		backend.fill_rect(
			GamePos::from(self.pos) + GamePos::new(2.0, 2.0),
			(12.0, 12.0),
			Color::rgb(255, 0, 0),
		);
	}
}

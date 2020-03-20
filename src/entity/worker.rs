use super::Exec;
use crate::{
	ui::{Clickable, Hitbox},
	world::{GamePos, TilePos},
	Backend, BackendStyle, Color,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkerID(usize);
crate::make_id!(WorkerID, Worker);

#[derive(Debug)]
pub struct Worker {
	pub id: WorkerID,
	pub pos: TilePos,
	pub plan: Vec<Exec>,
}

impl Worker {
	pub fn new(id: WorkerID, pos: TilePos) -> Self {
		Self {
			id,
			pos,
			plan: vec![],
		}
	}
	pub fn draw(&self, backend: &mut Backend) {
		backend.fill_hitbox(self.hitbox(), Color::rgb(255, 0, 0));
	}
}

impl Clickable for Worker {
	fn hitbox(&self) -> Hitbox {
		let pos = GamePos::from(self.pos);
		Hitbox::Rect {
			pos: pos + GamePos::new(2.0, 2.0),
			size: GamePos::new(12.0, 12.0),
		}
	}
}

use super::Exec;
use crate::{
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
		backend.fill_rect(
			GamePos::from(self.pos) + GamePos::new(2.0, 2.0),
			(12.0, 12.0),
			Color::rgb(255, 0, 0),
		);
	}
}

use super::{Exec, GamePos, TilePos};
use crate::{Backend, BackendStyle, Color};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkerID(usize);

impl std::fmt::Display for WorkerID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Worker({})", self.0)
	}
}
impl From<WorkerID> for usize {
	fn from(id: WorkerID) -> usize {
		id.0
	}
}

pub struct Worker {
	pub id: WorkerID,
	pub pos: TilePos,
	pub plan: Vec<Exec>,
}

impl Worker {
	pub fn new(id: usize, pos: TilePos) -> Self {
		Worker {
			id: WorkerID(id),
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

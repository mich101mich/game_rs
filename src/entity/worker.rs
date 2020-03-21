use super::{Exec, Item};
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
	item: Option<Item>,
}

impl Worker {
	pub fn new(id: WorkerID, pos: TilePos) -> Self {
		Self {
			id,
			pos,
			plan: vec![],
			item: None,
		}
	}

	pub fn draw(&self, backend: &mut Backend) {
		backend.fill_hitbox(self.hitbox(), Color::rgb(250, 191, 15));
		if let Some(item) = self.item.as_ref() {
			item.draw(backend);
		}
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
	fn context_menu(&self) -> std::vec::Vec<(usize, String)> {
		unimplemented!()
	}
}

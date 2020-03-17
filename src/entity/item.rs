use crate::{
	world::{GamePos, Mineral},
	Backend, BackendStyle, Color,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemID(usize);
crate::make_id!(ItemID, Item);

#[derive(Debug)]
pub struct Item {
	id: ItemID,
	pos: GamePos,
	mineral: Mineral,
}

impl Item {
	pub fn new(id: ItemID, pos: GamePos, mineral: Mineral) -> Self {
		Self { id, pos, mineral }
	}

	pub fn draw(&self, backend: &mut Backend) {
		backend.fill_circle(self.pos, 4.0, Color::rgb(255, 0, 0));
	}
}

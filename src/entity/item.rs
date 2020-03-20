use crate::{
	ui::{Clickable, Hitbox},
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
		backend.fill_hitbox(
			self.hitbox(),
			match self.mineral {
				Mineral::Crystal => Color::rgb(87, 255, 23),
				Mineral::Ore => Color::rgb(165, 110, 31),
			},
		);
	}
}

impl Clickable for Item {
	fn hitbox(&self) -> Hitbox {
		Hitbox::Circle {
			pos: self.pos,
			radius: 3.0,
		}
	}
}

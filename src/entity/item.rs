use crate::{
	ui::{Clickable, Hitbox},
	world::{GamePos, Mineral},
	Backend, BackendStyle, Colors,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemID(usize);
crate::make_id!(ItemID, Item);

#[derive(Debug)]
pub struct Item {
	pub id: ItemID,
	pub pos: GamePos,
	pub mineral: Mineral,
}

impl Item {
	const RADIUS: f32 = 3.0;

	pub fn new(id: ItemID, pos: GamePos, mineral: Mineral) -> Self {
		Self { id, pos, mineral }
	}

	pub fn draw(&self, backend: &mut Backend) {
		backend.fill_circle(
			self.pos,
			Item::RADIUS,
			match self.mineral {
				Mineral::Crystal => Colors::Crystal,
				Mineral::Ore => Colors::Ore,
			},
		);
	}

	pub fn draw_on_worker(&self, backend: &mut Backend, worker: Hitbox) {
		let (pos, size) = match worker {
			Hitbox::Rect { pos, size } => (pos, size),
			_ => panic!("Workers are Rectangles"),
		};
		backend.fill_circle(
			pos + size / 2.0,
			Item::RADIUS,
			match self.mineral {
				Mineral::Crystal => Colors::Crystal,
				Mineral::Ore => Colors::Ore,
			},
		);
	}
}

impl Clickable for Item {
	fn hitbox(&self) -> Hitbox {
		Hitbox::Circle {
			pos: self.pos,
			radius: Item::RADIUS,
		}
	}
	fn on_context_clicked(&mut self, item: usize) -> bool {
		panic!("Items don't have a Context Menu, but {} was clicked", item)
	}
}

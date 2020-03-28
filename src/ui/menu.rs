use crate::{
	entity::{Entities, ItemID, WorkerID},
	ui::Clickable,
	world::{GamePos, TilePos},
	Backend, BackendStyle, Colors, HashSet,
};

#[derive(Debug)]
pub struct Menu {
	pub selection: Selection,
}

impl Menu {
	pub const fn new() -> Self {
		Self {
			selection: Selection::Nothing,
		}
	}

	pub fn draw(&self, backend: &mut Backend, entities: &Entities) {
		use Selection::*;
		match &self.selection {
			Nothing => {}
			Workers(workers) => {
				for id in workers.iter() {
					backend.stroke_hitbox(entities.worker(*id).hitbox(), 1.0, Colors::Highlight);
				}
			}
			Item(id) => {
				backend.stroke_hitbox(entities.item(*id).hitbox(), 1.0, Colors::Highlight);
			}
			Walls(tiles) => {
				for pos in tiles.iter() {
					backend.stroke_rect(*pos, GamePos::TILE, 1.0, Colors::Highlight);
				}
			}
			Machine(pos) => backend.stroke_rect(*pos, GamePos::TILE, 1.0, Colors::Highlight),
			Air(pos) => backend.stroke_rect(*pos, GamePos::TILE, 1.0, Colors::Highlight),
		}
	}
}

#[derive(Debug)]
pub enum Selection {
	Nothing,
	Workers(HashSet<WorkerID>),
	Item(ItemID),
	Walls(HashSet<TilePos>),
	Machine(TilePos),
	Air(TilePos),
}

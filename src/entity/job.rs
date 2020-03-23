use super::{Entities, ItemID};
use crate::world::TilePos;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct JobID(usize);
crate::make_id!(JobID, Job);

#[derive(Debug, Clone)]
pub enum JobVariant {
	Destroy(TilePos),
	MoveTo(TilePos),
	PickUp(ItemID),
	BringTo(ItemID, TilePos),
}

use JobVariant::*;

#[derive(Debug)]
pub struct Job {
	id: JobID,
	variant: JobVariant,
}

impl Job {
	pub fn new(id: JobID, variant: JobVariant) -> Self {
		Self { id, variant }
	}

	pub fn get_target(&self, entities: &Entities) -> TilePos {
		match self.variant {
			Destroy(pos) => pos,
			MoveTo(pos) => pos,
			PickUp(item) => entities.item(item).pos.into(),
			BringTo(_, pos) => pos,
		}
	}
}

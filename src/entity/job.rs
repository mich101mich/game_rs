use super::ItemID;
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

#[derive(Debug)]
pub struct Job {
	id: JobID,
	variant: JobVariant,
}

impl Job {
	pub fn new(id: JobID, variant: JobVariant) -> Self {
		Self { id, variant }
	}
}

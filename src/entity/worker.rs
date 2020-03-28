use super::{Item, JobID};
use crate::{
	ui::{Clickable, Hitbox},
	world::{GamePos, Path, TilePos},
	Backend, BackendStyle, Colors, Game,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkerID(usize);
crate::make_id!(WorkerID, Worker);

#[derive(Debug)]
pub struct Worker {
	pub id: WorkerID,
	pub pos: TilePos,
	pub plan: Vec<JobID>,
	pub next_target: Option<(TilePos, Path)>,
	pub move_progress: Option<(TilePos, usize, usize)>,
	item: Option<Item>,
}

impl Worker {
	pub fn new(id: WorkerID, pos: TilePos) -> Self {
		Self {
			id,
			pos,
			plan: vec![],
			next_target: None,
			move_progress: None,
			item: None,
		}
	}

	pub fn draw(&self, backend: &mut Backend) {
		let hitbox = self.hitbox();
		backend.fill_hitbox(hitbox, Colors::Worker);
		if let Some(item) = self.item.as_ref() {
			item.draw_on_worker(backend, hitbox);
		}
	}
}

impl Clickable for Worker {
	fn hitbox(&self) -> Hitbox {
		let mut pos = GamePos::from(self.pos);

		if let Some((next_pos, progress, total)) = &self.move_progress {
			let percent = (*progress as f32 + Game::time().fract()) / *total as f32;

			let delta = (GamePos::from(*next_pos) - pos) * percent;
			pos += delta;
		}

		Hitbox::Rect {
			pos: pos + GamePos::new(2.0, 2.0),
			size: GamePos::new(12.0, 12.0),
		}
	}
	fn context_menu(&self) -> std::vec::Vec<(usize, String)> {
		unimplemented!()
	}
}

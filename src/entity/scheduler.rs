use super::*;
use crate::{world::World, HashSet};

pub struct Scheduler {
	dirty_workers: HashSet<WorkerID>,
	dirty_jobs: HashSet<JobID>,
}

impl Scheduler {
	pub fn new() -> Self {
		Scheduler {
			dirty_workers: HashSet::default(),
			dirty_jobs: HashSet::default(),
		}
	}

	pub fn update(&mut self, world: &mut World) {}

	pub fn worker_dirty(&mut self, id: WorkerID) {
		self.dirty_workers.insert(id);
	}
	pub fn job_dirty(&mut self, id: JobID) {
		self.dirty_jobs.insert(id);
	}
	pub fn item_dirty(&mut self, id: ItemID) {
	}
}

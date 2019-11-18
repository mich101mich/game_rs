use super::*;
use crate::Backend;
use vec_map::VecMap;

pub struct Scheduler {
	workers: VecMap<Worker>,
	jobs: VecMap<Job>,
}

impl Scheduler {
	pub fn new() -> Self {
		Scheduler {
			workers: VecMap::new(),
			jobs: VecMap::new(),
		}
	}

	pub fn add_worker(&mut self, pos: TilePos) {
		let len = self.workers.len();
		let id = (0..len)
			.find(|i| !self.workers.contains_key(*i))
			.unwrap_or(len);

		self.workers.insert(id, Worker::new(id, pos));
	}

	pub fn get_worker(&self, id: WorkerID) -> Option<&Worker> {
		self.workers.get(id.into())
	}
	pub fn get_worker_mut(&mut self, id: WorkerID) -> Option<&mut Worker> {
		self.workers.get_mut(id.into())
	}

	pub fn get_job(&self, id: JobID) -> Option<&Job> {
		self.jobs.get(id.into())
	}
	pub fn get_job_mut(&mut self, id: JobID) -> Option<&mut Job> {
		self.jobs.get_mut(id.into())
	}

	pub fn draw(&self, backend: &mut Backend) {
		for worker in self.workers.values() {
			worker.draw(backend);
		}
	}
}

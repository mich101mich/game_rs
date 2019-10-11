use super::*;
use crate::Backend;

pub struct Scheduler {
	pub workers: Vec<Worker>,
}

impl Scheduler {
	pub fn new() -> Self {
		Scheduler {
			workers: Vec::new(),
		}
	}

	pub fn add_worker(&mut self, pos: TilePos) {
		self.workers.push(Worker::new(pos));
	}

	pub fn draw(&self, backend: &mut Backend) {
		for worker in &self.workers {
			worker.draw(backend);
		}
	}
}

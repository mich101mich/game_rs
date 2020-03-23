use super::*;
use crate::{
	ui::{Clickable, Clicked},
	world::{GamePos, Mineral, TilePos},
	Backend,
};
use vec_map::VecMap;

pub struct Entities {
	workers: VecMap<Worker>,
	jobs: VecMap<Job>,
	items: VecMap<Item>,
}

impl Entities {
	pub fn new() -> Self {
		Self {
			workers: VecMap::new(),
			jobs: VecMap::new(),
			items: VecMap::new(),
		}
	}

	pub fn draw(&self, backend: &mut Backend) {
		for worker in self.workers.values() {
			worker.draw(backend);
		}
		for item in self.items.values() {
			item.draw(backend);
		}
	}

	pub fn add_worker(&mut self, pos: TilePos) {
		let len = self.workers.len();
		let id: WorkerID = (0..len)
			.find(|i| !self.workers.contains_key(*i))
			.unwrap_or(len)
			.into();

		self.workers.insert(id.into(), Worker::new(id, pos));
	}
	pub fn add_job(&mut self, variant: JobVariant) {
		let len = self.jobs.len();
		let id: JobID = (0..len)
			.find(|i| !self.jobs.contains_key(*i))
			.unwrap_or(len)
			.into();

		self.jobs.insert(id.into(), Job::new(id, variant));
	}
	pub fn add_item(&mut self, pos: GamePos, mineral: Mineral) {
		let len = self.items.len();
		let id: ItemID = (0..len)
			.find(|i| !self.items.contains_key(*i))
			.unwrap_or(len)
			.into();

		self.items.insert(id.into(), Item::new(id, pos, mineral));
	}

	pub fn entity_at(&self, pos: GamePos) -> Option<Clicked> {
		self.items
			.values()
			.find(|i| i.contains(pos))
			.map(|i| Clicked::Item(i.id))
			.or_else(|| {
				self.workers
					.values()
					.find(|w| w.contains(pos))
					.map(|w| Clicked::Worker(w.id))
			})
	}

	pub fn worker(&self, id: WorkerID) -> &Worker {
		&self.workers[usize::from(id)]
	}
	pub fn worker_mut(&mut self, id: WorkerID) -> &mut Worker {
		&mut self.workers[usize::from(id)]
	}
	pub fn worker_at(&self, pos: TilePos) -> Option<&Worker> {
		self.workers.values().find(|w| w.pos == pos)
	}
	pub fn worker_at_mut(&mut self, pos: TilePos) -> Option<&mut Worker> {
		self.workers.values_mut().find(|w| w.pos == pos)
	}
	pub fn workers(&self) -> vec_map::Values<Worker> {
		self.workers.values()
	}
	pub fn workers_mut(&mut self) -> vec_map::ValuesMut<Worker> {
		self.workers.values_mut()
	}

	pub fn job(&self, id: JobID) -> &Job {
		&self.jobs[usize::from(id)]
	}
	pub fn job_mut(&mut self, id: JobID) -> &mut Job {
		&mut self.jobs[usize::from(id)]
	}
	pub fn jobs(&self) -> vec_map::Values<Job> {
		self.jobs.values()
	}
	pub fn jobs_mut(&mut self) -> vec_map::ValuesMut<Job> {
		self.jobs.values_mut()
	}

	pub fn item(&self, id: ItemID) -> &Item {
		&self.items[usize::from(id)]
	}
	pub fn item_mut(&mut self, id: ItemID) -> &mut Item {
		&mut self.items[usize::from(id)]
	}
	pub fn items(&self) -> vec_map::Values<Item> {
		self.items.values()
	}
	pub fn items_mut(&mut self) -> vec_map::ValuesMut<Item> {
		self.items.values_mut()
	}
}

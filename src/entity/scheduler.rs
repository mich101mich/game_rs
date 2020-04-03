use super::*;
use crate::{world::World, HashSet};

pub struct Scheduler {
	known_workers: HashSet<WorkerID>,
	known_jobs: HashSet<JobID>,
	free_workers: HashSet<WorkerID>,
	free_jobs: HashSet<JobID>,
}

impl Scheduler {
	pub fn new() -> Self {
		Scheduler {
			known_workers: HashSet::default(),
			known_jobs: HashSet::default(),
			free_workers: HashSet::default(),
			free_jobs: HashSet::default(),
		}
	}

	pub fn update(&mut self, entities: &mut Entities, world: &mut World) {
		let mut marked_kill = vec![];
		for worker in entities.workers() {
			if worker.mark_killed {
				marked_kill.push(worker.id);
			}
		}
		for id in marked_kill {
			entities.remove_worker(id);
			self.known_workers.remove(&id);
			// TODO: remove Worker
		}

		for worker in entities.workers_mut() {
			// update between-tile movement
			if let Some((next_pos, mut progress, total)) = worker.move_progress.take() {
				progress += 1;
				if progress == total {
					worker.pos = next_pos;
					worker.move_progress = None;
				} else {
					worker.move_progress = Some((next_pos, progress, total));
				}
			}

			if worker.move_progress.is_none() {
				if let Some((_target, path)) = &mut worker.next_target {
					// TODO: check path valid
					if let Some(next) =
						path.next()
							.and_then(|p| if world.is_solid(p) { None } else { Some(p) })
					{
						let cost = world.walk_cost(worker.pos).expect("Worker on Solid Tile");
						worker.move_progress = Some((next.into(), 0, cost));
					} else {
						// TODO: Reached Goal
					}
				}
			}
		}

		let new_workers = entities
			.workers()
			.map(|w| w.id)
			.filter(|id| !self.known_workers.contains(&id))
			.collect::<Vec<_>>();

		for id in new_workers {
			self.known_workers.insert(id);
		}
	}
}

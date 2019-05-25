use super::{Dir, Grid, Machine, MachineType, Material, TilePos};
use hierarchical_pathfinding::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct World {
	grid: Grid,
	hpa_map: PathCache<ManhattanNeighborhood>,
	dirty: bool,
	changes: HashSet<TilePos>,
	machines: HashMap<TilePos, Machine>,
	spawns: HashSet<TilePos>,
}

impl World {
	pub fn new(width: usize, height: usize) -> Self {
		let grid = Grid::new(width, height);

		let neighborhood = ManhattanNeighborhood::new(width, height);
		let hpa_map = PathCache::new(
			(width, height),
			grid.cost_fn(),
			neighborhood,
			PathCacheConfig {
				..Default::default()
			},
		);

		World {
			grid,
			hpa_map,
			dirty: true,
			changes: HashSet::new(),
			machines: HashMap::new(),
			spawns: HashSet::new(),
		}
	}

	pub fn width(&self) -> usize {
		self.grid.size().x
	}
	pub fn height(&self) -> usize {
		self.grid.size().y
	}

	pub fn set_dirty(&mut self) {
		self.dirty = true;
	}

	pub fn set_p(&mut self, pos: TilePos, mat: Material) {
		self.set_dirty();
		self.changes.insert(pos);
		let old = self
			.grid
			.get_p(pos)
			.unwrap_or_else(|| panic!("Called set_p on invalid pos: {}", pos));

		use Material::*;
		match old {
			Machine | Platform => {
				self.machines
					.remove(&pos)
					.expect("Missing Machine")
					.remove();
			}
			_ => {}
		}

		self.grid.set_p(pos, mat);

		if mat == Material::Platform {
			self.machines
				.insert(pos, super::Machine::new(pos, MachineType::Platform));
		}
	}

	pub fn set_visible_p(&mut self, pos: TilePos) {
		self.set_dirty();
		self.grid.set_visible_p(pos)
	}

	pub fn draw(&mut self, backend: &mut crate::Backend) {
		use crate::BackendStyle;
		use Material::{Machine, Platform};

		if self.dirty {
			self.dirty = false;

			backend.clear_background();

			for y in 0..self.height() {
				for x in 0..self.width() {
					if self.grid.is_visible(x, y) {
						let mat = self.grid.get(x, y).expect("Grid size mismatch");
						let pos = super::TilePos::new(x, y);
						let (row, col) = if mat == Platform {
							let variant = Dir::all()
								.map(|dir| {
									self.grid
										.tile_in_dir(pos, dir)
										.and_then(|p| self.grid.get_p(p))
										.map(|mat| mat == Platform || mat == Machine)
										.unwrap_or(false) as usize
								})
								.rfold(0, |prev, cur| (prev << 1) | cur);
							(2, variant)
						} else {
							(0, mat as usize)
						};
						backend.draw_to_background((row, col), pos.into())
					}
				}
			}
		}

		backend.draw_background();

		for machine in self.machines.values() {
			machine.draw(backend);
		}
	}

	pub fn update(&mut self, spawn_has_power: bool) {
		let mut source_change = vec![];
		for machine in self.machines.values() {
			if let Some(change) = machine.power_source_changed(self) {
				source_change.push((machine.pos, change));
			}
		}
		for (pos, change) in source_change {
			self.machine_mut(pos).unwrap().set_power_source(change);
		}
		for machine in self.machines.values_mut() {
			machine.update(spawn_has_power);
		}
	}

	pub fn add_machine(&mut self, pos: TilePos, machine: MachineType) {
		if machine == MachineType::Spawn {
			self.spawns.insert(pos);
		}
		self.set_p(pos, Material::Machine);
		self.machines.insert(pos, Machine::new(pos, machine));
	}
	pub fn machine(&self, pos: TilePos) -> Option<&Machine> {
		self.machines.get(&pos)
	}
	pub fn machine_mut(&mut self, pos: TilePos) -> Option<&mut Machine> {
		self.machines.get_mut(&pos)
	}
}

impl std::ops::Deref for World {
	type Target = Grid;
	fn deref(&self) -> &Grid {
		&self.grid
	}
}

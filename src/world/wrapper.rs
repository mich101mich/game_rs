use super::{Dir, Grid, Machine, MachineType, Material, TilePos};
use crate::{HashMap, HashSet};
use hierarchical_pathfinding::{
	prelude::{ManhattanNeighborhood, PathCache, PathCacheConfig},
	AbstractPath,
};

pub type Neighborhood = ManhattanNeighborhood;
pub type Path = AbstractPath<Neighborhood>;

pub struct World {
	grid: Grid,
	hpa_map: PathCache<Neighborhood>,
	dirty: bool,
	changes: HashSet<TilePos>,
	machines: HashMap<TilePos, Machine>,
	spawns: HashSet<TilePos>,
}

impl World {
	pub fn new(width: usize, height: usize) -> Self {
		let grid = Grid::new(width, height);

		let neighborhood = Neighborhood::new(width, height);
		let hpa_map = PathCache::new(
			(width, height),
			grid.cost_fn(),
			neighborhood,
			PathCacheConfig {
				..Default::default()
			},
		);

		Self {
			grid,
			hpa_map,
			dirty: true,
			changes: HashSet::default(),
			machines: HashMap::default(),
			spawns: HashSet::default(),
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

	pub fn set<T: Into<TilePos>>(&mut self, pos: T, mat: Material) {
		let pos: TilePos = pos.into();
		self.set_dirty();
		self.changes.insert(pos);
		let old = self
			.grid
			.get(pos)
			.unwrap_or_else(|| panic!("Called set on invalid pos: {}", pos));

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

		self.grid[pos] = mat;

		if mat == Material::Platform {
			self.machines
				.insert(pos, super::Machine::new(pos, MachineType::Platform));
		}
	}

	pub fn set_visible_p(&mut self, pos: TilePos) {
		self.set_dirty();
		self.grid.set_visible(pos)
	}

	pub fn draw(&mut self, backend: &mut crate::Backend) {
		use crate::{BackendStyle, Color};
		use Material::{Machine, Platform};

		if self.dirty {
			self.dirty = false;

			backend.clear_background();

			for y in 0..self.height() {
				for x in 0..self.width() {
					if self.grid.is_visible((x, y)) {
						let mat = self.grid[(x, y)];
						let pos = super::TilePos::new(x, y);
						let (row, col) = if mat == Platform {
							let variant = Dir::all()
								.map(|dir| {
									self.grid
										.tile_in_dir(pos, dir)
										.and_then(|p| self.grid.get(p))
										.map(|mat| mat == Platform || mat == Machine)
										.unwrap_or(false) as usize
								})
								.rfold(0, |prev, cur| (prev << 1) | cur);
							(2, variant)
						} else {
							(0, mat as usize)
						};
						backend.draw_to_background((row, col), pos)
					}
				}
			}
		}

		backend.draw_background();

		for machine in self.machines.values() {
			machine.draw(backend);
		}

		// ============================= <Node Drawing> =============================

		if !self.changes.is_empty() {
			let tiles: Vec<_> = self.changes.iter().map(|p| (*p).into()).collect();
			self.changes.clear();
			self.hpa_map.tiles_changed(&tiles, self.grid.cost_fn());
		}

		// only draw the connections between Nodes once
		let mut visited = HashSet::default();
		use super::GamePos;
		let offset = super::TILE_SIZE as f32 / 2.0;
		let o = GamePos::new(offset, offset);

		{
			let chunk_size = self.hpa_map.config().chunk_size;
			let chunk_width = self.width() / chunk_size;
			let chunk_height = self.height() / chunk_size;
			let w = (self.width() * super::TILE_SIZE) as f32;
			let h = (self.height() * super::TILE_SIZE) as f32;
			for y in (1..chunk_height).map(|y| (y * chunk_size * super::TILE_SIZE) as f32) {
				backend.draw_line((0.0, y), (w, y), Color::rgb(255, 0, 0));
			}
			for x in (1..chunk_width).map(|x| (x * chunk_size * super::TILE_SIZE) as f32) {
				backend.draw_line((x, 0.0), (x, h), Color::rgb(255, 0, 0));
			}
		}

		for node in self.hpa_map.inspect_nodes() {
			let pos: GamePos = TilePos::from(node.pos()).into();
			backend.stroke_circle(
				pos + GamePos::TILE / 2.0,
				super::TILE_SIZE as f32 / 4.0,
				1.0,
				Color::rgba(255, 0, 0, 150),
			);

			visited.insert(node.id());

			for neighbor in node.connected().filter(|n| !visited.contains(&n.id())) {
				let other_pos: GamePos = TilePos::from(neighbor.pos()).into();

				backend.draw_line(pos + o, other_pos + o, Color::rgba(255, 0, 0, 150));
			}
		}

		// ============================= </Node Drawing> =============================
	}

	pub fn update(&mut self, spawn_has_power: bool) {
		let mut source_change = vec![];
		for machine in self.machines.values() {
			if let Some(change) = machine.power_source_changed(self) {
				source_change.push((machine.pos, change));
			}
		}
		for (pos, change) in source_change {
			self.machine_at_mut(pos).unwrap().set_power_source(change);
		}
		for machine in self.machines.values_mut() {
			machine.update(spawn_has_power);
		}
	}

	pub fn add_machine<T: Into<TilePos>>(&mut self, pos: T, machine: MachineType) {
		let pos: TilePos = pos.into();
		if machine == MachineType::Spawn {
			self.spawns.insert(pos);
		}
		self.set(pos, Material::Machine);
		self.machines.insert(pos, Machine::new(pos, machine));
	}
	pub fn machine_at(&self, pos: TilePos) -> Option<&Machine> {
		self.machines.get(&pos)
	}
	pub fn machine_at_mut(&mut self, pos: TilePos) -> Option<&mut Machine> {
		self.machines.get_mut(&pos)
	}
}

impl std::ops::Deref for World {
	type Target = Grid;
	fn deref(&self) -> &Grid {
		&self.grid
	}
}

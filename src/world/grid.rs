use super::{Dir, Material, TilePos};
use hierarchical_pathfinding::prelude::{ManhattanNeighborhood, Neighborhood};
use rand::Rng;

pub struct Grid {
	width: usize,
	height: usize,
	grid: Vec<Vec<Material>>,
	visible: Vec<Vec<bool>>,
	neighborhood: ManhattanNeighborhood,
}

impl Grid {
	pub fn new(width: usize, height: usize) -> Grid {
		let grid = std::iter::repeat(std::iter::repeat(Material::Bedrock).take(height).collect())
			.take(width)
			.collect();

		let visible = std::iter::repeat(std::iter::repeat(false).take(height).collect())
			.take(width)
			.collect();

		let mut ret = Grid {
			width,
			height,
			grid,
			visible,
			neighborhood: ManhattanNeighborhood::new(width, height),
		};
		ret.generate();

		ret
	}

	pub fn size(&self) -> TilePos {
		TilePos::new(self.width, self.height)
	}

	pub fn get(&self, x: usize, y: usize) -> Option<Material> {
		self.grid.get(x).and_then(|v| v.get(y).cloned())
	}
	pub fn set(&mut self, x: usize, y: usize, mat: Material) {
		self.grid[x][y] = mat
	}

	pub fn is_solid(&self, x: usize, y: usize) -> bool {
		match self.get(x, y) {
			Some(m) => m.is_solid(),
			None => true,
		}
	}

	pub fn walk_cost(&self, x: usize, y: usize) -> Option<usize> {
		self.get(x, y).and_then(Material::walk_cost)
	}

	pub fn is_visible(&self, x: usize, y: usize) -> bool {
		self.visible
			.get(x)
			.and_then(|v| v.get(y).cloned())
			.unwrap_or(false)
	}
	pub fn set_visible(&mut self, x: usize, y: usize) {
		if self.is_visible(x, y) {
			return;
		}

		self.visible[x][y] = true;

		if self.is_solid(x, y) {
			return;
		}

		let mut next = vec![TilePos::new(x, y)];

		while let Some(p) = next.pop() {
			for n in self.neighbors_of_p(p) {
				if self.is_visible_p(n) {
					continue;
				}
				self.visible[n.x][n.y] = true;
				if !self.is_solid_p(n) {
					next.push(n);
				}
			}
		}
	}

	pub fn neighbors_of(&self, x: usize, y: usize) -> impl Iterator<Item = TilePos> {
		self.neighborhood
			.get_all_neighbors((x, y))
			.map(TilePos::from)
	}

	pub fn get_p(&self, pos: TilePos) -> Option<Material> {
		self.get(pos.x, pos.y)
	}
	pub fn set_p(&mut self, pos: TilePos, mat: Material) {
		self.set(pos.x, pos.y, mat)
	}
	pub fn is_solid_p(&self, pos: TilePos) -> bool {
		self.is_solid(pos.x, pos.y)
	}
	pub fn walk_cost_p(&self, pos: TilePos) -> Option<usize> {
		self.walk_cost(pos.x, pos.y)
	}
	pub fn is_visible_p(&self, pos: TilePos) -> bool {
		self.is_visible(pos.x, pos.y)
	}
	pub fn set_visible_p(&mut self, pos: TilePos) {
		self.set_visible(pos.x, pos.y)
	}
	pub fn neighbors_of_p(&self, pos: TilePos) -> impl Iterator<Item = TilePos> {
		self.neighbors_of(pos.x, pos.y)
	}

	pub fn tile_in_dir(&self, pos: TilePos, dir: Dir) -> Option<TilePos> {
		self.tile_jump_in_dir(pos, dir, 1)
	}
	pub fn tile_jump_in_dir(&self, pos: TilePos, dir: Dir, dist: usize) -> Option<TilePos> {
		let dist = dist as isize;
		let (dx, dy) = dir.as_delta();
		let x = pos.x as isize + dist * dx;
		let y = pos.y as isize + dist * dy;
		if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
			Some(TilePos::new(x as usize, y as usize))
		} else {
			None
		}
	}

	pub fn cost_fn<'a>(&'a self) -> impl 'a + Fn((usize, usize)) -> isize {
		move |(x, y)| match self.walk_cost(x, y) {
			Some(cost) => cost as isize,
			None => -1,
		}
	}

	pub fn generate(&mut self) {
		use Material::*;

		let num_tiles = (self.width * self.height) as f64;

		let mut rng = rand::thread_rng();

		let min = num_tiles / 256.0;
		let max = num_tiles / 200.0;
		let cave_count = rng.gen_range(min, max) as usize;
		for _ in 0..cave_count {
			let x = rng.gen_range(0, self.width);
			let y = rng.gen_range(0, self.height);
			self.set(x, y, Air);
		}

		let radius = 2;
		let mid = self.size() / 2;
		for x in (mid.x - radius)..(mid.x + radius) {
			for y in (mid.y - radius)..(mid.y + radius) {
				if (mid.x as isize - x as isize).pow(2) + (mid.y as isize - y as isize).pow(2)
					< (radius * radius) as isize
				{
					self.set(x, y, Air);
				}
			}
		}

		self.grow(Air, Bedrock, Air, 1.0);
		self.grow(Air, Bedrock, Air, 1.0);
		self.grow(Air, Bedrock, Air, 0.8);
		self.grow(Air, Bedrock, Air, 0.6);
		self.grow(Air, Bedrock, Air, 0.4);
		self.grow(Air, Bedrock, Air, 0.4);

		self.grow(Rock, Bedrock, Air, 1.0);
		self.grow(Rock, Bedrock, Rock, 0.6);
		self.grow(Rock, Bedrock, Rock, 0.5);
		self.grow(Rock, Bedrock, Rock, 0.3);
		self.grow(Rock, Bedrock, Rock, 0.3);

		self.grow(Rock, Air, Rock, 0.2);

		self.grow(Granite, Bedrock, Rock, 0.8);
		self.grow(Granite, Bedrock, Granite, 0.4);
		self.grow(Granite, Bedrock, Granite, 0.4);
		self.grow(Granite, Rock, Rock, 0.03);
		self.grow(Granite, Rock, Granite, 0.1);

		let min = num_tiles / 64.0;
		let max = num_tiles / 48.0;
		let ore_count = rng.gen_range(min, max) as i32;
		for _ in 0..ore_count {
			let x = rng.gen_range(0, self.width);
			let y = rng.gen_range(0, self.height);
			if self.get(x, y) == Some(Rock) {
				self.set(x, y, Ore);
			}
		}

		self.grow(Ore, Rock, Ore, 0.23);
		self.grow(Ore, Rock, Ore, 0.23);

		let min = num_tiles / 80.0;
		let max = num_tiles / 64.0;
		let ore_count = rng.gen_range(min, max) as i32;
		for _ in 0..ore_count {
			let x = rng.gen_range(0, self.width);
			let y = rng.gen_range(0, self.height);
			if self.get(x, y) == Some(Rock) {
				self.set(x, y, Crystal);
			}
		}

		self.grow(Crystal, Rock, Crystal, 0.14);
		self.grow(Crystal, Rock, Crystal, 0.14);

		for x in 0..self.width {
			self.set(x, 0, Bedrock);
			self.set(x, self.height - 1, Bedrock);
		}
		for y in 0..self.height {
			self.set(0, y, Bedrock);
			self.set(self.width - 1, y, Bedrock);
		}

		self.set_visible_p(mid);
	}

	fn grow(&mut self, material: Material, src: Material, neighbor: Material, odd_increase: f64) {
		let mut changes = vec![];

		for x in 0..self.height {
			for y in 0..self.width {
				if self.get(x, y) != Some(src) {
					continue;
				}

				let odds: f64 = self
					.neighbors_of(x, y)
					.filter_map(|p| self.get_p(p))
					.filter(|m| *m == neighbor)
					.map(|_| odd_increase)
					.sum();

				if rand::random::<f64>() <= odds {
					changes.push((x, y));
				}
			}
		}
		for (x, y) in changes {
			self.set(x, y, material);
		}
	}
}

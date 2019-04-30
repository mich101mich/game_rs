use super::{Material, TilePos};
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

		Grid {
			width,
			height,
			grid,
			visible,
			neighborhood: ManhattanNeighborhood::new(width, height),
		}
	}

	pub fn size(&self) -> TilePos {
		TilePos::new(self.width, self.height)
	}

	pub fn get(&self, pos: TilePos) -> Material {
		self.grid[pos.x][pos.y]
	}
	pub fn set(&mut self, pos: TilePos, mat: Material) {
		self.grid[pos.x][pos.y] = mat
	}

	pub fn is_solid(&self, pos: TilePos) -> bool {
		self.get(pos).is_solid()
	}
	pub fn walk_cost(&self, pos: TilePos) -> Option<usize> {
		self.get(pos).walk_cost()
	}

	pub fn is_visible(&self, pos: TilePos) -> bool {
		self.visible[pos.x][pos.y]
	}
	pub fn set_visible(&mut self, pos: TilePos) {
		if self.is_visible(pos) {
			return;
		}

		self.visible[pos.x][pos.y] = true;

		if self.is_solid(pos) {
			return;
		}

		let mut next = vec![pos];

		while let Some(p) = next.pop() {
			for n in self.neighbors_of(p) {
				self.visible[n.x][n.y] = true;
				if !self.is_solid(n) {
					next.push(n);
				}
			}
		}
	}

	pub fn neighbors_of(&self, pos: TilePos) -> impl Iterator<Item = TilePos> {
		self.neighborhood
			.get_all_neighbors((pos.x, pos.y))
			.map(TilePos::from)
	}

	pub fn cost_fn<'a>(&'a self) -> impl 'a + Fn((usize, usize)) -> isize {
		move |(x, y)| match self.grid[y][x].walk_cost() {
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
			self.grid[x][y] = Air;
		}

		let radius = 2;
		let mid = self.size() / 2;
		for x in (mid.x - radius)..(mid.x + radius) {
			for y in (mid.y - radius)..(mid.y + radius) {
				if (mid.x - x).pow(2) + (mid.y - y).pow(2) < radius * radius {
					self.grid[x][y] = Air;
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
			if self.grid[x][y] == Rock {
				self.grid[x][y] = Ore;
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
			if self.grid[x][y] == Rock {
				self.grid[x][y] = Crystal;
			}
		}

		self.grow(Crystal, Rock, Crystal, 0.14);
		self.grow(Crystal, Rock, Crystal, 0.14);

		for x in 0..self.width {
			self.grid[x][0] = Bedrock;
			self.grid[x][self.height - 1] = Bedrock;
		}
		for y in 0..self.height {
			self.grid[0][y] = Bedrock;
			self.grid[self.width - 1][y] = Bedrock;
		}

		self.set_visible(mid);
	}

	fn grow(&mut self, material: Material, src: Material, neighbor: Material, odd_increase: f64) {
		let mut changes = vec![];

		for x in 0..self.height {
			for y in 0..self.width {
				if self.grid[x][y] != src {
					continue;
				}

				let odds: f64 = self
					.neighbors_of((x, y).into())
					.filter(|p| self.get(*p) == neighbor)
					.map(|_| odd_increase)
					.sum();

				if rand::random::<f64>() <= odds {
					changes.push((x, y));
				}
			}
		}
		for (x, y) in changes {
			self.grid[x][y] = material;
		}
	}
}

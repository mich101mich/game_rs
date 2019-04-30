use super::Grid;
use hierarchical_pathfinding::prelude::*;

pub struct World {
	pub grid: Grid,
	hpa_map: PathCache<ManhattanNeighborhood>,
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

		World { grid, hpa_map }
	}

	pub fn width(&self) -> usize {
		self.grid.size().x
	}
	pub fn height(&self) -> usize {
		self.grid.size().y
	}
}

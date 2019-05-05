use super::{Dir, Grid, Material, TilePos};
use hierarchical_pathfinding::prelude::*;

pub struct World {
	grid: Grid,
	hpa_map: PathCache<ManhattanNeighborhood>,
	dirty: bool,
	changes: Vec<TilePos>,
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
			changes: vec![],
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
		self.changes.push(pos);
		self.grid.set_p(pos, mat)
	}

	pub fn set_visible_p(&mut self, pos: TilePos) {
		self.set_dirty();
		self.grid.set_visible_p(pos)
	}

	pub fn draw(&mut self, backend: &mut crate::Backend) {
		use crate::BackendStyle;
		use Material::Platform;

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
										.map(|p| self.grid.get_p(p) == Some(Platform))
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
	}
}

impl std::ops::Deref for World {
	type Target = Grid;
	fn deref(&self) -> &Grid {
		&self.grid
	}
}

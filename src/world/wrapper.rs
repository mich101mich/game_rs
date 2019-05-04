use super::{Dir, Grid, Material};
use hierarchical_pathfinding::prelude::*;

pub struct World {
	grid: Grid,
	hpa_map: PathCache<ManhattanNeighborhood>,
	dirty: bool,
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
						let (row, col) = if mat == Platform {
							let pos = super::TilePos::new(x, y);
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
						backend.draw_to_background((row, col), (x as f32 * 16.0, y as f32 * 16.0))
					}
				}
			}
		}

		backend.draw_background();
	}
}

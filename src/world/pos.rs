
pub const GAME_SCALE: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct GamePos {
	pub x: f32,
	pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub struct TilePos {
	pub x: usize,
	pub y: usize,
}

impl GamePos {
	pub fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}
}

impl TilePos {
	pub fn new(x: usize, y: usize) -> Self {
		Self { x, y }
	}
}

use std::ops::*;

macro_rules! impl_op {
	($target: ty, $trait: ty, $f: ident, $op: tt) => {
		impl $trait for $target {
			type Output = Self;
			fn $f(self, rhs: Self) -> Self {
				Self {
					x: self.x $op rhs.x,
					y: self.y $op rhs.y,
				}
			}
		}
	};
	($target: ty, $trait1: ty, $rhs: ty, $trait2: ty, $f: ident, $op: tt) => {
		impl $trait1 for $target {
			type Output = Self;
			fn $f(self, rhs: $rhs) -> Self {
				Self {
					x: self.x $op rhs,
					y: self.y $op rhs,
				}
			}
		}
		impl $trait2 for $rhs {
			type Output = $target;
			fn $f(self, rhs: $target) -> $target {
				Self::Output {
					x: self $op rhs.x,
					y: self $op rhs.y,
				}
			}
		}
	};
}

impl_op!(GamePos, Add, add, +);
impl_op!(GamePos, Sub, sub, -);
impl_op!(GamePos, Mul<f32>, f32, Mul<GamePos>, mul, *);
impl_op!(GamePos, Div<f32>, f32, Div<GamePos>, div, /);

impl_op!(TilePos, Add, add, +);
impl_op!(TilePos, Sub, sub, -);
impl_op!(TilePos, Mul<usize>, usize, Mul<TilePos>, mul, *);
impl_op!(TilePos, Div<usize>, usize, Div<TilePos>, div, /);

macro_rules! quick_impl {
	(From<$src: ty> for $dest: ty: $pattern: pat => $result: expr) => {
		impl From<$src> for $dest {
			fn from($pattern: $src) -> Self {
				$result
			}
		}
	};
}

quick_impl!(From<(f32, f32)> for GamePos: (x, y) => GamePos {x, y});
quick_impl!(From<[f32; 2]> for GamePos: [x, y] => GamePos {x, y});
quick_impl!(From<GamePos> for (f32, f32): GamePos {x, y} => (x, y));
quick_impl!(From<GamePos> for [f32; 2]: GamePos {x, y} => [x, y]);

quick_impl!(From<(f64, f64)> for GamePos: (x, y) => GamePos {x: x as f32, y: y as f32});
quick_impl!(From<[f64; 2]> for GamePos: [x, y] => GamePos {x: x as f32, y: y as f32});
quick_impl!(From<GamePos> for (f64, f64): GamePos {x, y} => (x as f64, y as f64));
quick_impl!(From<GamePos> for [f64; 2]: GamePos {x, y} => [x as f64, y as f64]);

quick_impl!(From<(usize, usize)> for TilePos: (x, y) => TilePos {x, y});
quick_impl!(From<[usize; 2]> for TilePos: [x, y] => TilePos {x, y});
quick_impl!(From<TilePos> for (usize, usize): TilePos {x, y} => (x, y));
quick_impl!(From<TilePos> for [usize; 2]: TilePos {x, y} => [x, y]);

quick_impl!(From<TilePos> for GamePos: TilePos {x, y} => GamePos {x: (x * GAME_SCALE) as f32, y: (y * GAME_SCALE) as f32});
quick_impl!(From<GamePos> for TilePos: GamePos {x, y} => TilePos {x: x as usize / GAME_SCALE, y: y as usize / GAME_SCALE});

use std::fmt;
impl fmt::Display for GamePos {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "G({:.1}, {:.1})", self.x, self.y)
	}
}

impl fmt::Display for TilePos {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "T({}, {})", self.x, self.y)
	}
}

pub const TILE_SIZE: usize = 16;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct GamePos {
	pub x: f32,
	pub y: f32,
}

#[derive(Debug, Clone, Copy, Hash, Default, PartialEq, Eq)]
pub struct TilePos {
	pub x: usize,
	pub y: usize,
}

impl GamePos {
	pub const UNIT: GamePos = GamePos { x: 1.0, y: 1.0 };
	pub const TILE: GamePos = GamePos {
		x: TILE_SIZE as f32,
		y: TILE_SIZE as f32,
	};

	pub fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}
	pub fn dist(self, other: GamePos) -> f32 {
		self.dist_sq(other).sqrt()
	}
	pub fn dist_sq(self, other: GamePos) -> f32 {
		(self - other).norm_sq()
	}
	pub fn norm(self) -> f32 {
		self.norm_sq().sqrt()
	}
	pub fn norm_sq(self) -> f32 {
		self * self
	}
}

impl TilePos {
	pub const UNIT: TilePos = TilePos { x: 1, y: 1 };

	pub fn new(x: usize, y: usize) -> Self {
		Self { x, y }
	}
	pub fn dist(self, other: TilePos) -> usize {
		let dx = if self.x < other.x {
			other.x - self.x
		} else {
			self.x - other.x
		};
		let dy = if self.y < other.y {
			other.y - self.y
		} else {
			self.y - other.y
		};
		dx + dy
	}
	pub fn rect_iter(self, other: TilePos) -> RectIter {
		let tl = TilePos::new(self.x.min(other.x), self.y.min(other.y));
		let br = TilePos::new(self.x.max(other.x), self.y.max(other.y));
		RectIter {
			tl,
			br,
			current: tl,
		}
	}
}
pub struct RectIter {
	tl: TilePos,
	br: TilePos,
	current: TilePos,
}
impl Iterator for RectIter {
	type Item = TilePos;
	fn next(&mut self) -> Option<Self::Item> {
		let ret = Some(self.current);
		if self.current.x == self.br.x {
			if self.current.y == self.br.y {
				return None;
			}
			self.current.x = self.tl.x;
			self.current.y += 1;
		} else {
			self.current.x += 1;
		}
		ret
	}
}

use std::ops::*;

impl Mul<GamePos> for GamePos {
	type Output = f32;
	fn mul(self, other: GamePos) -> Self::Output {
		self.x * other.x + self.y * other.y
	}
}

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

macro_rules! impl_assign {
	($target: ty, $trait: ty, $rhs: ty, $f: ident, $op: tt) => {
		impl $trait for $target {
			fn $f(&mut self, rhs: $rhs) {
				*self = *self $op rhs
			}
		}
	};
}

impl_op!(GamePos, Add, add, +);
impl_op!(GamePos, Sub, sub, -);
impl_op!(GamePos, Mul<f32>, f32, Mul<GamePos>, mul, *);
impl_op!(GamePos, Div<f32>, f32, Div<GamePos>, div, /);
impl_assign!(GamePos, AddAssign, GamePos, add_assign, +);
impl_assign!(GamePos, SubAssign, GamePos, sub_assign, -);
impl_assign!(GamePos, MulAssign<f32>, f32, mul_assign, *);
impl_assign!(GamePos, DivAssign<f32>, f32, div_assign, /);

impl Neg for GamePos {
	type Output = Self;
	fn neg(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
		}
	}
}

impl_op!(TilePos, Add, add, +);
impl_op!(TilePos, Sub, sub, -);
impl_op!(TilePos, Mul<usize>, usize, Mul<TilePos>, mul, *);
impl_op!(TilePos, Div<usize>, usize, Div<TilePos>, div, /);
impl_assign!(TilePos, AddAssign, TilePos, add_assign, +);
impl_assign!(TilePos, SubAssign, TilePos, sub_assign, -);
impl_assign!(TilePos, MulAssign<usize>, usize, mul_assign, *);
impl_assign!(TilePos, DivAssign<usize>, usize, div_assign, /);

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
quick_impl!(From<GamePos> for (f64, f64): GamePos {x, y} => (f64::from(x), f64::from(y)));
quick_impl!(From<GamePos> for [f64; 2]: GamePos {x, y} => [f64::from(x), f64::from(y)]);

quick_impl!(From<(usize, usize)> for TilePos: (x, y) => TilePos {x, y});
quick_impl!(From<[usize; 2]> for TilePos: [x, y] => TilePos {x, y});
quick_impl!(From<TilePos> for (usize, usize): TilePos {x, y} => (x, y));
quick_impl!(From<TilePos> for [usize; 2]: TilePos {x, y} => [x, y]);

quick_impl!(From<TilePos> for GamePos: TilePos {x, y} => GamePos {x: (x * TILE_SIZE) as f32, y: (y * TILE_SIZE) as f32});
quick_impl!(From<GamePos> for TilePos: GamePos {x, y} => TilePos {x: x as usize / TILE_SIZE, y: y as usize / TILE_SIZE});

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

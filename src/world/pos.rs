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

impl From<(f32, f32)> for GamePos {
	fn from((x, y): (f32, f32)) -> Self {
		Self { x, y }
	}
}
impl From<[f32; 2]> for GamePos {
	fn from([x, y]: [f32; 2]) -> Self {
		Self { x, y }
	}
}
impl From<(usize, usize)> for TilePos {
	fn from((x, y): (usize, usize)) -> Self {
		Self { x, y }
	}
}
impl From<[usize; 2]> for TilePos {
	fn from([x, y]: [usize; 2]) -> Self {
		Self { x, y }
	}
}

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

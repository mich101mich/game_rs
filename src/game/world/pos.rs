#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct GamePos {
	x: f32,
	y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub struct TilePos {
	x: u32,
	y: u32,
}

impl GamePos {
	pub fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}
}

impl TilePos {
	pub fn new(x: u32, y: u32) -> Self {
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
impl_op!(TilePos, Mul<u32>, u32, Mul<TilePos>, mul, *);
impl_op!(TilePos, Div<u32>, u32, Div<TilePos>, div, /);

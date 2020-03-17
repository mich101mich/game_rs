#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
	Up,
	Right,
	Down,
	Left,
}
pub use Dir::*;

impl Dir {
	pub fn clockwise(self) -> Dir {
		((self.num() + 1) % 4).into()
	}
	pub fn counter_clockwise(self) -> Dir {
		((self.num() + 3) % 4).into()
	}
	pub fn num(self) -> usize {
		self.into()
	}
	pub fn all() -> impl DoubleEndedIterator<Item = Dir> {
		[Up, Right, Down, Left].iter().copied()
	}
	pub fn as_delta(self) -> (isize, isize) {
		[(0, -1), (1, 0), (0, 1), (-1, 0)][self.num()]
	}
}

macro_rules! impl_from_into {
	($type:ty) => {
		impl From<$type> for Dir {
			fn from(val: $type) -> Dir {
				match val {
					0 => Up,
					1 => Right,
					2 => Down,
					3 => Left,
					n => panic!("Invalid Dir value: {}", n),
				}
			}
		}
		impl Into<$type> for Dir {
			fn into(self) -> $type {
				self as $type
			}
		}
	};
}

impl_from_into!(u8);
impl_from_into!(u16);
impl_from_into!(u32);
impl_from_into!(u64);
impl_from_into!(usize);
impl_from_into!(i8);
impl_from_into!(i16);
impl_from_into!(i32);
impl_from_into!(i64);
impl_from_into!(isize);

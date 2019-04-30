#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dir {
	UP,
	RIGHT,
	DOWN,
	LEFT,
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
}

macro_rules! impl_from_into {
	($type:ty) => {
		impl From<$type> for Dir {
			fn from(val: $type) -> Dir {
				match val {
					0 => UP,
					1 => RIGHT,
					2 => DOWN,
					3 => LEFT,
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

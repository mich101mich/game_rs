use crate::{
	entity::{ItemID, WorkerID},
	world::GamePos,
};

pub trait Clickable {
	fn hitbox(&self) -> Hitbox;
	fn context_menu(&self) -> Vec<(usize, String)>;

	fn intersects<O: Clickable>(&self, other: &O) -> bool {
		self.hitbox().intersects(other.hitbox())
	}
	fn contains(&self, point: GamePos) -> bool {
		self.hitbox().contains(point)
	}
}

pub enum Clicked {
	Worker(WorkerID),
	Item(ItemID),
}

#[derive(Debug, Clone, Copy)]
pub enum Hitbox {
	Rect { pos: GamePos, size: GamePos },
	Circle { pos: GamePos, radius: f32 },
}

use Hitbox::*;

impl Hitbox {
	pub fn intersects(self, other: Hitbox) -> bool {
		match self {
			Rect { pos, size } => {
				let (a_left, a_top): (f32, f32) = pos.into();
				let (a_right, a_bottom): (f32, f32) = (pos + size).into();
				match other {
					Rect { pos, size } => {
						let (b_left, b_top): (f32, f32) = pos.into();
						let (b_right, b_bottom): (f32, f32) = (pos + size).into();
						a_left < b_right && a_right > b_left && a_top < b_bottom && a_bottom > b_top
					}
					Circle { pos, .. } => {
						let nearest_x = f32::max(a_left, f32::min(pos.x, a_right));
						let nearest_y = f32::max(a_top, f32::min(pos.y, a_bottom));
						other.contains((nearest_x, nearest_y).into())
					}
				}
			}
			Circle { pos, radius } => {
				let (pa, ra) = (pos, radius);
				match other {
					Rect { .. } => other.intersects(self),
					Circle { pos, radius } => pos.dist(pa) < radius + ra,
				}
			}
		}
	}

	pub fn contains(self, point: GamePos) -> bool {
		match self {
			Rect { pos, size } => {
				point.x >= pos.x
					&& point.x <= pos.x + size.x
					&& point.y >= pos.y && point.y <= pos.y + size.y
			}
			Circle { pos, radius } => pos.dist_sq(point) <= radius * radius,
		}
	}
}

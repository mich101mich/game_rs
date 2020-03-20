use crate::world::GamePos;

#[derive(Debug, Clone, Copy)]
pub enum Hitbox {
	Rect { pos: GamePos, size: GamePos },
	Circle { pos: GamePos, radius: f32 },
}

use Hitbox::*;

impl Hitbox {
	fn intersects(self, other: Hitbox) -> bool {
		match self {
			Rect { pos, size } => {
				let (a_left, a_top): (f32, f32) = pos.into();
				let (a_right, a_bottom): (f32, f32) = (pos + size).into();
				match other {
					Rect { pos, size } => {
						let (b_left, b_top): (f32, f32) = pos.into();
						let (b_right, b_bottom): (f32, f32) = (pos + size).into();
						a_left < b_right && a_right > b_left && a_top > b_bottom && a_bottom < b_top
					}
					Circle { pos, .. } => {
						let nearest_x = f32::max(a_left, f32::min(pos.x, a_right));
						let nearest_y = f32::max(a_top, f32::min(pos.y, a_bottom));
						other.inside((nearest_x, nearest_y).into())
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

	fn inside(self, point: GamePos) -> bool {
		match self {
			Rect { pos, size } => {
				let (left, top): (f32, f32) = pos.into();
				let (right, bottom): (f32, f32) = (pos + size).into();
				point.x >= left && point.x <= right && point.y >= top && point.y <= bottom
			}
			Circle { pos, radius } => pos.dist_sq(point) <= radius * radius,
		}
	}
}

pub trait Clickable {
	fn hitbox(&self) -> Hitbox;
	fn intersects<O: Clickable>(&self, other: &O) -> bool {
		self.hitbox().intersects(other.hitbox())
	}
	fn inside(&self, point: GamePos) -> bool {
		self.hitbox().inside(point)
	}
}

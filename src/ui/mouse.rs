use super::ButtonState;
use crate::{world::GamePos, Backend, BackendStyle, Color};

#[derive(Debug, Default)]
pub struct Mouse {
	pos: GamePos,
	start_pos: Option<GamePos>,
	brush_size: f32,

	left: ButtonState,
	right: ButtonState,
	shift: ButtonState,
	ctrl: ButtonState,

	scale: f32,
	offset: GamePos,
}

impl Mouse {
	pub fn new() -> Mouse {
		Mouse {
			scale: 1.0,
			brush_size: 30.0,
			..Default::default()
		}
	}

	pub fn on_event(&mut self, event: MouseEvent) {
		use MouseEvent::*;
		match event {
			Move(delta) => {
				if self.left_down() {
					if self.shift_down() {
						// TODO: add to selection
					} else if !self.ctrl_down() || self.start_pos.is_none() {
						let moved = delta / self.scale;
						self.offset += moved;
					}
				}
				self.pos += delta;
			}
			ClickDown(button) => {
				self.set_button(button, ButtonState::Down);
				if button == MouseButton::Left && self.ctrl_down() {
					self.start_pos = Some(self.pos);
				}
			}
			ClickUp(button) => {
				self.set_button(button, ButtonState::Up);
				if button == MouseButton::Left && self.ctrl_down() {
					self.start_pos = None;
					// TODO: handle selection
				}
			}
			Scroll(delta) => {
				let factor = 1.0 - delta / 10.0;
				self.scale *= factor;

				self.offset -= self.pos / (self.scale / factor) - self.pos / self.scale;
			}
		}
	}

	pub fn screen_to_world(&self, pos: GamePos) -> GamePos {
		pos / self.scale - self.offset
	}

	pub fn draw(&self, backend: &mut Backend) {
		let pos = self.screen_to_world(self.pos);
		let start = self.start_pos.map(|p| self.screen_to_world(p));

		const SELECT_COLOR: Color = Color {
			r: 180,
			g: 180,
			b: 255,
			a: 180,
		};

		if self.shift_down() {
			backend.fill_circle(pos, self.brush_size, SELECT_COLOR);
		} else if self.ctrl_down() {
			if let Some(start) = start {
				let tl = GamePos::new(pos.x.min(start.x), pos.y.min(start.y));
				let br = GamePos::new(pos.x.max(start.x), pos.y.max(start.y));
				backend.fill_rect(
					tl,
					br - tl,
					SELECT_COLOR,
				);
			}
		}
	}

	pub fn set_shift(&mut self, shift: ButtonState) {
		self.shift = shift;
	}
	pub fn set_ctrl(&mut self, ctrl: ButtonState) {
		self.ctrl = ctrl;
		if ctrl == ButtonState::Up {
			self.start_pos = None;
		}
	}
	pub fn set_button(&mut self, button: MouseButton, state: ButtonState) {
		match button {
			MouseButton::Left => {
				self.left = state;
			}
			MouseButton::Right => {
				self.right = state;
			}
		}
	}
	pub fn left(&self) -> ButtonState {
		self.left
	}
	pub fn left_down(&self) -> bool {
		self.left == ButtonState::Down
	}
	pub fn right(&self) -> ButtonState {
		self.right
	}
	pub fn right_down(&self) -> bool {
		self.right == ButtonState::Down
	}
	pub fn shift(&self) -> ButtonState {
		self.shift
	}
	pub fn shift_down(&self) -> bool {
		self.shift == ButtonState::Down
	}
	pub fn ctrl(&self) -> ButtonState {
		self.ctrl
	}
	pub fn ctrl_down(&self) -> bool {
		self.ctrl == ButtonState::Down
	}

	pub fn scale(&self) -> f32 {
		self.scale
	}
	pub fn offset(&self) -> GamePos {
		self.offset
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
	Left,
	Right,
}

#[derive(Debug)]
pub enum MouseEvent {
	Move(GamePos),
	ClickDown(MouseButton),
	ClickUp(MouseButton),
	Scroll(f32),
}

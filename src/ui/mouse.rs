use super::ButtonState;
use crate::{world::GamePos, Backend, BackendStyle, Colors};

#[derive(Debug, Default)]
pub struct Mouse {
	pos: GamePos,
	start_pos: Option<GamePos>,
	brush_size: f32,
	is_single_click: bool,

	left: ButtonState,
	right: ButtonState,
	shift: ButtonState,
	ctrl: ButtonState,

	pub scale: f32,
	offset: GamePos,
}

impl Mouse {
	pub fn new() -> Self {
		Self {
			scale: 1.0,
			brush_size: 30.0,
			..Default::default()
		}
	}

	pub fn on_event(&mut self, event: MouseEvent) -> SelectionInfo {
		use MouseEvent::*;
		match event {
			Move(delta) => {
				self.pos += delta;
				self.is_single_click = false;
				if self.left_down() {
					if self.brush_mode() {
						return SelectionInfo::Brush(self.pos_world(), self.brush_size, true);
					} else if !self.area_mode() || self.start_pos.is_none() {
						let moved = delta / self.scale;
						self.offset += moved;
					}
				}
				SelectionInfo::NoChange
			}
			ClickDown(MouseButton::Left) => {
				self.set_button(MouseButton::Left, ButtonState::Down);
				self.is_single_click = true;
				if self.brush_mode() {
					return SelectionInfo::Brush(self.pos_world(), self.brush_size, false);
				} else if self.area_mode() {
					self.start_pos = Some(self.pos_world());
				}
				SelectionInfo::NoChange
			}
			ClickUp(MouseButton::Left) => {
				self.set_button(MouseButton::Left, ButtonState::Up);
				if self.is_single_click && !self.brush_mode() {
					return SelectionInfo::Click(self.pos_world());
				}
				if self.ctrl_down() {
					if let Some(start_pos) = self.start_pos.take() {
						let pos = self.pos_world();
						let tl = (start_pos.x.min(pos.x), start_pos.y.min(pos.y)).into();
						let br = (start_pos.x.max(pos.x), start_pos.y.max(pos.y)).into();
						return SelectionInfo::Area(tl, br);
					}
				}
				SelectionInfo::NoChange
			}
			Scroll(delta) => {
				let factor = 1.0 - delta / 10.0;
				self.scale *= factor;

				self.offset -= self.pos / (self.scale / factor) - self.pos / self.scale;
				SelectionInfo::NoChange
			}
			ClickDown(MouseButton::Right) => {
				self.set_button(MouseButton::Right, ButtonState::Down);
				// right mouse ignored for now
				SelectionInfo::NoChange
			}
			ClickUp(MouseButton::Right) => {
				self.set_button(MouseButton::Right, ButtonState::Up);
				// right mouse ignored for now
				SelectionInfo::NoChange
			}
		}
	}

	pub fn pos_world(&self) -> GamePos {
		self.screen_to_world(self.pos)
	}
	pub fn screen_to_world(&self, pos: GamePos) -> GamePos {
		pos / self.scale - self.offset
	}

	pub fn draw(&self, backend: &mut Backend) {
		let pos = self.pos_world();

		if self.shift_down() {
			backend.fill_circle(pos, self.brush_size, Colors::Cursor);
		} else if self.ctrl_down() {
			if let Some(start) = self.start_pos {
				let tl = GamePos::new(pos.x.min(start.x), pos.y.min(start.y));
				let br = GamePos::new(pos.x.max(start.x), pos.y.max(start.y));
				backend.fill_rect(tl, br - tl, Colors::Cursor);
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

	pub fn brush_mode(&self) -> bool {
		self.shift_down()
	}
	pub fn area_mode(&self) -> bool {
		self.ctrl_down() && !self.brush_mode()
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

#[derive(Debug)]
pub enum SelectionInfo {
	NoChange,
	Click(GamePos),
	Brush(GamePos, f32, bool),
	Area(GamePos, GamePos),
}

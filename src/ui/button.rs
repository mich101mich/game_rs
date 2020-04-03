use crate::{
	ui::{Clickable, Hitbox},
	world::GamePos,
	Backend, BackendStyle, Colors,
};

#[derive(Debug)]
pub struct Button<T> {
	pub identifier: T,
	pub pos: GamePos,
	pub size: GamePos,
	text: String,
}

impl<T> Button<T> {
	pub fn new(identifier: T, text: String, pos: GamePos, size: GamePos) -> Self {
		Self {
			identifier,
			pos,
			size,
			text,
		}
	}

	pub fn draw(&self, backend: &mut Backend) {
		backend.absolute_mode(true);
		backend.fill_rect(self.pos, self.size, Colors::Button);
		backend.draw_text(&self.text, self.pos + GamePos::UNIT, Colors::Black);
		backend.absolute_mode(false);
	}
}

impl<T> Clickable for Button<T> {
	fn hitbox(&self) -> Hitbox {
		Hitbox::Rect {
			pos: self.pos,
			size: self.size,
		}
	}
	fn context_menu(&self) -> Option<Vec<(usize, String)>> {
		None
	}
	fn on_context_clicked(&mut self, item: usize) -> bool {
		panic!(
			"Buttons don't have a Context Menu, but {} was clicked",
			item
		)
	}
}

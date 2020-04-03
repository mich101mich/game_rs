#[cfg(target_arch = "wasm32")]
mod web_backend;
#[cfg(target_arch = "wasm32")]
pub use web_backend::{Backend, Color};

#[cfg(not(target_arch = "wasm32"))]
mod window_backend;
#[cfg(not(target_arch = "wasm32"))]
pub use window_backend::{Backend, Color};

use crate::{ui::Hitbox, world::GamePos, Game};

pub const TEXT_SIZE: usize = 16;

#[derive(Debug, Clone, Copy)]
pub enum Colors {
	Background = 0,
	Crystal,
	Ore,
	Worker,
	NoPower,
	Chunk,
	Node,
	Highlight,
	Cursor,
	Button,
	Black,
}
impl Colors {
	pub fn num(self) -> i32 {
		self as i32
	}
}
impl From<Colors> for Color {
	fn from(color: Colors) -> Self {
		match color {
			Colors::Background => Color::rgb(128, 128, 128),
			Colors::Crystal => Color::rgb(87, 255, 23),
			Colors::Ore => Color::rgb(165, 110, 31),
			Colors::Worker => Color::rgb(250, 191, 15),
			Colors::NoPower => Color::rgba(0, 0, 0, 140),
			Colors::Chunk => Color::rgb(255, 0, 0),
			Colors::Node => Color::rgba(255, 0, 0, 150),
			Colors::Highlight => Color::rgb(255, 0, 0),
			Colors::Cursor => Color::rgba(180, 180, 255, 180),
			Colors::Button => Color::rgb(128, 128, 128),
			Colors::Black => Color::rgb(0, 0, 0),
		}
	}
}

pub trait BackendStyle {
	fn start(game: Game);

	/// Returns the Width of the Screen in Pixels
	fn get_width(&self) -> u32;
	/// Returns the Height of the Screen in Pixels
	fn get_height(&self) -> u32;

	/// Fill the entire Screen with a single Color
	fn fill(&mut self, color: Colors);

	fn absolute_mode(&mut self, on: bool);

	/// Draw a Line from `start` to `end`
	fn draw_line<T: Into<GamePos>, T2: Into<GamePos>>(&mut self, start: T, end: T2, color: Colors);
	/// Fill a Hitbox
	fn fill_hitbox(&mut self, hitbox: Hitbox, color: Colors) {
		match hitbox {
			Hitbox::Rect { pos, size } => self.fill_rect(pos, size, color),
			Hitbox::Circle { pos, radius } => self.fill_circle(pos, radius, color),
		}
	}

	/// Draw the outline of a Hitbox
	fn stroke_hitbox(&mut self, hitbox: Hitbox, line_width: f32, color: Colors) {
		match hitbox {
			Hitbox::Rect { pos, size } => self.stroke_rect(pos, size, line_width, color),
			Hitbox::Circle { pos, radius } => self.stroke_circle(pos, radius, line_width, color),
		}
	}

	/// Fill a Rectangle at `pos` with `size`
	fn fill_rect<T: Into<GamePos>, T2: Into<GamePos>>(&mut self, pos: T, size: T2, color: Colors);
	/// Draw the outline of a Rectangle at `pos` with `size`
	fn stroke_rect<T: Into<GamePos>, T2: Into<GamePos>>(
		&mut self,
		pos: T,
		size: T2,
		line_width: f32,
		color: Colors,
	);

	/// Fill a Circle between centered at `pos`, with `radius`
	fn fill_circle<T: Into<GamePos>>(&mut self, pos: T, radius: f32, color: Colors);
	/// Draw the outline of a Circle between centered at `pos`, with `radius`
	fn stroke_circle<T: Into<GamePos>>(
		&mut self,
		pos: T,
		radius: f32,
		line_width: f32,
		color: Colors,
	);

	/// Draws `text` at `pos`
	fn draw_text<T: Into<GamePos>>(&mut self, text: &str, pos: T, color: Colors);

	/// Draws a segment of an image from `tile` at `target_pos`
	fn draw_asset<T: Into<GamePos>>(&mut self, tile: (usize, usize), target_pos: T);

	/// Draw the cached background
	fn draw_background(&mut self);

	/// Fills the background cache with black
	fn clear_background(&mut self);

	/// draws an asset from `tile` to the background cache at `target_pos`
	fn draw_to_background<T: Into<GamePos>>(&mut self, tile: (usize, usize), target_pos: T);
}

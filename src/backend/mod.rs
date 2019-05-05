
#[cfg(target_arch = "wasm32")]
mod web_backend;
#[cfg(target_arch = "wasm32")]
pub use web_backend::{Backend, Color};

#[cfg(not(target_arch = "wasm32"))]
mod window_backend;
#[cfg(not(target_arch = "wasm32"))]
pub use window_backend::{Backend, Color};

use crate::{Game, world::GamePos};

pub const TEXT_SIZE: usize = 16;

pub trait BackendStyle {

	fn start(game: Game);

	/// Returns the Width of the Screen in Pixels
	fn get_width(&self) -> u32;
	/// Returns the Height of the Screen in Pixels
	fn get_height(&self) -> u32;

	/// Fill the entire Screen with a single Color
	fn fill(&mut self, color: Color);

	/// Draw a Line from `start` to `end`
	fn draw_line(&mut self, start: GamePos, end: GamePos, color: Color);

	/// Fill a Rectangle at `pos` with `size`
	fn fill_rect(&mut self, pos: GamePos, size: GamePos, color: Color);
	/// Draw the outline of a Rectangle at `pos` with `size`
	fn stroke_rect(&mut self, pos: GamePos, size: GamePos, line_width: f32, color: Color);

	/// Fill a Circle between centered at `pos`, with `radius`
	fn fill_circle(&mut self, pos: GamePos, radius: f32, color: Color);
	/// Draw the outline of a Circle between centered at `pos`, with `radius`
	fn stroke_circle(&mut self, pos: GamePos, radius: f32, line_width: f32, color: Color);

	/// Draws `text` at `pos`
	fn draw_text(&mut self, text: &str, pos: GamePos, color: Color);

	/// Draws a segment of an image from `tile` at `target_pos`
	fn draw_asset(&mut self, tile: (usize, usize), target_pos: GamePos);

	/// Draw the cached background
	fn draw_background(&mut self);

	/// Fills the background cache with black
	fn clear_background(&mut self);

	/// draws an asset from `tile` to the background cache at `target_pos`
	fn draw_to_background(&mut self, tile: (usize, usize), target_pos: GamePos);
}

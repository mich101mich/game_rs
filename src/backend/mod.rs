
#[cfg(target_arch = "wasm32")]
mod web_backend;
#[cfg(target_arch = "wasm32")]
pub use web_backend::{Backend, Color};

#[cfg(not(target_arch = "wasm32"))]
mod window_backend;
#[cfg(not(target_arch = "wasm32"))]
pub use window_backend::{Backend, Color};

use crate::Game;

pub const TEXT_SIZE: usize = 16;

pub trait BackendStyle {

	fn start(game: Game);

	/// Returns the Width of the Screen in Pixels
	fn get_width(&self) -> u32;
	/// Returns the Height of the Screen in Pixels
	fn get_height(&self) -> u32;

	/// Fill the entire Screen with a single Color
	fn fill(&mut self, color: Color);

	/// Draw a Line from 'start' to 'end'
	fn draw_line(&mut self, start: (f32, f32), end: (f32, f32), color: Color);

	/// Fill a Rectangle between ('left', 'top'), ('right', 'bottom')
	fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color);
	/// Draw the outline of a Rectangle between ('left', 'top'), ('right', 'bottom')
	fn stroke_rect(&mut self, x: f32, y: f32, width: f32, height: f32, line_width: f32, color: Color);

	/// Fill a Circle between centered at ('x', 'y'), with radius 'radius'
	fn fill_circle(&mut self, x: f32, y: f32, radius: f32, color: Color);
	/// Draw the outline of a Circle between centered at ('x', 'y'), with radius 'radius'
	fn stroke_circle(&mut self, x: f32, y: f32, radius: f32, line_width: f32, color: Color);

	/// Draws 'text' at ('x', 'y')
	fn draw_text(&mut self, text: &str, x: f32, y: f32, color: Color);
}

use super::BackendStyle;
use crate::Game;

use stdweb::{unstable::TryInto, web::html_element::CanvasElement, web::*};

static mut GAME: Option<Game> = None;
static mut BACKEND: Option<Backend> = None;

fn game() -> &'static mut Game {
	unsafe { GAME.as_mut().unwrap() }
}

fn backend() -> &'static mut Backend {
	unsafe { BACKEND.as_mut().unwrap() }
}

fn update(time: f64) {
	window().request_animation_frame(update);

	let backend = backend();
	game().draw(backend);
}

pub struct Backend {
	base: CanvasRenderingContext2d,
	width: u32,
	height: u32,
}
impl BackendStyle for Backend {

	fn start(game: Game) {
		let canvas: CanvasElement = document()
			.create_element("canvas")
			.unwrap()
			.try_into()
			.unwrap();

		let (width, height) = (640, 480);
		canvas.set_width(width);
		canvas.set_height(height);

		document().body().unwrap().append_child(&canvas);

		unsafe {
			BACKEND = Some(Backend {
				base: canvas.get_context().unwrap(),
				width,
				height,
			});
			GAME = Some(game);
		}

		update(0.0);

		let message = "Hello, 世界!";
		js! {
		    console.log( @{message} );
		}
	}

	fn get_width(&self) -> u32 {
		self.width
	}
	fn get_height(&self) -> u32 {
		self.height
	}

	fn fill(&mut self, color: Color) {
		self.base.set_fill_style_color(&color.to_css());
		self.base.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
	}


	fn draw_line(&mut self, start: (f32, f32), end: (f32, f32), width: f32, color: Color) {}


	fn fill_rect(&mut self, left: f32, top: f32, right: f32, bottom: f32, color: Color) {}
	fn draw_rect(
		&mut self,
		left: f32,
		top: f32,
		right: f32,
		bottom: f32,
		width: f32,
		color: Color,
	) {

	}

	fn fill_ellipse(&mut self, x: f32, y: f32, rx: f32, ry: f32, color: Color) {}
	fn draw_ellipse(&mut self, x: f32, y: f32, rx: f32, ry: f32, width: f32, color: Color) {}


	fn draw_text(&mut self, text: &str, x: f32, y: f32, height: f32, color: Color) {}
}


#[macro_export]
macro_rules! log {
	( $( $x: expr ),* ) => {
		let s = format!($( $x ),*);
		js!(
			console.log(@{ s });
		)
	};
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

#[cfg(target_arch = "wasm32")]
impl Color {
	pub fn rgb(r: u8, g: u8, b: u8) -> Color {
		Color { r, g, b, a: 255 }
	}
	pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
		Color { r, g, b, a }
	}
	pub fn to_css(self) -> String {
		format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
	}
}

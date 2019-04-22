use super::{BackendStyle, TEXT_SIZE};
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
	context: CanvasRenderingContext2d,
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
				context: canvas.get_context().unwrap(),
				width,
				height,
			});
			GAME = Some(game);
		}

		backend().context.set_font(&format!("{}px consolas", TEXT_SIZE as f32 + 0.3));
		backend().context.set_text_baseline(TextBaseline::Top);

		update(0.0);
	}

	fn get_width(&self) -> u32 {
		self.width
	}
	fn get_height(&self) -> u32 {
		self.height
	}

	fn fill(&mut self, color: Color) {
		self.context.set_fill_style_color(&color.to_css());
		self.context.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
	}


	fn draw_line(&mut self, start: (f32, f32), end: (f32, f32), color: Color) {
		self.context.set_stroke_style_color(&color.to_css());
		self.context.set_line_width(1.0);
		self.context.begin_path();
		self.context.move_to(start.0 as f64, start.1 as f64);
		self.context.line_to(end.0 as f64, end.1 as f64);
		self.context.stroke();
	}


	fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) {
		self.context.set_fill_style_color(&color.to_css());
		self.context.fill_rect(x as f64, y as f64, width as f64, height as f64);
	}
	fn stroke_rect(&mut self, x: f32, y: f32, width: f32, height: f32, line_width: f32, color: Color) {
		self.context.set_stroke_style_color(&color.to_css());
		self.context.set_line_width(line_width as f64);
		self.context.stroke_rect(x as f64, y as f64, width as f64, height as f64)
	}

	fn fill_circle(&mut self, x: f32, y: f32, radius: f32, color: Color) {
		self.context.set_fill_style_color(&color.to_css());
		self.context.begin_path();
		self.context.arc(x as f64, y as f64, radius as f64, 0.0, 2.0 * std::f64::consts::PI, false);
		self.context.fill(Default::default());
	}
	fn stroke_circle(&mut self, x: f32, y: f32, radius: f32, line_width: f32, color: Color) {
		self.context.set_stroke_style_color(&color.to_css());
		self.context.set_line_width(line_width as f64);
		self.context.begin_path();
		self.context.arc(x as f64, y as f64, radius as f64, 0.0, 2.0 * std::f64::consts::PI, false);
		self.context.stroke();
	}

	fn draw_text(&mut self, text: &str, x: f32, y: f32, color: Color) {
		self.context.set_fill_style_color(&color.to_css());
		self.context.fill_text(text, x as f64, y as f64 + 4.0, None);
	}
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

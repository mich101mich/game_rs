use super::{BackendStyle, TEXT_SIZE};
use crate::Game;

use stdweb::{unstable::TryInto, web::html_element::*, web::*};

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
	background: CanvasRenderingContext2d,
	background_canvas: CanvasElement,
	assets: ImageElement,
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

		let background_canvas: CanvasElement = document()
			.create_element("canvas")
			.unwrap()
			.try_into()
			.unwrap();

		background_canvas.set_width(width);
		background_canvas.set_height(height);
		background_canvas.set_attribute("hidden", "true").unwrap();

		document().body().unwrap().append_child(&background_canvas);

		let assets: &[u8] = include_bytes!("../../assets/assets.png");
		let assets = base64::encode(assets);

		let img: ImageElement = document()
			.create_element("img")
			.unwrap()
			.try_into()
			.unwrap();
		img.set_src(&(String::from("data:image/png;base64,") + &assets));
		img.set_attribute("hidden", "true").unwrap();
		document().body().unwrap().append_child(&img);

		unsafe {
			BACKEND = Some(Backend {
				context: canvas.get_context().unwrap(),
				background: background_canvas.get_context().unwrap(),
				background_canvas,
				assets: img,
				width,
				height,
			});
			GAME = Some(game);
		}

		backend()
			.context
			.set_font(&format!("{}px consolas", TEXT_SIZE as f32 + 0.3));
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
		self.context
			.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
	}

	fn draw_line(&mut self, start: (f32, f32), end: (f32, f32), color: Color) {
		self.context.set_stroke_style_color(&color.to_css());
		self.context.set_line_width(1.0);
		self.context.begin_path();
		self.context.move_to(start.0 as f64, start.1 as f64);
		self.context.line_to(end.0 as f64, end.1 as f64);
		self.context.stroke();
	}

	fn fill_rect(&mut self, (x, y): (f32, f32), (w, h): (f32, f32), color: Color) {
		self.context.set_fill_style_color(&color.to_css());
		self.context
			.fill_rect(x as f64, y as f64, w as f64, h as f64);
	}
	fn stroke_rect(
		&mut self,
		(x, y): (f32, f32),
		(w, h): (f32, f32),
		line_width: f32,
		color: Color,
	) {
		self.context.set_stroke_style_color(&color.to_css());
		self.context.set_line_width(line_width as f64);
		self.context
			.stroke_rect(x as f64, y as f64, w as f64, h as f64)
	}

	fn fill_circle(&mut self, (x, y): (f32, f32), radius: f32, color: Color) {
		self.context.set_fill_style_color(&color.to_css());
		self.context.begin_path();
		self.context.arc(
			x as f64,
			y as f64,
			radius as f64,
			0.0,
			2.0 * std::f64::consts::PI,
			false,
		);
		self.context.fill(Default::default());
	}
	fn stroke_circle(&mut self, (x, y): (f32, f32), radius: f32, line_width: f32, color: Color) {
		self.context.set_stroke_style_color(&color.to_css());
		self.context.set_line_width(line_width as f64);
		self.context.begin_path();
		self.context.arc(
			x as f64,
			y as f64,
			radius as f64,
			0.0,
			2.0 * std::f64::consts::PI,
			false,
		);
		self.context.stroke();
	}

	fn draw_text(&mut self, text: &str, (x, y): (f32, f32), color: Color) {
		self.context.set_fill_style_color(&color.to_css());
		self.context.fill_text(text, x as f64, y as f64 + 4.0, None);
	}

	fn draw_asset(&mut self, (row, id): (usize, usize), (tx, ty): (f32, f32)) {
		#[rustfmt::skip]
		self.context.draw_image_s(
			self.assets.clone(),
			(id * 16) as f64, (row * 16) as f64,
			16.0, 16.0,
			tx as f64, ty as f64,
			16.0, 16.0
		).expect("Unable to draw image");
	}

	fn draw_background(&mut self) {
		js! {
			@{ &self.context }.drawImage(@{ &self.background_canvas }, 0.0, 0.0);
		}
	}

	fn clear_background(&mut self) {
		self.background.set_fill_style_color("black");
		self.background
			.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
	}

	fn draw_to_background(&mut self, (row, id): (usize, usize), (tx, ty): (f32, f32)) {
		#[rustfmt::skip]
		self.background.draw_image_s(
			self.assets.clone(),
			(id * 16) as f64, (row * 16) as f64,
			16.0, 16.0,
			tx as f64, ty as f64,
			16.0, 16.0
		).expect("Unable to draw image");
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

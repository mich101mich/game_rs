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

fn resize() {
	let backend = backend();

	let canvas_size = backend.canvas.get_bounding_client_rect();
	let width = canvas_size.get_width() as u32;
	let height = canvas_size.get_height() as u32;

	backend.width = width;
	backend.height = height;
	backend.canvas.set_width(width);
	backend.canvas.set_height(height);

	game().world.set_dirty();
}

pub struct Backend {
	canvas: CanvasElement,
	background_canvas: CanvasElement,
	ctx: CanvasRenderingContext2d,
	bg: CanvasRenderingContext2d,
	assets: ImageElement,
	width: u32,
	height: u32,
}
impl BackendStyle for Backend {
	fn start(game: Game) {

		document().body().unwrap().set_attribute("style", "margin: 0;  width: 100vw;  height: 100vh; background-color: black;").unwrap();

		let canvas: CanvasElement = document()
			.create_element("canvas")
			.unwrap()
			.try_into()
			.unwrap();

		canvas.set_attribute("style", "width: 100%; height: calc(100% - 3px);").unwrap();

		document().body().unwrap().append_child(&canvas);

		let background_canvas: CanvasElement = document()
			.create_element("canvas")
			.unwrap()
			.try_into()
			.unwrap();

		background_canvas.set_width(game.world.width() as u32 * 16);
		background_canvas.set_height(game.world.height() as u32 * 16);

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
				ctx: canvas.get_context().unwrap(),
				bg: background_canvas.get_context().unwrap(),
				canvas,
				background_canvas,
				assets: img,
				width: 200,
				height: 200,
			});
			GAME = Some(game);
		}

		{
			let ctx = &backend().ctx;
			ctx.set_font(&format!("{}px consolas", TEXT_SIZE as f32 + 0.3));
			ctx.set_text_baseline(TextBaseline::Top);
			js! {
				@{ &ctx }.imageSmoothingEnabled = false;
			}
		}

		js! {
			window.addEventListener("resize", () => {
				console.log("resize");
				@{ resize }();
			});
		}
		resize();

		update(0.0);
	}

	fn get_width(&self) -> u32 {
		self.width
	}
	fn get_height(&self) -> u32 {
		self.height
	}

	fn fill(&mut self, color: Color) {
		self.ctx.set_fill_style_color(&color.to_css());
		self.ctx.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
	}

	fn draw_line(&mut self, start: (f32, f32), end: (f32, f32), color: Color) {
		self.ctx.set_stroke_style_color(&color.to_css());
		self.ctx.set_line_width(1.0);
		self.ctx.begin_path();
		self.ctx.move_to(start.0 as f64, start.1 as f64);
		self.ctx.line_to(end.0 as f64, end.1 as f64);
		self.ctx.stroke();
	}

	fn fill_rect(&mut self, (x, y): (f32, f32), (w, h): (f32, f32), color: Color) {
		self.ctx.set_fill_style_color(&color.to_css());
		self.ctx.fill_rect(x as f64, y as f64, w as f64, h as f64);
	}
	fn stroke_rect(
		&mut self,
		(x, y): (f32, f32),
		(w, h): (f32, f32),
		line_width: f32,
		color: Color,
	) {
		self.ctx.set_stroke_style_color(&color.to_css());
		self.ctx.set_line_width(line_width as f64);
		self.ctx.stroke_rect(x as f64, y as f64, w as f64, h as f64)
	}

	fn fill_circle(&mut self, (x, y): (f32, f32), radius: f32, color: Color) {
		self.ctx.set_fill_style_color(&color.to_css());
		self.ctx.begin_path();
		self.ctx.arc(
			x as f64, y as f64,
			radius as f64,
			0.0, 2.0 * std::f64::consts::PI,
			false,
		);
		self.ctx.fill(Default::default());
	}
	fn stroke_circle(&mut self, (x, y): (f32, f32), radius: f32, line_width: f32, color: Color) {
		self.ctx.set_stroke_style_color(&color.to_css());
		self.ctx.set_line_width(line_width as f64);
		self.ctx.begin_path();
		self.ctx.arc(
			x as f64, y as f64,
			radius as f64,
			0.0, 2.0 * std::f64::consts::PI,
			false,
		);
		self.ctx.stroke();
	}

	fn draw_text(&mut self, text: &str, (x, y): (f32, f32), color: Color) {
		self.ctx.set_fill_style_color(&color.to_css());
		self.ctx.fill_text(text, x as f64, y as f64 + 4.0, None);
	}

	fn draw_asset(&mut self, (row, id): (usize, usize), (tx, ty): (f32, f32)) {
		#[rustfmt::skip]
		self.ctx.draw_image_s(
			self.assets.clone(),
			(id * 16) as f64, (row * 16) as f64,
			16.0, 16.0,
			tx as f64, ty as f64,
			16.0, 16.0
		).expect("Unable to draw image");
	}

	fn draw_background(&mut self) {
		js! {
			@{ &self.ctx }.drawImage(@{ &self.background_canvas }, 0.0, 0.0);
		}
	}

	fn clear_background(&mut self) {
		self.bg.set_fill_style_color("black");
		self.bg.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
	}

	fn draw_to_background(&mut self, (row, id): (usize, usize), (tx, ty): (f32, f32)) {
		#[rustfmt::skip]
		self.bg.draw_image_s(
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

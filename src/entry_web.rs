#[macro_use]
extern crate stdweb;

use stdweb::{
	unstable::TryInto,
	web::*,
	web::html_element::CanvasElement
};

macro_rules! log {
	( $( $x: expr ),* ) => {
		let s = format!($( $x ),*);
		js!(
			console.log(@{ s });
		)
	};
}

mod game;
use game::Game;

static mut GAME: Option<Game> = None;
static mut BACKEND: Option<CanvasGraphics> = None;

fn game() -> &'static mut Game {
	unsafe { GAME.as_mut().unwrap() }
}

fn backend() -> &'static mut CanvasGraphics {
	unsafe { BACKEND.as_mut().unwrap() }
}

fn main() {
    stdweb::initialize();

	let canvas: CanvasElement = document().create_element("canvas").unwrap().try_into().unwrap();

	let (width, height) = (640, 480);
	canvas.set_width(width);
	canvas.set_height(height);

	document().body().unwrap().append_child(&canvas);

	unsafe {
		BACKEND = Some(CanvasGraphics{
			base: canvas.get_context().unwrap(),
			context: Context {
				view: graphics::math::identity(),
				transform: graphics::math::identity(),
				draw_state: Default::default(),
				viewport: Some(Viewport {
					rect: [0, 0, width as i32, height as i32],
				    draw_size: [width, height],
				    window_size: [width as f64, height as f64],
				}),
			},
			width,
			height,
		});
		GAME = Some(Game::new());
	}
	

	update(0.0);

    let message = "Hello, 世界!";
    js! {
        console.log( @{message} );
    }
}

fn update(time: f64) {
	window().request_animation_frame(update);

	let backend = backend();
	game().draw(backend.context, backend);
}

use graphics::*;
use graphics::types::Color;

struct CanvasGraphics {
	base: CanvasRenderingContext2d,
	context: Context,
	width: u32, height: u32,
}

struct CanvasTexture {
	size: (u32, u32),
}

impl ImageSize for CanvasTexture {
	fn get_size(&self) -> (u32, u32) {
		self.size
	}
}

impl Graphics for CanvasGraphics {
	type Texture = CanvasTexture;

	fn clear_color(&mut self, color: Color) {
		self.base.set_fill_style_color(&convert_color(&color));
		self.base.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
	}
	fn clear_stencil(&mut self, _value: u8) {

	}
	fn tri_list<F>(&mut self, _draw_state: &DrawState, color: &Color, mut f: F)
		where F: FnMut(&mut FnMut(&[[f32; 2]]))
	{
		self.base.set_fill_style_color(&convert_color(color));
		//log!("{}", convert_color(color));

		f(&mut |vertices: &[[f32; 2]]| {
			self.base.begin_path();

			//log!("{:?}", vertices);

			for triangle in vertices.chunks(3) {
				self.base.move_to(triangle[0][0] as f64, triangle[0][1] as f64);
				self.base.line_to(triangle[1][0] as f64, triangle[1][1] as f64);
				self.base.line_to(triangle[2][0] as f64, triangle[2][1] as f64);
			}

			self.base.fill(Default::default());
        });
	}
	fn tri_list_uv<F>(&mut self, _draw_state: &DrawState, color: &Color, _texture: &Self::Texture, mut f: F)
		where F: FnMut(&mut FnMut(&[[f32; 2]], &[[f32; 2]]))
	{
		self.base.set_fill_style_color(&convert_color(color));

		f(&mut |vertices: &[[f32; 2]], _| {
			self.base.begin_path();

			for triangle in vertices.chunks(3) {
				self.base.move_to(triangle[0][0] as f64, triangle[0][1] as f64);
				self.base.line_to(triangle[1][0] as f64, triangle[1][1] as f64);
				self.base.line_to(triangle[2][0] as f64, triangle[2][1] as f64);
			}

			self.base.fill(Default::default());
        });
	}
}

fn convert_color(color: &Color) -> String {
	format!(
		"rgba({}, {}, {}, {})",
		color[0] * 255.0,
		color[1] * 255.0,
		color[2] * 255.0,
		color[3] * 255.0,
	)
}

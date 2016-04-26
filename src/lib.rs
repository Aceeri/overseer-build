#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;

extern crate cgmath;
extern crate num_traits;
extern crate time;
extern crate regex;

use gfx::traits::{Factory, FactoryExt};
use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub mod world;
pub mod camera;

use camera::Camera;

gfx_vertex_struct!( Vertex {
	pos: [i8; 4] = "a_Pos",
	tex_coord: [i8; 2] = "a_TexCoord",
});

impl Vertex {
	fn new(p: [i8; 3], t: [i8; 2]) -> Vertex {
		Vertex {
			pos: [p[0], p[1], p[2], 1],
			tex_coord: t,
		}
	}
}

gfx_pipeline!( pipe {
	vbuf: gfx::VertexBuffer<Vertex> = (),
	transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
	color: gfx::TextureSampler<[f32; 4]> = "t_Color",
	out_color: gfx::RenderTarget<ColorFormat> = "Target0",
	out_depth: gfx::DepthTarget<DepthFormat> =
		gfx::preset::depth::LESS_EQUAL_WRITE,
});

pub struct Overseer {
	pub window: glutin::Window,
	pub device: gfx_device_gl::Device,
	pub factory: gfx_device_gl::Factory,
	pub encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
	pub bundle: pipe::Bundle<gfx_device_gl::Resources>,
	pub camera: self::camera::Camera,
}

impl Overseer {
	pub fn new() -> Self {
		let vs = include_bytes!("../shader/voxel.glslv");
		let fs = include_bytes!("../shader/voxel.glslf");

		// cube vertex data
		let vertex_data = [
			// top (0, 0, 1)
			Vertex::new([-1, -1,  1], [0, 0]),
			Vertex::new([ 1, -1,  1], [1, 0]),
			Vertex::new([ 1,  1,  1], [1, 1]),
			Vertex::new([-1,  1,  1], [0, 1]),
			// bottom (0, 0, -1)
			Vertex::new([-1,  1, -1], [1, 0]),
			Vertex::new([ 1,  1, -1], [0, 0]),
			Vertex::new([ 1, -1, -1], [0, 1]),
			Vertex::new([-1, -1, -1], [1, 1]),
			// right (1, 0, 0)
			Vertex::new([ 1, -1, -1], [0, 0]),
			Vertex::new([ 1,  1, -1], [1, 0]),
			Vertex::new([ 1,  1,  1], [1, 1]),
			Vertex::new([ 1, -1,  1], [0, 1]),
			// left (-1, 0, 0)
			Vertex::new([-1, -1,  1], [1, 0]),
			Vertex::new([-1,  1,  1], [0, 0]),
			Vertex::new([-1,  1, -1], [0, 1]),
			Vertex::new([-1, -1, -1], [1, 1]),
			// front (0, 1, 0)
			Vertex::new([ 1,  1, -1], [1, 0]),
			Vertex::new([-1,  1, -1], [0, 0]),
			Vertex::new([-1,  1,  1], [0, 1]),
			Vertex::new([ 1,  1,  1], [1, 1]),
			// back (0, -1, 0)
			Vertex::new([ 1, -1,  1], [0, 0]),
			Vertex::new([-1, -1,  1], [1, 0]),
			Vertex::new([-1, -1, -1], [1, 1]),
			Vertex::new([ 1, -1, -1], [0, 1]),
		];

		let index_data: &[u16] = &[
			 0,  1,  2,  2,  3,  0, // top
			 4,  5,  6,  6,  7,  4, // bottom
			 8,  9, 10, 10, 11,  8, // right
			12, 13, 14, 14, 15, 12, // left
			16, 17, 18, 18, 19, 16, // front
			20, 21, 22, 22, 23, 20, // back
		];

		let (width, height) = (1600, 900);

		let builder = glutin::WindowBuilder::new()
			.with_title("Cube with glutin example".to_string())
			.with_dimensions(width, height)
			.with_min_dimensions(800, 600)
			.with_vsync();
		let (window, device, mut factory, main_color, main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
		let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

		window.set_cursor_state(glutin::CursorState::Grab).unwrap();
		window.set_cursor_position(width as i32 / 2, height as i32 / 2).unwrap();

		let camera = Camera::new(&window);

		let (vertex_buffer, slice) = factory.create_vertex_buffer_indexed(&vertex_data, index_data);

		let texels = [[0x20, 0xA0, 0xC0, 0x00]];
		let (_, texture_view) = factory.create_texture_const::<gfx::format::Rgba8>(
			gfx::tex::Kind::D2(1, 1, gfx::tex::AaMode::Single), &[&texels]
			).unwrap();

		let sinfo = gfx::tex::SamplerInfo::new(gfx::tex::FilterMethod::Bilinear, gfx::tex::WrapMode::Clamp);

		let pso = factory.create_pipeline_simple(vs, fs, gfx::state::CullFace::Back, pipe::new()).unwrap();

		let data = pipe::Data {
			vbuf: vertex_buffer,
			transform: (camera.perspective * camera.view).into(),
			color: (texture_view, factory.create_sampler(sinfo)),
			out_color: main_color,
			out_depth: main_depth,
		};

		let bundle = pipe::Bundle {
			slice: slice,
			pso: pso, 
			data: data,
		};

		Overseer {
			window: window,
			device: device,
			factory: factory,
			encoder: encoder,
			bundle: bundle,
			camera: camera,
		}
	}

	pub fn update(&mut self) {
		self.camera.update(&self.window);

		self.bundle.data.transform = (self.camera.perspective * self.camera.view).into();
	}

	pub fn render(&mut self) {
		self.encoder.clear(&self.bundle.data.out_color, [0.1, 0.2, 0.3, 1.0]);
		self.encoder.clear_depth(&self.bundle.data.out_depth, 1.0);
		self.bundle.encode(&mut self.encoder);
		self.encoder.flush(&mut self.device);
		self.window.swap_buffers().unwrap();
		self.device.cleanup();
	}
}
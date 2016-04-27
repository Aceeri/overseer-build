#![feature(io)]

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;

extern crate cgmath;
extern crate num_traits;
extern crate time;
extern crate regex;
extern crate rand;

use std::path::PathBuf;

use gfx::traits::{Factory, FactoryExt};
use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub mod world;
pub mod camera;

use camera::Camera;

gfx_vertex_struct!( Vertex {
	pos: [i8; 4] = "vertex_position",
	normal: [i8; 4] = "vertex_normal",
});

gfx_pipeline!( pipe {
	vbuf: gfx::VertexBuffer<Vertex> = (),
	transform: gfx::Global<[[f32; 4]; 4]> = "camera_transform",
	voxels: gfx::InstanceBuffer<world::chunk::InstancedVoxel> = (),
	out_color: gfx::RenderTarget<ColorFormat> = "fragment",
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
	pub world: world::World,
}

impl Overseer {
	pub fn new() -> Self {
		let mut world = world::World::new();
		//world.chunks = world::chunk::Chunk::stress(5);
		world.load_wrld(PathBuf::from("world/tree.wrld"));
		world.load_chunk([0, 0, 0]);

		let vs = include_bytes!("../shader/voxel.glslv");
		let fs = include_bytes!("../shader/voxel.glslf");

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

		let mut instances = Vec::new();
		for chunk in world.chunks.iter() {
			chunk.instances(&mut instances);
		}

		let voxel_buffer = factory.create_buffer_const(&instances, gfx::BufferRole::Vertex, gfx::Bind::empty()).unwrap();

		let (vertex_buffer, mut slice) = factory.create_vertex_buffer_indexed(&world::chunk::VERTICES, world::chunk::INDICES);
		slice.instances = Some((instances.len() as u32, 0));
		println!("Voxels: {:?}", instances.len());

		let pso = factory.create_pipeline_simple(vs, fs, gfx::state::CullFace::Back, pipe::new()).unwrap();

		let data = pipe::Data {
			vbuf: vertex_buffer,
			transform: (camera.perspective * camera.view).into(),
			voxels: voxel_buffer,
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
			world: world,
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
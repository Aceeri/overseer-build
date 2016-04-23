#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate cgmath;
extern crate num_traits;

use gfx::traits::{Factory, FactoryExt};
use gfx::Device;

use num_traits::float::Float;
use cgmath::prelude::Angle;
use cgmath::{Point3, Vector3, Vector4, Transform, AffineMatrix3, Matrix4, Deg};

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

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

gfx_constant_struct!( Locals {
	transform: [[f32; 4]; 4] = "u_Transform",
});

gfx_pipeline!( pipe {
	vbuf: gfx::VertexBuffer<Vertex> = (),
	transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
	locals: gfx::ConstantBuffer<Locals> = "Locals",
	color: gfx::TextureSampler<[f32; 4]> = "t_Color",
	out_color: gfx::RenderTarget<ColorFormat> = "Target0",
	out_depth: gfx::DepthTarget<DepthFormat> =
		gfx::preset::depth::LESS_EQUAL_WRITE,
});

fn main() {
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

	let builder = glutin::WindowBuilder::new()
		.with_title("Cube with glutin example".to_string())
		.with_dimensions(1024, 768)
		.with_vsync();
	let (window, mut device, mut factory, main_color, main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
	let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

	let (vertex_buffer, slice) = factory.create_vertex_buffer_indexed(&vertex_data, index_data);

	let texels = [[0x20, 0xA0, 0xC0, 0x00]];
	let (_, texture_view) = factory.create_texture_const::<gfx::format::Rgba8>(
		gfx::tex::Kind::D2(1, 1, gfx::tex::AaMode::Single), &[&texels]
		).unwrap();

	let sinfo = gfx::tex::SamplerInfo::new(
		gfx::tex::FilterMethod::Bilinear,
		gfx::tex::WrapMode::Clamp);

	let pso = factory.create_pipeline_simple(vs, fs, gfx::state::CullFace::Back, pipe::new()).unwrap();

	let mut rotation = Vector3::new(0.0, 0.0, 0.0);
	let mut view: AffineMatrix3<f32> = Transform::look_at(
		Point3::new(5.0, 5.0, 5.0),
		Point3::new(0f32, 0.0, 0.0),
		Vector3::unit_z(),
	);

	let (width, height) = window.get_inner_size().unwrap();
	let aspect = (width as f32 * window.hidpi_factor()) / (height as f32 * window.hidpi_factor());
	let proj = cgmath::perspective(cgmath::deg(45.0f32), aspect, 1.0, 10.0);

	let mut data = pipe::Data {
		vbuf: vertex_buffer,
		transform: (proj * view.mat).into(),
		locals: factory.create_constant_buffer(1),
		color: (texture_view, factory.create_sampler(sinfo)),
		out_color: main_color,
		out_depth: main_depth,
	};

	let mut prev_mouse: Option<(i32, i32)> = None;
	let mut position = Vector3::new(0.0, 0.0, -5.0);

	'main: loop {
		let mut direction = Vector4::new(0.0, 0.0, 0.0, 0.0);
		
		let (x_s, x_c) = rotation.x.sin_cos();
		let (y_s, y_c) = rotation.y.sin_cos();
		let (z_s, z_c) = rotation.z.sin_cos();

		let rotation_mat = Matrix4::new(
			y_c*z_c, x_c*z_s + x_s*y_s*z_c, x_s*z_s - x_c*y_s*z_c, 0.0,
			-y_c*z_s, x_c*z_c - x_s*y_s*z_s, x_s*z_c + x_c*y_s*z_s, 0.0,
			y_s, -x_s*y_c, x_c*y_c, 0.0,
			0.0, 0.0, 0.0, 1.0,
		);

		// loop over events
		for event in window.poll_events() {
			use glutin::{Event, ElementState, VirtualKeyCode};

			match event {
				Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
				Event::Closed => break 'main,

				Event::MouseMoved((x, y)) => {
					if let Some(prev) = prev_mouse {
						let dx = prev.0 - x;
						let dy = prev.1 - y;
						rotation -= Vector3::new(dy as f32 / 100.0, dx as f32 / 100.0, 0.0);
					}
					prev_mouse = Some((x, y));
				},
				Event::KeyboardInput(ElementState::Pressed, _, Some(key)) => {
					match key {
						VirtualKeyCode::D => {
							let rotated = rotation_mat * Vector4::new(1.0, 0.0, 0.0, 0.0);
							direction += Vector4::new(rotated.x, 0.0, rotated.z, 0.0);
						},
						VirtualKeyCode::A => {
							let rotated = rotation_mat * Vector4::new(1.0, 0.0, 0.0, 0.0);
							direction -= Vector4::new(rotated.x, 0.0, rotated.z, 0.0);
						},
						VirtualKeyCode::W => direction += rotation_mat * Vector4::new(0.0, 0.0, 1.0, 0.0),
						VirtualKeyCode::S => direction -= rotation_mat * Vector4::new(0.0, 0.0, 1.0, 0.0),
						_ => { },
					}
				}
				_ => {},
			}
		}

		let rot_view = Vector3::new(direction.x, direction.y, direction.z);

		position -= Vector3::new(rot_view.x, rot_view.y, -rot_view.z);

		let translation = Matrix4::new(
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			position.x,  position.y,  position.z, 1.0,
		);

		data.transform = (proj * rotation_mat * translation).into();

		// draw a frame
		encoder.clear(&data.out_color, [0.1, 0.2, 0.3, 1.0]);
		encoder.clear_depth(&data.out_depth, 1.0);
		encoder.draw(&slice, &pso, &data);
		encoder.flush(&mut device);
		window.swap_buffers().unwrap();
		device.cleanup();
	}
}
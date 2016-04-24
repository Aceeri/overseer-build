#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate cgmath;
extern crate num_traits;
extern crate time;

use gfx::traits::{Factory, FactoryExt};
use gfx::Device;

use num_traits::float::Float;
use cgmath::prelude::{Angle, InnerSpace};
use cgmath::{Point3, Vector3, Vector4, Transform, AffineMatrix3, Matrix4, Deg};

use time::{Duration, PreciseTime};

use std::collections::HashMap;

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
		.with_min_dimensions(800, 600)
		.with_vsync();
	let (mut window, mut device, mut factory, main_color, main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
	let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

	window.set_cursor_state(glutin::CursorState::Grab);

	let (vertex_buffer, slice) = factory.create_vertex_buffer_indexed(&vertex_data, index_data);

	let texels = [[0x20, 0xA0, 0xC0, 0x00]];
	let (_, texture_view) = factory.create_texture_const::<gfx::format::Rgba8>(
		gfx::tex::Kind::D2(1, 1, gfx::tex::AaMode::Single), &[&texels]
		).unwrap();

	let sinfo = gfx::tex::SamplerInfo::new(
		gfx::tex::FilterMethod::Bilinear,
		gfx::tex::WrapMode::Clamp);

	let pso = factory.create_pipeline_simple(vs, fs, gfx::state::CullFace::Back, pipe::new()).unwrap();

	//let mut rotation = Vector3::new(0.0, 0.0, 0.0);
	let mut pitch = 0.0;
	let mut yaw = 0.0;
	let mut view: AffineMatrix3<f32> = Transform::look_at(
		Point3::new(5.0, 5.0, 5.0),
		Point3::new(0f32, 0.0, 0.0),
		Vector3::unit_z(),
	);

	let (width, height) = window.get_inner_size().unwrap();
	let aspect = (width as f32 * window.hidpi_factor()) / (height as f32 * window.hidpi_factor());
	let proj = cgmath::perspective(cgmath::deg(45.0f32), aspect, 0.1, 100.0);

	let mut data = pipe::Data {
		vbuf: vertex_buffer,
		transform: (proj * view.mat).into(),
		locals: factory.create_constant_buffer(1),
		color: (texture_view, factory.create_sampler(sinfo)),
		out_color: main_color,
		out_depth: main_depth,
	};

	let mut locked = (0, 0);
	let mut prev_mouse: Option<(i32, i32)> = None;
	let mut position = Vector3::new(0.0, 0.0, -5.0);

	let mut keys: [bool; 255] = [false; 255];

	let mut dt32 = 0.0f32;
	let mut dt64 = 0.0f64;
	let mut now = PreciseTime::now();

	'main: loop {
		use glutin::{Event, ElementState, VirtualKeyCode};

		let temp = PreciseTime::now();
		let delta = now.to(temp).num_nanoseconds();

		if let Some(dt) = delta {
			dt32 = (dt as f32 / 1_000_000_000f32);
			dt64 = (dt as f64 / 1_000_000_000f64);
		} else {
			dt32 = 0.0;
			dt64 = 0.0;
		}
		
		now = temp;

		// loop over events
		for event in window.poll_events() {
			match event {
				Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) |
				Event::Closed => break 'main,

				Event::MouseMoved((x, y)) => {
					if let Some(prev) = prev_mouse {
						let dx = prev.0 - x;
						let dy = prev.1 - y;
						yaw += dx as f32 / 200.0;
						pitch += dy as f32 / 200.0;

						// clamps y axis rotation to 85 degrees
						if pitch > 1.48 {
							pitch = 1.48;
						} else if pitch < -1.48 {
							pitch = -1.48;
						}
					} else {
						locked = (x, y);
					}
					prev_mouse = Some((x, y));
					//window.set_cursor_position(locked.0, locked.1);
				},

				Event::KeyboardInput(state, code, _) => {
					//println!("{:?}", code);
					if state == ElementState::Pressed {
						keys[code as usize] = true;
					} else {
						keys[code as usize] = false;
					}
				},
				_ => {},
			}
		}

		let (sin_pitch, cos_pitch) = pitch.sin_cos();
		let (sin_yaw, cos_yaw) = yaw.sin_cos();

		let x_axis = Vector3::new(cos_yaw, 0.0, -sin_yaw);
		let y_axis = Vector3::new(sin_yaw * sin_pitch, cos_pitch, cos_yaw * sin_pitch);
		let z_axis = Vector3::new(sin_yaw * cos_pitch, -sin_pitch, cos_pitch * cos_yaw);

		if keys[17] { // W
			position -= z_axis * dt32;
		}

		if keys[31] { // S
			position += z_axis * dt32;
		}

		if keys[30] { // A 
			position -= x_axis * dt32;
		}

		if keys[32] { // D
			position += x_axis * dt32;
		}

		if keys[57] { // Space
			position += y_axis * dt32;
		}

		if keys[29] { // Left Control
			position -= y_axis * dt32;
		}

		let view = Matrix4::new(
			x_axis.x, y_axis.x, z_axis.x, 0.0,
			x_axis.y, y_axis.y, z_axis.y, 0.0,
			x_axis.z, y_axis.z, z_axis.z, 0.0,
			-x_axis.dot(position), -y_axis.dot(position), -z_axis.dot(position), 1.0,
		);

		data.transform = (proj * view).into();

		// draw a frame
		encoder.clear(&data.out_color, [0.1, 0.2, 0.3, 1.0]);
		encoder.clear_depth(&data.out_depth, 1.0);
		encoder.draw(&slice, &pso, &data);
		encoder.flush(&mut device);
		window.swap_buffers().unwrap();
		device.cleanup();
	}
}
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

pub mod camera;




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
	window: glutin::Window,
	//data: pipe::Data,
	camera: self::camera::Camera,
}
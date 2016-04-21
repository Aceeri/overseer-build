extern crate gl;
extern crate gtk;
extern crate gobject_sys;
extern crate epoxy;

#[macro_use]
extern crate gfx;
extern crate gfx_core;
extern crate gfx_device_gl;

use gtk::prelude::*;
use gtk::{Window, WindowType, Grid, GLArea, Box as GTKBox, Orientation, ActionBar, ButtonBox, Label, Paned};

use gfx::traits::FactoryExt;
use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_vertex_struct!(Vertex {
    pos: [f32; 2] = "a_Pos",
    color: [f32; 3] = "a_Color",
});

gfx_pipeline!(pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    out: gfx::RenderTarget<ColorFormat> = "Target0",
});

const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
    Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 1.0] }
];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub struct OverseerWindow {
	window: Window,
	toolbar: ActionBar,
	glarea: GLArea,
	//gfx: 
}

impl OverseerWindow {
	pub fn new() -> OverseerWindow {
		let window = Window::new(WindowType::Toplevel);
		window.set_title("Overseer Voxel Builder");
		window.set_default_size(800, 600);
		window.connect_delete_event(|_, _| {
			gtk::main_quit();
			Inhibit(false)
		});

		let pane = Paned::new(Orientation::Horizontal);
		window.add(&pane);

		pane.set_position(100);

		let toolbar = ActionBar::new();
		pane.add1(&toolbar);

		let glarea = GLArea::new();
		pane.add2(&glarea);

		window.show_all();

		OverseerWindow {
			window: window,
			toolbar: toolbar,
			glarea: glarea,
		}
	}

	pub fn init(&mut self) {

		//let (window, mut device, mut factory, main_color, _main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
		//let (device, factory) = gfx_device_gl::create(|s| { gdk_sys::gdk_gl_get_proc_address(s) });
		/*let (color_view, ds_view) = gfx_device_gl::create_main_targets_raw(dim, color_format.0, ds_format.0);

		let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
		let pso = factory.create_pipeline_simple(
			include_bytes!("../shader/triangle_150.glslv"),
			include_bytes!("../shader/triangle_150.glslf"),
			gfx::state::CullFace::Nothing,
			pipe::new()
		).unwrap();
		let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
		let data = pipe::Data {
			vbuf: vertex_buffer,
			out: main_color
		};

		self.glarea.connect_render(|_glarea, _glcontext| {
			encoder.clear(&data.out, CLEAR_COLOR);
        	encoder.draw(&slice, &pso, &data);
        	encoder.flush(&mut device);
        	device.cleanup();
			Inhibit(false)
		});*/
	}
}
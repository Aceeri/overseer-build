#![feature(io)]

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;

extern crate cgmath;
extern crate collision;
extern crate num_traits;
extern crate time;
extern crate regex;
extern crate rand;
extern crate bit_set;

use std::path::PathBuf;

use gfx::traits::{Factory, FactoryExt};
use gfx::Device;

use cgmath::{Matrix4};

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub mod world;
pub mod camera;

use camera::Camera;

gfx_vertex_struct!( Vertex {
    pos: [f32; 4] = "vert_Pos",
    normal: [i8; 4] = "vert_Normal",
});

gfx_constant_struct!( LightParam {
    pos: [f32; 4] = "pos",
    color: [f32; 4] = "color",
    proj: [[f32; 4]; 4] = "proj",
});

gfx_pipeline!( pipe {
    time: gfx::Global<f32> = "Time",
    vbuf: gfx::VertexBuffer<Vertex> = (),
    transform: gfx::Global<[[f32; 4]; 4]> = "c_Transform",
    voxels: gfx::InstanceBuffer<world::chunk::InstancedVoxel> = (),
    lights: gfx::ConstantBuffer<LightParam> = "b_Lights",
    out_color: gfx::RenderTarget<ColorFormat> = "Target0",
    out_depth: gfx::DepthTarget<DepthFormat> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});

pub struct Overseer {
    pub window: glutin::Window,
    pub device: gfx_device_gl::Device,
    pub factory: gfx_device_gl::Factory,
    pub encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    pub bundle: gfx::Bundle<gfx_device_gl::Resources, pipe::Data<gfx_device_gl::Resources>>,
    pub camera: self::camera::Camera,
    pub world: world::World,
}

impl Overseer {
    pub fn new() -> Self {
        let mut world = world::World::new();
        world.load_wrld(PathBuf::from("world/wall.wrld"));
        for x in -2..2 {
            for z in -2..2 {
                world.load_chunk([x, 0, z]);
            }
        }

        let vs = include_bytes!("../shader/voxel.glslv");
        let fs = include_bytes!("../shader/voxel.glslf");

        let (width, height) = (1600, 900);

        let builder = glutin::WindowBuilder::new()
            .with_title("Cube with glutin example".to_string())
            .with_dimensions(width, height)
            .with_min_dimensions(800, 600)
            .with_vsync();
        let (window, device, mut factory,
            main_color, main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
        let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        let camera = Camera::new(&window);

        let mut instances = Vec::new();
        for chunk in world.chunks.iter() {
            chunk.instances(&mut instances);
        }

        let voxel_buffer = factory.create_buffer_const(&instances, gfx::BufferRole::Vertex, gfx::Bind::empty()).unwrap();

        let (vertex_buffer, mut slice) = factory.create_vertex_buffer_with_slice(&world::chunk::VERTICES, world::chunk::INDICES);
        slice.instances = Some((instances.len() as u32, 0));
        println!("Voxels: {:?}", instances.len());

        let raster = gfx::state::Rasterizer {
            front_face: gfx::state::FrontFace::CounterClockwise,
            cull_face: gfx::state::CullFace::Nothing,
            //method: gfx::state::RasterMethod::Line(3),
            method: gfx::state::RasterMethod::Fill,
            offset: None,
            samples: None,
        };

        let shader_set = factory.create_shader_set(vs, fs).unwrap();

        let pso = factory.create_pipeline_state(&shader_set, gfx::Primitive::TriangleList, raster, pipe::new()).unwrap();

        let pos = [25.0, 4.0, 22.0, 1.0];
        let pos2 = [25.0, 15.0, 22.0, 1.0];

        let light_params = vec![LightParam {
            pos: pos,
            color: [ 1.0, 1.0, 1.0, 1.0],
            proj: {
                let mx_proj: Matrix4<_> =
                    cgmath::PerspectiveFov {
                        fovy: cgmath::deg(60f32).into(),
                        aspect: 1.0,
                        near: 1f32,
                        far: 20f32,
                    }.to_perspective().into();

                let mx_view = cgmath::Matrix4::look_at(
                        cgmath::Point3::new(pos[0], pos[1], pos[2]),
                        cgmath::Point3::new(0.0, 0.0, 0.0),
                        cgmath::Vector3::unit_z(),
                    );

                (mx_view * mx_proj).into()
            },
        }, LightParam {
            pos: pos2,
            color: [ 1.0, 1.0, 1.0, 1.0],
            proj: {
                let mx_proj: Matrix4<_> =
                    cgmath::PerspectiveFov {
                        fovy: cgmath::deg(60f32).into(),
                        aspect: 1.0,
                        near: 1f32,
                        far: 20f32,
                    }.to_perspective().into();

                let mx_view = cgmath::Matrix4::look_at(
                        cgmath::Point3::new(pos2[0], pos2[1], pos2[2]),
                        cgmath::Point3::new(0.0, 0.0, 0.0),
                        cgmath::Vector3::unit_z(),
                    );

                (mx_view * mx_proj).into()
            },
        }];

        let light_buf = factory.create_buffer_const(&light_params, gfx::BufferRole::Uniform, gfx::Bind::empty()).unwrap();

        let data = pipe::Data {
            time: 0.0,
            vbuf: vertex_buffer,
            transform: (camera.perspective * camera.view).into(),
            voxels: voxel_buffer,
            lights: light_buf,
            out_color: main_color,
            out_depth: main_depth,
        };

        let bundle = gfx::Bundle {
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

    pub fn update(&mut self, delta: f32) {
        self.camera.update(&self.window);

        self.bundle.data.time += delta;
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
use super::cgmath;
use cgmath::prelude::{InnerSpace, SquareMatrix};
use cgmath::{Vector3, Matrix4};

#[derive(Debug)]
pub struct Camera {
    pub view: Matrix4<f32>,
    pub perspective: Matrix4<f32>,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub position: Vector3<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl Camera {
    pub fn new(window: &super::glutin::Window) -> Camera {
        let (fov, near, far) = (45.0f32, 0.1f32, 100f32);

        let (width, height) = window.get_inner_size().unwrap();
        let aspect = (width as f32 * window.hidpi_factor()) /
                     (height as f32 * window.hidpi_factor());

        Camera {
            view: Matrix4::identity(),
            perspective: cgmath::perspective(cgmath::deg(fov), aspect, near, far),
            fov: fov,
            near: near,
            far: far,
            position: Vector3::new(0.0, 0.0, -5.0),
            pitch: 0.0f32,
            yaw: 0.0f32,
            roll: 0.0f32,
        }
    }

    pub fn axis(&self) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        let (sin_roll, cos_roll) = self.roll.sin_cos();

        let x_axis = Vector3::new(cos_roll * cos_yaw, sin_roll * cos_yaw, -sin_yaw);
        let y_axis = Vector3::new(-sin_roll * cos_pitch + sin_pitch * cos_roll * sin_yaw,
                                  cos_pitch * cos_roll + sin_roll * sin_pitch * sin_yaw,
                                  sin_pitch * cos_yaw);
        let z_axis = Vector3::new(sin_roll * sin_pitch + cos_pitch * cos_roll * sin_yaw,
                                  -sin_pitch * cos_roll + sin_roll * cos_pitch * sin_yaw,
                                  cos_pitch * cos_yaw);

        (x_axis, y_axis, z_axis)
    }

    pub fn update(&mut self, window: &super::glutin::Window) {
        let axis = self.axis();

        let x_axis = axis.0;
        let y_axis = axis.1;
        let z_axis = axis.2;

        self.view = Matrix4::new(
            x_axis.x, y_axis.x, z_axis.x, 0.0,
            x_axis.y, y_axis.y, z_axis.y, 0.0,
            x_axis.z, y_axis.z, z_axis.z, 0.0,
            -x_axis.dot(self.position), -y_axis.dot(self.position), -z_axis.dot(self.position), 1.0,
        );

        let (width, height) = window.get_inner_size().unwrap();
        let aspect = (width as f32 * window.hidpi_factor()) /
                     (height as f32 * window.hidpi_factor());

        self.perspective = cgmath::perspective(cgmath::deg(self.fov), aspect, self.near, self.far);
    }
}
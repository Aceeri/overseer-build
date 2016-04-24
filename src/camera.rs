use cgmath::prelude::{InnerSpace, SquareMatrix};
use cgmath::{Vector3, Matrix4};

pub struct Camera {
	view: Matrix4<f32>,
	position: Vector3<f32>,
	pitch: f32,
	yaw: f32,
}

impl Camera {
	pub fn new() -> Camera {
		Camera {
			view: Matrix4::identity(),
			position: Vector3::new(0.0, 0.0, 0.0),
			pitch: 0.0f32,
			yaw: 0.0f32,
		}
	}

	pub fn axis(&self) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
		let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
		let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

		let x_axis = Vector3::new(cos_yaw, 0.0, -sin_yaw);
		let y_axis = Vector3::new(sin_yaw * sin_pitch, cos_pitch, cos_yaw * sin_pitch);
		let z_axis = Vector3::new(sin_yaw * cos_pitch, -sin_pitch, cos_pitch * cos_yaw);

		(x_axis, y_axis, z_axis)
	}

	pub fn update(&mut self) {
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
	}
}
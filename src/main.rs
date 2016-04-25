#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate cgmath;
extern crate num_traits;
extern crate time;
extern crate overseer_voxel;

use overseer_voxel::{Overseer};

use gfx::traits::{Factory, FactoryExt};
use gfx::Device;

use num_traits::float::Float;
use cgmath::prelude::{Angle, InnerSpace};
use cgmath::{Point3, Vector3, Vector4, Transform, AffineMatrix3, Matrix4, Deg};

use time::{Duration, PreciseTime};

fn main() {
	let mut overseer = Overseer::new();

	let mut locked = (0, 0);
	let mut prev_mouse: Option<(i32, i32)> = None;

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

		overseer.update();
		overseer.render();

		let camera = &mut overseer.camera;

		// loop over events
		for event in overseer.window.poll_events() {
			match event {
				Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) |
				Event::Closed => break 'main,

				Event::MouseMoved((x, y)) => {
					if let Some(prev) = prev_mouse {
						let dx = prev.0 - x;
						let dy = prev.1 - y;
						camera.yaw += dx as f32 / 200.0;
						camera.pitch += dy as f32 / 200.0;

						// clamps y axis rotation to 85 degrees
						if camera.pitch > 1.48 {
							camera.pitch = 1.48;
						} else if camera.pitch < -1.48 {
							camera.pitch = -1.48;
						}
					} else {
						locked = (x, y);
					}
					prev_mouse = Some((x, y));
					//window.set_cursor_position(locked.0, locked.1);
				},

				Event::KeyboardInput(state, code, _) => {
					if state == ElementState::Pressed {
						keys[code as usize] = true;
					} else {
						keys[code as usize] = false;
					}
				},
				_ => {},
			}
		}

		let axis = camera.axis();

		if keys[17] { // W
			camera.position -= axis.2 * dt32;
		}

		if keys[31] { // S
			camera.position += axis.2 * dt32;
		}

		if keys[30] { // A 
			camera.position -= axis.0 * dt32;
		}

		if keys[32] { // D
			camera.position += axis.0 * dt32;
		}

		if keys[57] { // Space
			camera.position += axis.1 * dt32;
		}

		if keys[29] { // Left Control
			camera.position -= axis.1 * dt32;
		}
	}
}
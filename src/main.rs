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

use time::PreciseTime;

use std::collections::VecDeque;
use std::path::PathBuf;

fn main() {
	let mut world = overseer_voxel::world::World::new();
	world.load_wrld(PathBuf::from("world/test.wrld"));
	//let chunk = overseer_voxel::world::chunk::Chunk::new([0, 0, 0]);
	//println!("{:?}", chunk);

	let mut overseer = Overseer::new();

	let mut keys: [bool; 255] = [false; 255];

	let mut dt32 = 0.0f32;
	let mut dt64 = 0.0f64;
	let mut now = PreciseTime::now();

	let mut count = 0.0f64;

	let mut average = VecDeque::new();

	let mut focused = true;

	'main: loop {
		use glutin::{Event, ElementState, VirtualKeyCode};

		let temp = PreciseTime::now();
		let delta = now.to(temp).num_microseconds();

		if let Some(dt) = delta {
			dt32 = dt as f32 / 1_000_000f32;
			dt64 = dt as f64 / 1_000_000f64;
		} else {
			dt32 = 0.0;
			dt64 = 0.0;
		}

		now = temp;

		average.push_back(1.0 / dt64);

		if average.len() > 30 {
			average.pop_front();
		}

		count += dt64;

		if count > 1.0f64 {
			let mut av = 0.0;
			for point in &average {
				av += point.clone();
			}
			av /= average.len() as f64;

			println!("fps: {:?}", av as u32);
			count = 0.0f64;
		}
		
		overseer.update();
		overseer.render();

		let camera = &mut overseer.camera;

		// loop over events
		for event in overseer.window.poll_events() {
			match event {
				Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) |
				Event::Closed => break 'main,

				Event::MouseMoved((x, y)) => {
					if focused {
						if let Some((width, height)) = overseer.window.get_inner_size() {
							let dx = width as i32 / 2 - x;
							let dy = height as i32 / 2 - y;

							camera.yaw += dx as f32 / 200.0;
							camera.pitch += dy as f32 / 200.0;

							// clamps y axis rotation to 85 degrees
							if camera.pitch > 1.48 {
								camera.pitch = 1.48;
							} else if camera.pitch < -1.48 {
								camera.pitch = -1.48;
							}

							if let Err(e) = overseer.window.set_cursor_position(width as i32 / 2, height as i32 / 2) {
								println!("SET CURSOR ERROR {:?}", e);
							}
						}
					}
				},

				Event::Focused(focus) => {
					if focus {
						overseer.window.set_cursor_state(glutin::CursorState::Grab).unwrap();

						if let Some((width, height)) = overseer.window.get_inner_size() {
							if let Err(e) = overseer.window.set_cursor_position(width as i32 / 2, height as i32 / 2) {
								println!("SET CURSOR ERROR {:?}", e);
							}
						}
					} else {
						overseer.window.set_cursor_state(glutin::CursorState::Normal).unwrap();
					}

					focused = focus;

					println!("{:?}", focus);
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

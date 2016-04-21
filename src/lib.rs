#[macro_use]
extern crate glium;
extern crate gtk;
extern crate gobject_sys;
extern crate libc;
extern crate epoxy;
extern crate shared_library;

use gtk::prelude::*;
use gtk::{Window, WindowType, Grid, GLArea, Box as GTKBox, Orientation, ActionBar, ButtonBox, Label, Paned};

use std::ptr;
use std::cell::RefCell;
use std::rc::Rc;
use std::os::raw::c_void;

use glium::Surface;

use shared_library::dynamic_library::DynamicLibrary;

pub struct OverseerWindow {
	window: Window,
	toolbar: ActionBar,
	glarea: GLArea,
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

	pub fn glium(&mut self) {
		// get proc address

		// create backend using GLArea
	}
}
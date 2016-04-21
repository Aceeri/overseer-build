extern crate overseer_voxel as ov;
extern crate gtk;

use ov::OverseerWindow;

fn main() {
	if gtk::init().is_err() {
		println!("gtk failed to initialize");
	};

	let mut window = OverseerWindow::new();
	window.init();

	gtk::main();
}
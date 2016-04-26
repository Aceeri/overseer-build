use std::path::PathBuf;
use std::collections::HashMap;

pub struct World {
	wdfn_file: PathBuf,
	wrld_file: PathBuf,

	definitions: Vec<Definition>,
	map: HashMap<[i64; 3], i64>, // location in file
	chunks: Vec<Chunk>, // current chunks loaded
}

pub struct Definition {
	name: String, // identifier
	color: [u8; 4], // color of voxel
}

pub struct Voxel(usize); // index to a definition

pub struct Chunk {
	position: [i64; 3],
	data: [[[Voxel; 16]; 16]; 16], // 16x16x16 array of voxels
}

impl World {
	pub fn new() -> World {
		World {
			wdfn_file: PathBuf::new(),
			wrld_file: PathBuf::new(),

			definitions: Vec::new(),
			map: HashMap::new(),
			chunks: Vec::new(),
		}
	}

	pub fn load_wdfn(&mut self, wdfn: PathBuf) {


		self.wdfn_file = wdfn;
	}

	pub fn load_wrld(&mut self, wrld: PathBuf) {

		
		self.wrld_file = wrld;
	}
}
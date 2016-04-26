
#[derive(Copy, Clone, Debug)]
pub struct Voxel(usize); // index to a definition

#[derive(Copy, Clone, Debug)]
pub struct Chunk {
  position: [i64; 3],
  data: [[[Voxel; 16]; 16]; 16], // 16x16x16 array of voxels
}

impl Chunk {
  pub fn new(position: [i64; 3]) -> Chunk {
    Chunk {
      position: position,
      data: [[[Voxel(0); 16]; 16]; 16],
    }
  }

  /*pub fn from(file: PathBuf, location: i64) -> Chunk {
    // read file
  }*/
}
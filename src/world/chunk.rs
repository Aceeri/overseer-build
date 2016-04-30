
use std::fmt;
use std::path::Path;
use std::fs::File;
use std::io::{Read, BufReader, Write, Seek, SeekFrom};

use bit_set::BitSet;

use super::super::Vertex;

pub static VERTICES: [Vertex; 24] = [
    // top (0, 0, 1)
    Vertex { pos: [-1, -1,  1,  1], normal: [0, 0, 1, 1], },
    Vertex { pos: [ 1, -1,  1,  1], normal: [0, 0, 1, 1], },
    Vertex { pos: [ 1,  1,  1,  1], normal: [0, 0, 1, 1], },
    Vertex { pos: [-1,  1,  1,  1], normal: [0, 0, 1, 1], },
    // bottom (0, 0, -1)
    Vertex { pos: [-1,  1, -1,  1], normal: [0, 0, -1, 1], },
    Vertex { pos: [ 1,  1, -1,  1], normal: [0, 0, -1, 1], },
    Vertex { pos: [ 1, -1, -1,  1], normal: [0, 0, -1, 1], },
    Vertex { pos: [-1, -1, -1,  1], normal: [0, 0, -1, 1], },
    // right (1, 0, 0)
    Vertex { pos: [ 1, -1, -1,  1], normal: [1, 0, 0, 1], },
    Vertex { pos: [ 1,  1, -1,  1], normal: [1, 0, 0, 1], },
    Vertex { pos: [ 1,  1,  1,  1], normal: [1, 0, 0, 1], },
    Vertex { pos: [ 1, -1,  1,  1], normal: [1, 0, 0, 1], },
    // left (-1, 0, 0)
    Vertex { pos: [-1, -1,  1,  1], normal: [-1, 0, 0, 1], },
    Vertex { pos: [-1,  1,  1,  1], normal: [-1, 0, 0, 1], },
    Vertex { pos: [-1,  1, -1,  1], normal: [-1, 0, 0, 1], },
    Vertex { pos: [-1, -1, -1,  1], normal: [-1, 0, 0, 1], },
    // front (0, 1, 0)
    Vertex { pos: [ 1,  1, -1,  1], normal: [0, 1, 0, 1], },
    Vertex { pos: [-1,  1, -1,  1], normal: [0, 1, 0, 1], },
    Vertex { pos: [-1,  1,  1,  1], normal: [0, 1, 0, 1], },
    Vertex { pos: [ 1,  1,  1,  1], normal: [0, 1, 0, 1], },
    // back (0, -1, 0)
    Vertex { pos: [ 1, -1,  1,  1], normal: [0, -1, 0, 1], },
    Vertex { pos: [-1, -1,  1,  1], normal: [0, -1, 0, 1], },
    Vertex { pos: [-1, -1, -1,  1], normal: [0, -1, 0, 1], },
    Vertex { pos: [ 1, -1, -1,  1], normal: [0, -1, 0, 1], },
];

pub static INDICES: &'static [u16] = &[
     0,  1,  2,  2,  3,  0, // top
     4,  5,  6,  6,  7,  4, // bottom
     8,  9, 10, 10, 11,  8, // right
    12, 13, 14, 14, 15, 12, // left
    16, 17, 18, 18, 19, 16, // front
    20, 21, 22, 22, 23, 20, // back
];

gfx_vertex_struct!( InstancedVoxel {
    position: [i32; 4] = "vox_Pos",
    color: [f32; 4] = "vox_Color",
});

#[derive(Copy, Clone)]
pub struct Voxel {
    id: u16, // index to a definition
}

impl fmt::Debug for Voxel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Chunk {
    position: [i32; 3],
    data: [[[Voxel; 16]; 16]; 16], // 16x16x16 array of voxels
}

impl Chunk {
    pub fn new(position: [i32; 3]) -> Chunk {
        Chunk {
            position: position,
            data: [[[Voxel { id: 0 }; 16]; 16]; 16],
        }
    }

    pub fn from(file: &Path, position: [i32; 3], location: u64) -> Option<Chunk> {
        let mut file = File::open(file).unwrap();
        file.seek(SeekFrom::Start(location));

        let mut expr = "".to_owned();
        let (mut x, mut y, mut z) = (0, 0, 0);

        let mut chunk = Chunk::new(position);

        'parse: for found in file.chars() {
            match found {
                Ok(character) => {
                    
                    match character {
                        // ignore
                        '\n' | '\r' | '\t' => continue,

                        // end of chunk
                        '^' => {
                            return Some(chunk);
                        },

                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '*' => {
                            expr.push(character);
                        },

                        // parse expr and reset
                        ',' => {

                            let mut voxel = expr.clone();
                            let mut times = 1;

                            if let Some(_) = expr.clone().find('*') {
                                let split = expr.split('*').collect::<Vec<_>>();
                                voxel = split[0].to_owned();

                                times = match split[1].parse::<u64>() {
                                    Ok(t) => t,
                                    Err(_) => 1,
                                }
                            }

                            for _ in 0..times {
                                match voxel.parse::<u16>() {
                                    Ok(id) => {
                                        chunk.data[y][x][z] = Voxel { id: id };

                                        z += 1;
                                        if z >= 16 {
                                            z = 0;
                                            x += 1;

                                            if x >= 16 {
                                                x = 0;
                                                y += 1;

                                                if y > 16 {
                                                    println!("CHUNK TOO LONG");
                                                    return Some(chunk);
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!("VOXEL ERROR: {:?}", e);
                                        chunk.data[y][x][z] = Voxel { id: 0 };
                                    },
                                };
                            }

                            expr = "".to_owned();
                        },

                        _ => {
                            return None;
                        }
                    }
                },
                Err(e) => {
                    println!("ERR: {:?}", e);
                }
            }
        }

        None
    }

    pub fn write(&self) -> String {
        let header = format!("^({},{},{}):\r\n", self.position[0],
                             self.position[1], self.position[2]);

        let mut content = "".to_owned();

        let mut first = true;
        let mut last = "".to_owned();
        let mut times = 1;
        for (i, y) in self.data.iter().enumerate() {
            for (l, x) in y.iter().enumerate() {
                for (k, z) in x.iter().enumerate() {
                    let s = z.id.to_string();

                    if s == last {
                        times += 1;
                    } else {
                        if times > 1 {
                            content = content + r"*" + &times.to_string() + r",";
                        } else if !first {
                            content = content + r",";
                        } else {
                            first = false;
                        }

                        times = 1;
                        content = content + &s;
                    }

                    if i == 15 && l == 15 && k == 15 {
                        if times > 1 {
                            content = content + r"*" + &times.to_string();
                        }

                        content = content + r",";
                    }

                    last = s;
                }
            }
        }

        format!("{}{}^\r\n", header, content)
    }

    pub fn stress(range: u32) -> Vec<Chunk> {
        let mut list = Vec::new();

        for x in 0..range {
            for z in 0..range {
                for y in 0..range {
                    list.push(
                        Chunk {
                            position: [x as i32, y as i32, z as i32],
                            data: [[[Voxel { id: 2 }; 16]; 16]; 16],
                        }
                    );
                }
            }
        }
        
        list 
    }

    pub fn instances(&self, list: &mut Vec<InstancedVoxel>) {
        for (y_pos, y) in self.data.iter().enumerate() {
            for (x_pos, x) in y.iter().enumerate() {
                for (z_pos, z) in x.iter().enumerate() {

                    if z.id != 0 {
                        let mut color = match z.id {
                            1 => [0.02, 0.55, 0.0, 1.0],
                            2 => [0.43, 0.35, 0.286, 1.0],
                            3 => [0.11, 0.385, 0.102, 1.0],
                            _ => [0.00, 0.00, 0.00, 0.00],
                        };
                        list.push(InstancedVoxel {
                            position: [
                                self.position[0] * 16 + x_pos as i32, 
                                self.position[1] * 16 + y_pos as i32, 
                                self.position[2] * 16 + z_pos as i32, 1],
                            color: color,
                        });
                    }
                }
            }
        }
    }
}
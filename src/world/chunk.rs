
use std::fmt;
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};

#[derive(Copy, Clone)]
pub struct Voxel(usize); // index to a definition

impl fmt::Debug for Voxel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

  pub fn from(file: &Path, position: [i64; 3], location: u64) -> Option<Chunk> {
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
                match voxel.parse::<usize>() {
                  Ok(id) => {
                    chunk.data[x][y][z] = Voxel(id);

                    z += 1;
                    if z >= 16 {
                      z = 0;
                      y += 1;

                      if y >= 16 {
                        y = 0;
                        x += 1;

                        if x > 16 {
                          println!("CHUNK TOO LONG");
                          return Some(chunk);
                        }
                      }
                    }
                  }
                  Err(e) => {
                    println!("VOXEL ERROR: {:?}", e);
                    chunk.data[x][y][z] = Voxel(0);
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
    let header = format!("^({},{},{}):\r\n", self.position[0], self.position[1], self.position[2]);

    let mut content = "".to_owned();

    let mut last = "".to_owned();
    let mut times = 1;
    for (i, x) in self.data.iter().enumerate() {
      for (l, y) in x.iter().enumerate() {
        for (k, z) in y.iter().enumerate() {
          let s = z.0.to_string();

          if s == last {
            times += 1;
          } else {
            if times > 1 {
              content = content + r"*" + &times.to_string() + r",";
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

    //File::create("test.wrld").unwrap().write(format!("{}{}^", header, content).as_bytes());

    format!("{}{}^", header, content)
  }
}
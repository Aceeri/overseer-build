
pub mod chunk;

use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

use regex::Regex;

use self::chunk::Chunk;

#[derive(Debug)]
pub struct Definition {
  name: String, // identifier
  color: [u8; 4], // color of voxel
}

#[derive(Debug)]
pub struct World {
  wdfn_file: PathBuf,
  wrld_file: PathBuf,

  definitions: Vec<Definition>,
  map: HashMap<[i64; 3], i64>, // location in file
  chunks: Vec<Chunk>, // current chunks loaded
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

  pub fn load_wdfn(&mut self, path: PathBuf) {
    self.wdfn_file = path;
  }

  pub fn load_wrld(&mut self, path: PathBuf) {
    let region_regex = Regex::new(r"\^\((\d+),(\d+),(\d+)\):").unwrap(); // matches ^(0,0,0):

    match File::open(&path) {
      Ok(mut file) => {
        let mut buffer = "".to_owned();
        file.read_to_string(&mut buffer).unwrap();

        for (position, captured) in region_regex.find_iter(&buffer).zip(region_regex.captures_iter(&buffer)) {
          let x = captured.at(1).unwrap().parse::<i64>().unwrap();
          let y = captured.at(2).unwrap().parse::<i64>().unwrap();
          let z = captured.at(3).unwrap().parse::<i64>().unwrap();

          self.map.insert([x, y, z], position.1 as i64);
        }

        self.wrld_file = path;
      },
      Err(e) => {
        println!("{:?}", e);
      }
    }
  }
}
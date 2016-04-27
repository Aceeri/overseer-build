
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

  pub definitions: Vec<Definition>,
  pub map: HashMap<[i32; 3], u64>, // location in file
  pub chunks: Vec<Chunk>, // current chunks loaded
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
    let definition_regex = Regex::new(r#"\"(.+)\"\s+?(.)\((.+)\);"#).unwrap();

    match File::open(&path) {
      Ok(mut file) => {
        let mut buffer = "".to_owned();
        file.read_to_string(&mut buffer).unwrap();

        for captures in definition_regex.captures_iter(&buffer) {
          let name = captured.at(1).unwrap();
          let attr_type = captured.at(2).unwrap();
          let attr = captured.at(3).unwrap();

          
        }

        self.wdfn_file = path;
      },
      Err(e) => {
        println!("{:?}", e);
      }
    }
  }

  pub fn load_wrld(&mut self, path: PathBuf) {
    let region_regex = Regex::new(r"\^\((\d+),(\d+),(\d+)\):").unwrap(); // matches ^(0,0,0):

    // load locations of chunks
    match File::open(&path) {
      Ok(mut file) => {
        let mut buffer = "".to_owned();
        file.read_to_string(&mut buffer).unwrap();

        for (position, captured) in region_regex.find_iter(&buffer).zip(region_regex.captures_iter(&buffer)) {
          let x = captured.at(1).unwrap().parse::<i32>().unwrap();
          let y = captured.at(2).unwrap().parse::<i32>().unwrap();
          let z = captured.at(3).unwrap().parse::<i32>().unwrap();

          self.map.insert([x, y, z], position.1 as u64);
        }

        self.wrld_file = path;
      },
      Err(e) => {
        println!("{:?}", e);
      }
    }
  }

  pub fn load_chunk(&mut self, position: [i32; 3]) {
    if let Some(location) = self.map.get(&position) {
      println!("Chunk {:?} found at {:?}", position, location);

      match Chunk::from(&self.wrld_file, position, location.clone()) {
        Some(chunk) => self.chunks.push(chunk),
        None => { },
      }
    } else {
      println!("No chunk found at location: {:?}", position);
    }
  }

  fn unload_chunk(&mut self, index: usize) {
    self.chunks.remove(index);
  }
}
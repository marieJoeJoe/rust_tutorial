use std::fs::File;
use std::io::prelude::*;



fn main() {
  let mut file = File::create("output.txt").expect("Could not create file!");

  
  file.write_all(b"welcome").expect("Could not write 2 the file!");

}


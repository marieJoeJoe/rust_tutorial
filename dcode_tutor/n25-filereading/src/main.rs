use std::fs::File;
use std::io::prelude::*;

fn main() {
  
  let mut file = File::open("akd_free_pdo.sh").expect("can not open file!");
  //cargo run, search current directory

  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Oops! can not read the file ... ");

  println!("file:\n\n{}",contents);

}

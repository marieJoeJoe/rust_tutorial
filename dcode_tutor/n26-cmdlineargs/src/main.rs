use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  for arguments in args.iter(){
      println!("{}",arguments);
  }
}

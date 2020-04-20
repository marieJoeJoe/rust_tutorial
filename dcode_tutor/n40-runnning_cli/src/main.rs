use std::process::Command;

fn main() {
  // python decode.py
  let mut cmd = Command::new("python");
  cmd .arg("dcode.py");


  //execute the command
  match cmd.output() {
    Ok(o) =>{
      unsafe {
        println!("Output:\n {}",String::from_utf8_unchecked(o.stdout));
      }
    },
    Err(e) =>{
      println!(" There was an error! {}",e); 
    }, 
  }
}


use std::collections::HashMap;

fn main() {

  let mut marks = HashMap::new();

  marks.insert("rust programming",96);
  marks.insert("web development",94);
  marks.insert("UX design",92);
  marks.insert("professional computing studies",99);

  println!("how many subjects have you studied? {}", marks.len());

  match marks.get("web development") {
    Some(mark) => println!("you got {} fow web dev!", mark),
    None =>  println!("you didn't study web development"),
  }
  marks.remove("UX design");


  for (subject, mark) in &mut marks {
    println!("for {} you got {}%!",subject ,mark);
  }
  let sj = String::from("OS concepts");
  //println!("Did you study OS ? {}",marks.contains_key("OS concepts"));
  println!("Did you study {} ? {}",sj,marks.contains_key::<str>(&sj));

}

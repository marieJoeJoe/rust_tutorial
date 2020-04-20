fn get_occupation(name: &str) -> Option<&str>{
  match name {
    "Domenic" => Some("Software Developer"),
    "Michael" => Some("Dentist"),
    _=> None,
  }
}




fn main() {

    let name = String::from("Domenic!");

    for i in 0..name.len(){

      println!("{}",
        match name.chars().nth(i) {
          Some(c) => c.to_string(), // there is a value find 
          None => "No character".to_string(), //there is no value finded
        });
    }

    println!("occupation: {}",match get_occupation("Domenic"){
      Some(o) => o,
      None => "Nothing",
    });

    
    println!("occupation: {}",match get_occupation("James"){
      Some(o) => o,
      None => "Nothing",
    });




}

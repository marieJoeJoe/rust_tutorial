#![allow(dead_code)]
enum Day {
  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
  Sunday,
}


impl Day{
  fn is_weekday(&self)->bool {
    match self {
      &Day::Saturday | &Day::Sunday  => return false,
      _ => return true,
    }
  }


}




fn main() {

  let d1 = Day::Tuesday; 
  let d2 = Day::Saturday; 

  println!("{}",d1.is_weekday());
  println!("{}",d2.is_weekday());
}

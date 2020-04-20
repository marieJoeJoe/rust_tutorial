// color : red green blue
struct Color {
  red: u8,
  green:u8,
  blue:u8,
}


fn main() {
  let mut bg = Color {red: 255 , green: 100, blue: 100};
  
  bg.red = 33;
  
  println!("red:{} green:{} blue:{}",bg.red,bg.green,bg.blue)
}

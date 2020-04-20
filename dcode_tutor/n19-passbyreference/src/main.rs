// color : red green blue
struct Color {
  red: u8,
  green:u8,
  blue:u8,
}


fn main() {
  let mut bg = Color {red: 255 , green: 100, blue: 100};
  
  bg.red = 33;
  
  print_color(&mut bg);
}

fn print_color(c:&mut Color){
  c.green = 77;
  println!("Color - R:{}, G:{}, B:{}",c.red,c.green,c.blue);
}

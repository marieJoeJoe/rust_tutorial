

fn main() {
  let number = 120;

  match number{
    1 => println!("one"),
    2 => println!("two"),
    10|11 => println!("11|10"),
    12...20 =>println!("12...20"),
    _ => println!("does not match!")
  }

}

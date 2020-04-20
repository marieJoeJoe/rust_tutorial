
fn main() {
    print_number_to(10);
}


fn print_number_to(num: u32){
  for n in 1..num {
      if is_even(n)
      {
          println!("{} is even",n);
      }else{
          println!("{} is odd",n);
      }
  }
}

fn is_even(num: u32) -> bool{
  return num%2 == 0;
}

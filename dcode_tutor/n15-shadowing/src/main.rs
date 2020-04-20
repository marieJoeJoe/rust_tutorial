use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
  let mut x = 10;

  {
    x = 99;
    println!("{} type {}",x,type_of(x));

    let x = 1.5;

    println!("{} type {}",x,type_of(x));

  }

  let x = "x is string";
  println!("{} type {}",x,type_of(x));


  let x = true;
  println!("{} type {}",x,type_of(x));


  let x = 45.7;
  println!("{} type {}",x,type_of(x));
}

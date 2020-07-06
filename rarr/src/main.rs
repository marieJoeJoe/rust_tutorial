use std::any::type_name;
  
fn type_of<T>(_: T) -> &'static str {
  type_name::<T>()
}

extern crate rand;
use rand::prelude::*;

struct ZeroSet {
  e_cont:i32,
  z_set:Vec<Vec<i32>>,
}

fn main()
{

  let mut rng = rand::thread_rng();

  let mut nums:Vec<i32> = (1..10).collect();

  nums.shuffle(&mut rng);

  for num in nums.iter_mut() {
    *num -= 5;
  }

  //let mut pos = nums.len() - 1;

  //println!("pos {}",pos);

  for numi in nums.iter() {
    println!("{}",numi);
  }

  println!("----------------------------------------");

  let mut pos = nums.iter().rev();
  
  println!("{}",*pos);   
}

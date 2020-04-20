extern crate rand;

use rand::Rng;


fn main() {

  let random_number = rand::thread_rng().gen_range(1, 11);
  println!("Random Number: {}",random_number);


  //Filp a coin
  let probability = 2; // at a probability of {1/proability} to get true
  let random_bool = rand::thread_rng().gen_weighted_bool(probability);
  println!("Random Boolean: {}",random_bool);

}

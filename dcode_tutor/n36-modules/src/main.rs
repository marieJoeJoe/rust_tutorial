mod md_test{

  fn chicken(){
    println!("chicken!");
  }


  pub fn print_message(){
    chicken();
    println!("how's going!");

  }

  pub mod water {
    pub fn print_message(){
      println!("I am water");
    }
  }


}



fn main() {
    md_test::print_message();
    md_test::water::print_message();

}

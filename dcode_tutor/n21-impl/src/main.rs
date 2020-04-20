struct rectangle {
    width: u32,
    height: u32,
}

impl rectangle {
  fn print_description(&self){
    println!(" rec: {}x{} area {}",self.width,self.height,self.width*self.height);
  }
  fn is_square(&self)->bool{
    self.width == self.height
  }

}


fn main() {

  let my_rect = rectangle {width: 11, height: 17};
  my_rect.print_description();
  println!(" rec is square {} ",my_rect.is_square());
  //my_rec.print_description();
  let my_srect = rectangle {width: 22, height: 22};
  my_srect.print_description();
  println!(" rec is square {} ",my_srect.is_square());
 
}

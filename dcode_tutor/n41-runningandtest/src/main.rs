struct Rectangle{
  width:u8,
  height:u8,
}

impl Rectangle {
  fn is_square(&self)->bool {
    self.width == self.height
  }


}


fn main() {
    println!("Hello, world!");
}


fn give_two()->i32{
  2
}


#[cfg(test)]
mod md_test{
/*
#[test] run when cargo test
#[should_panic] will not panic
#[ignore] will ignore this test entry when cargo test

*/

  #[test]
  #[should_panic]
  fn test_basic(){
    assert!(1 == 1);
    panic!("Oops!");
  }

  #[test]
  //#[ignore]
  fn test_equals(){
    assert_eq!(2 , 1 + 1);
    assert_ne!(2 , 1 + 3);
  }


  #[test]
  fn test_fn(){
    assert_eq!(super::give_two() , 1 + 1);
    assert_ne!(super::give_two() , 1 + 3);
  }

  #[test]
  //#[should_panic]
  fn test_struct(){
    let r = super::Rectangle {
      width: 50,
      height: 50,
    };
  
    assert!(r.is_square());
  }

}


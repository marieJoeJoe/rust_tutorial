fn main() {
    let x = 10;

    {
      let y = 5; 
      //isolated
      println!("x: {} y: {}",x,y);
    }

    //println!("x: {} y: {}",x,y);
    //can not build, variable y cannot be accessed outside the declared code block.
}

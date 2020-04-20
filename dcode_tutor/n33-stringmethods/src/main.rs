// 4 methods

fn main() {
    /*replace*/
    println!("\n\nreplace:\n");
    {
      let my_string = String::from("Rust is fantastic!");
      println!("After replace: {}",my_string.replace("fantastic","great"));
    }

    /*lines*/
    println!("\n\nlines:\n");
    {
      let my_string = String::from("The\nweather\nis\nnice\noutside\nmate!");
      for line in my_string.lines(){
        println!("After replace: [ {} ]",line);
      }
    }

    /*split*/
    println!("\n\nsplit:\n");
    {
      let my_string = String::from("Rust is fantastic!");
      //let tokens:Vec<&str> = my_string.split("").collect(); split by character
      let tokens:Vec<&str> = my_string.split(" ").collect();
      for token in tokens.iter(){
          println!("{}",token);
      }
    }

    /*trim*/
    println!("\n\ntrim:\n");
    {
      let my_string = String::from("    Rust is fantastic! \n\n\r");
      println!("before {}",my_string);
      println!("after {}",my_string.trim());
    }
    /*chars*/
    println!("\n\nchars:\n");
    {
      let my_string = String::from("Rustisfantastic!");
      //println!("{}",Some(my_string.chars().nth(5)));
      for i in 0..my_string.len(){
        match my_string.chars().nth(i) {
          Some(c) => println!("{}",c),
          None => println!("None"),
        }
      }
      
//      for i in 0..my_string.len() {
//          println!("{}",my_string.chars().nth(i).to_string());
//      }
    }
 

}

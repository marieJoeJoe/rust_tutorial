enum Direction{

    
}

fn main() {
    // single line comment.
  
    /* multi-line comments
     *
     *
     * */

    println!("Hello, world!");

    let mut n = 0;

    loop{
      n+=1;

      if 0==n%3 {
        println!("n={}",n);
      }

      if 25==n {
          break;
      }

    }
    println!("n={}",n);

    while n>0 {
        n-=1;
  
        if 0==n%3 {
          println!("n={}",n);
        }
  
      }
      println!("n={}",n);

      let numbers = 10 .. 30;

      for (index,a) in numbers.enumerate() {
        println!("a[{}]={}",index,a);
      }

      let animals = vec!["rabbit","dog","cat"];


      for (ani_index,ani) in animals.iter().enumerate() {
        println!("animals[{}]={}",ani_index,ani);
      }


}

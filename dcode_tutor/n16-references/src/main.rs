fn main() {
  let mut x =10;
  {
    let  dom = &mut x;
    *dom += 1;
  }
  //println!("{}",dom);

  println!("{}",x);
  let xr = & mut x;
  *xr += 1;

  println!("{}",x);

}

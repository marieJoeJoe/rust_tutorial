struct person {
  name : String,
  age : u8,
}

impl ToString for person {
  fn to_string(&self) -> String {
    //return format!("My name is {} and I am {}.",self.name, self.age);
    format!("My name is {} and I am {}.",self.name, self.age)
  }


}

fn main() {
  let dom = person {name: String::from("Domenic"),age: 21};


  println!("{}",dom.to_string());
}

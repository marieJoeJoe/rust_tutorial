

fn main() {
    let mut tstr = String::from("how's going? I am bond.");

    println!("Length: {}",tstr.len());
    println!("String is empty ? {}",tstr.is_empty());

    for token in tstr.split_whitespace() {
      println!("{}",token);
    }

    println!("Dose string contain 'bond'? {} ",tstr.contains("bond"));

    tstr.push_str(" welcome !");

    println!("{}",tstr);
}

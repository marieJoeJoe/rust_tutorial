extern crate regex;

use regex::Regex;

fn main() {

    let re = Regex::new(r"\w{5}").unwrap();

    let text = "dcode";

    println!("{}",re.is_match(text));

    let rea = Regex::new(r"(\w{5})").unwrap();

    match rea.captures(text) {
        Some(caps) => println!("{}",caps.get(0).unwrap().as_str()),
        None => println!("could not find !"),
 
    }

    match rea.captures(text) {
        Some(caps) => println!("{}",&caps[0]),
        None => println!("could not find !"),
 
    }

}

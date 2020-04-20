extern crate reqwest;

fn main() {

    let response_text = reqwest::get("https://mariejoejoe.github.io")
        .expect("could not make request")
        .text()
        .expect("could not read response text!");
    
    println!("Response Text: {}",response_text);
    
/*
    match reqwest::get("https://mariejoejoe.github.io"){
      Ok(mut response) => {
        if response.status() == reqwest::StatusCode::OK {
          match response.text(){
            Ok(text) => println!("Response Text: {}",text),
            Err(_) => println!("could not read response text !"),
          }
        }else{
          println!("Response was not 200 OK .")
        }
      },
      Err(_) => println!("could not make the request !")
    }
*/
}

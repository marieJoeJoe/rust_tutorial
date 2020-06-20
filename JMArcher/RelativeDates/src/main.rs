extern crate clap;

use clap::{ App , Arg };

fn main() {
  let matches = App::new("reldate")
      .version("0.0.1")
      .author("J/A <archer884@gmail.com")
      .about("Prints relative dates.")
      .subcommand(Subcommand::new("month")
          .about("Allows creation of month-relative date stream.")
          .arg(Arg::new("day")
               .short("d")
               .long("day")
               .help("day of week")
               .takes_value(true))
          .arg(Arg::new("ord")
               .short("o")
               .long("ord")
               .help("ordinal value of repeated date")
               .takes_value(true))
      .get_matches();

}

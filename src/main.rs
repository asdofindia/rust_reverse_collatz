extern crate clap;
use clap::{Arg, App};

fn main() {
    println!("Hello, world!");
    let matches = App::new("reverse_collatz")
                          .version("0.1.0")
                          .author("Akshay S Dinesh <asdofindia@gmail.com>")
                          .about("Finds reverse collatz")
                          .arg(Arg::with_name("STEPS")
                               .help("Number of steps of collatz")
                               .required(true))
                          .get_matches();
    let steps = matches.value_of("STEPS").unwrap();
    println!("the number of steps you say is {}", steps);
    println!("Calculating the lowest number that takes {} steps to reach 1", steps);


}

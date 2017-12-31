extern crate clap;
use clap::{Arg, App};

use std::collections::HashMap;

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
    let steps = matches.value_of("STEPS").unwrap().parse::<i32>().unwrap();
    println!("the number of steps you say is {}", steps);
    println!("Calculating the lowest number that takes {} steps to reach 1", steps);

    // We need a vector to store the smallest number which will take the given number of steps to
    // finish collatz series
    let mut solutions: HashMap<i32, i32> = HashMap::new();
    // The answer to zero steps is when you are already at 1
    solutions.insert(0, 1);
    solutions.insert(1, 2);
    
    // Now we also need to memoize the steps_required for each number we calculate so that we don't
    // have to repeat it for large numbers
    let mut steps_required: HashMap<i32, i32> = HashMap::new();
    steps_required.insert(1, 0);
    steps_required.insert(2, 1);
    // etc
    
    // Now we need to solve the problem, in different scopes so we don't mix solutions
    
    // while loop
    {
        let mut w_solutions = solutions.clone();
        let mut w_steps_required = steps_required.clone();
        while !w_solutions.contains_key(&steps) {
            // find the number of steps required for the next integer and save the solutions
            let find_for: i32 = w_steps_required.len() as i32 + 1;
            // println!("currently calculating collatz series of {}", find_for);
            let mut steps_for: i32 = 0;
            let mut i = find_for;
            while i != 1 {
                if w_steps_required.contains_key(&i) {
                    steps_for += w_steps_required.get(&i).unwrap();
                    break;
                }
                if i % 2 == 0 {
                    i = i / 2
                } else {
                    i = i * 3 + 1
                }
                steps_for += 1
            }
            // println!("found that {} requires {} steps", find_for, steps_for);
            w_steps_required.insert(find_for, steps_for);
            if !w_solutions.contains_key(&steps_for) {
                w_solutions.insert(steps_for, find_for);
            }
        }
        println!("------------------");
        println!("THE SOLUTION IS: {}", w_solutions.get(&steps).unwrap());
        println!("------------------");
    }
}

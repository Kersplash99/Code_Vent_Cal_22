use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::VecDeque;


// Day 06 Challenge for advent of code

// define a unique code length ( to make part 1 easier to comment out :) )

/* Part 1
const CODE_LEN: usize = 4;
   ... */

/* Part 2 */
const CODE_LEN: usize = 14;
/* ... */

fn main() {
    // start the timer
    let now = std::time::Instant::now();

    // open the input file
    let in_file = match File::open(&Path::new("inputs/day_06/input.txt")) {
        Err(why) => panic!("Error opening input file: {}", why),
        Ok(in_file) => in_file,
    };

    // store the data stream of signals sent to device
    let mut data_stream = String::new();

    // setup a bufreader and read the data stream into memory
    io::BufReader::new(&in_file).read_line(&mut data_stream).unwrap();

    // setup a queue that will hold 4 elements which will be the unique signal code
    let mut unique_signals: VecDeque<char> = VecDeque::with_capacity(CODE_LEN);

    // store the index of elements we find the 4 unique signals
    let mut code_idx = 0;

    // loop through the characters in the data stream
    for signal in data_stream.chars() {
        // increment index
        code_idx += 1;

        // is this char in the unique signal queue
        if unique_signals.contains(&signal) {
            // remove all the elements upto and including the first occurrence of the dup signal
            for _ in 0..unique_signals.len() {
                // is the signal we just removed a dupe?
                if signal == unique_signals.pop_front().unwrap() {
                    break; // stop we have removed the dup element
                }
            }
        }

        // add in the signal to the queue
        unique_signals.push_back(signal);

        // do we have 4 unique signals?
        if unique_signals.len() == CODE_LEN {
            break; // our code has been found
        }
    }

    // // print out the signal
    // while ! unique_signals.is_empty() {
    //     print!("{}", unique_signals.pop_front().unwrap());
    // }
    // // print out a new line
    // println!("");

    // print out the index the signal was found
    println!("Code Index: {}", code_idx);

    // print out the time
    println!("Time: {:#?}", now.elapsed());
}

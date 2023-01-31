use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};


// Day 05 Challenge for advent of code

fn main() {
    // start the timer
    let now = std::time::Instant::now();

    // open the input file
    let in_file = match File::open(&Path::new("inputs/day_05/input.txt")) {
        Err(why) => panic!("Error opening input file: {}", why),
        Ok(in_file) => in_file,
    };

    // setup an array of char vectors, each index is a seperate stack
    let mut stacks: [Vec<char>; 9] = [  Vec::new(), Vec::new(), Vec::new(),
                                        Vec::new(), Vec::new(), Vec::new(),
                                        Vec::new(), Vec::new(), Vec::new(), ];

    // read in the composition of the stacks

    // read through the input file
    for line in io::BufReader::with_capacity(325, &in_file).lines() { // set cap to 325 to prevent buffer from eating commands
        // make sure the lines are doing alright.
        if let Ok(line) = line {
            // convert the line into a char vector cause im baby and am scared of strings
            let line_chars: Vec<char> = line.chars().collect();

            // exit out from this loop when the stacks have been built
            if line_chars[1] == '1' {
                break;
            }

            // for every stack see if there was any element placed on the stack
            for x in 0..9 {

                // what offset would we expect a box?
                let offset = x + 1 + ( x * 3 );

                // is there a box? well... add it in 
                if !line_chars[offset].is_whitespace() {
                    // add them to the bottom of the stack as we read from the top
                    stacks[x].insert(0, line_chars[offset]);
                }
            }

        }   
    }

    // orchestrate moves
    for line in io::BufReader::new(&in_file).lines() {
        // make sure the lines are doing alright.
        if let Ok(line) = line {
            // split the command into pieces
            let mov_cmd: Vec<&str> = line.split(' ').collect();
            // num boxes -> idx 1, from -> idx 3, to -> idx 5

            /* Part 1:

            // move it! move it! 
            for _ in 0..mov_cmd[1].parse::<usize>().unwrap() {
                // get box it should move from
                let to_move = stacks[ mov_cmd[3].parse::<usize>().unwrap() - 1 ].pop().unwrap();
                // put in stack it should move too 
                stacks[ mov_cmd[5].parse::<usize>().unwrap() - 1 ].push( to_move );
            } */

            /* Part 2: */
            
            // get the last index of the stack that boxes are moved too
            let stack_len = stacks[ mov_cmd[5].parse::<usize>().unwrap() - 1 ].len();

            // move it! move it! 
            for _ in 0..mov_cmd[1].parse::<usize>().unwrap() {
                // get box it should move from
                let to_move = stacks[ mov_cmd[3].parse::<usize>().unwrap() - 1 ].pop().unwrap();
                // put in stack it should move too at the recorded index
                stacks[ mov_cmd[5].parse::<usize>().unwrap() - 1 ].insert( stack_len, to_move );
            } 

            /* ... */
        }   
    }

    // print out all the top most boxes
    for mut stack in stacks {
        print!("{}", stack.pop().unwrap());
    }

    // print a new line
    println!("");

    // print out the time
    println!("Time: {:#?}", now.elapsed());
}

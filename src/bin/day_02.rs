use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};


// Day 02 Challenge for advent of code

/* Part 1: the scores are the selection + the win/lose/draw score

fn vs_rock(choice: char) -> u32 {
    match choice {
        'X' => 4,       // draw
        'Y' => 8,       // win
        'Z' => 3,       // lose
        _ => 0,
    }
}

fn vs_paper(choice: char) -> u32 {
    match choice {
        'X' => 1,       // lose
        'Y' => 5,       // draw
        'Z' => 9,       // win
        _ => 0,
    }
}


fn vs_scissor(choice: char) -> u32 {
    match choice {
        'X' => 7,       // win
        'Y' => 2,       // lose
        'Z' => 6,       // draw
        _ => 0,
    }
}
*/

/* Part 2 */
fn vs_rock(choice: char) -> u32 {
    match choice {
        'X' => 3,       // scissors
        'Y' => 4,       // rock
        'Z' => 8,       // paper
        _ => 0,
    }
}

fn vs_paper(choice: char) -> u32 {
    match choice {
        'X' => 1,       // rock
        'Y' => 5,       // paper
        'Z' => 9,       // scissors
        _ => 0,
    }
}


fn vs_scissor(choice: char) -> u32 {
    match choice {
        'X' => 2,       // paper
        'Y' => 6,       // scissors
        'Z' => 7,       // rock
        _ => 0,
    }
}

fn main() {
    // start the timer
    let now = std::time::Instant::now();

    // keep track of the score
    let mut score: u32 = 0;

    // open the input file
    let in_file = match File::open(&Path::new("inputs/day_02/input.txt")) {
        Err(why) => panic!("Error opening input file: {}", why),
        Ok(in_file) => in_file,
    };

    // read through the input file
    for line in io::BufReader::new(in_file).lines() {
        // make sure the lines are doing alright.
        if let Ok(line) = line {
            
            // convert the line into a char array cause im baby and am scared of strings
            let line_chars: Vec<char> = line.chars().collect();

            // line_chars[0] -> opponents choice
            // line_chars[2] -> our choice

            score += match line_chars[0] {
                'A' => vs_rock(line_chars[2]),
                'B' => vs_paper(line_chars[2]),
                'C' => vs_scissor(line_chars[2]),
                _ => 0,
            }
        }
    }

    println!("Score: {:?}", score);

    // print out the time
    println!("Time: {:#?}", now.elapsed());
}

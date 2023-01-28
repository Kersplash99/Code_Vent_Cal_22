use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};


// Day 04 Challenge for advent of code

// define a section
struct Section {
    start:  u32,
    end:    u32,
}

/* Part 1:

// returns 1 if sections completely overlap else 0
fn is_overlap(section_1: Section, section_2: Section) -> u32 {
    if section_1.start <= section_2.start && section_1.end >= section_2.end {
        1
    } else if section_1.start >= section_2.start && section_1.end <= section_2.end {
        1
    } else {
        0
    }
} 
   ... */

/* Part 2 */
// returns 1 if sections overlap else 0
fn is_overlap(section_1: Section, section_2: Section) -> u32 {
    if section_1.end < section_2.start || section_1.start > section_2.end {
        0
    } else {
        1
    }
} 
/* ... */

fn main() {
    // start the timer
    let now = std::time::Instant::now();

    // keep track of the number of overlaps
    let mut count_overlap: u32 = 0;

    // open the input file
    let in_file = match File::open(&Path::new("inputs/day_04/input.txt")) {
        Err(why) => panic!("Error opening input file: {}", why),
        Ok(in_file) => in_file,
    };

    // read through the input file
    for line in io::BufReader::new(in_file).lines() {
        // make sure the lines are doing alright.
        if let Ok(line) = line {
            
            // split the lines into 2 section assignments
            let mut sections = line.split(",");

            // setup section 1
            let mut section_1_info = sections.next().unwrap().split("-");

            let section_1 = Section {
                start: section_1_info.next().unwrap().parse::<u32>().unwrap(),
                end: section_1_info.next().unwrap().parse::<u32>().unwrap(),
            };

            // setup section 2
            let mut section_2_info = sections.next().unwrap().split("-");

            let section_2 = Section {
                start: section_2_info.next().unwrap().parse::<u32>().unwrap(),
                end: section_2_info.next().unwrap().parse::<u32>().unwrap(),
            };

            // if there is an overlap then add it to the total count
            count_overlap += is_overlap(section_1, section_2);
        }
    }

    println!("Total Count: {:?}", count_overlap);

    // print out the time
    println!("Time: {:#?}", now.elapsed());
}

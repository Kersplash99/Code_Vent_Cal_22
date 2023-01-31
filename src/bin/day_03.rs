use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashSet;

use itertools::Itertools;


// Day 03 Challenge for advent of code

// return the priority of the character
fn calc_priority(item: u32) -> u32 {
    // a - z
    if item >= 97 {
        item % 96 // a -> 1, z -> 26
    } 
    // A - Z 
    else {
        item % 64 + 26 // a -> 1 + 26 (27), z -> 26 + 26 (52)
    }
}


fn main() {
    // start the timer
    let now = std::time::Instant::now();

    // keep track of the priority
    let mut priority: u32 = 0;

    // open the input file
    let in_file = match File::open(&Path::new("inputs/day_03/input.txt")) {
        Err(why) => panic!("Error opening input file: {}", why),
        Ok(in_file) => in_file,
    };

    /* Part 1

    // read through the input file
    for line in io::BufReader::new(in_file).lines() {
        // make sure the lines are doing alright.
        if let Ok(line) = line {
            // convert the first half of the string (compartment 1) into a set
            let compartment_1: HashSet<char> = line[0..(line.len() / 2)].chars().collect();

            // setup an iterator for all the items in the compartment 2
            let compartment_2 = line[(line.len() / 2)..line.len()].chars();

            // check to see if any of the items in compartement 2 are in compartment 1
            for item in compartment_2 {
                if compartment_1.contains(&item) {
                    // add priority to total
                    priority += calc_priority(item as u32);
                    break;
                }
            }
            
        }
    }
    */

    /* Part 2 */
    // read through the input file 3 lines at a time (3 backpacks = 1 group)
    for mut group in &io::BufReader::new(in_file).lines().chunks(3) {

        // setup a hashset with all the items in the first backpack of the group
        let mut common_items: HashSet<char> = group.next().unwrap().expect("REASON").chars().collect();

        // for each succeeding backpack filter out common items to find the groups badge
        for bag in group {
            let next_bag: HashSet<char> = bag.unwrap().chars().collect();
            common_items = &common_items & &next_bag
        }

        // add in the priority of the badges
        for item in common_items {
            priority += calc_priority(item as u32);
        }
    }

    println!("Priority: {:?}", priority);
    // print out the time
    println!("Time: {:#?}", now.elapsed());
}

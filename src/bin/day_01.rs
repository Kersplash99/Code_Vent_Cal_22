use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::BinaryHeap;


// Day 01 Challenge for advent of code
fn main() {
    // start the timer
    let now = std::time::Instant::now();

    // setup max heap 
    let mut heap = BinaryHeap::new();

    // open the input file
    let in_file = match File::open(&Path::new("inputs/day_01/input.txt")) {
        Err(why) => panic!("Error opening input file: {}", why),
        Ok(in_file) => in_file,
    };

    // total cal count for each elf, temp var for use in loop
    let mut total_cal_count: u32 = 0;

    // read through the input file
    for line in io::BufReader::new(in_file).lines() {
        // make sure the lines are doing alright.
        if let Ok(line) = line {
            // read in line
            let cal_count = line.parse::<u32>();

            if let Ok(cal_count) = cal_count {
                // hit a number, add to cal count
                total_cal_count += cal_count;
            } else {
                // hit a empty/incorrect line, add current cal count to heap and reset
                heap.push(total_cal_count);
                // reset for next lil elf
                total_cal_count = 0;
            }
        }
    }

    // part 1
    // peek the first element in the heap
    // println!("Max Cal Count: {}", heap.peek().unwrap()); 

    // part 2
    // pop the top three elements from the max heap, and print out their sum
    let mut sum: u32 = 0;

    sum += heap.pop().unwrap();
    sum += heap.pop().unwrap();
    sum += heap.pop().unwrap();

    println!("Max Aggregate Cal Count: {}", sum);

    // print out the time
    println!("Time: {:#?}", now.elapsed());
}

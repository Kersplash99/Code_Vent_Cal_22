use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};


// Day 07 Challenge for advent of code: special thx to NathanielBumppo for their help on twitch :)

// define a directory
#[derive(Clone)]
struct Dir {
    // inum :)
    inum:       usize,
    // name
    name:       String,
    // total size
    size:       u32,

    // link to parents
    super_dir:  usize,

    // link to all children
    sub_dirs:   Vec<usize>,
}

// calculates directory size and returns it
fn calc_size(inum: usize, dir_list: &Vec<Dir>) -> u32 {
    // if there are no sub directories then we can leave
    if dir_list[inum].sub_dirs.is_empty() {
        // see ya!
        return dir_list[inum].size;

    };

    let mut sub_dir_size = 0;
    // calculate the size of all of the sub dirs of this dir recursively
    for sub_dir in &dir_list[inum].sub_dirs {
        sub_dir_size += calc_size(*sub_dir, dir_list);
    }

    // return the size of all sub dirs and this dir
    dir_list[inum].size + sub_dir_size
} 

// main func
fn main() {
    // start the timer
    let now = std::time::Instant::now();

    // open the input file
    let in_file = match File::open(&Path::new("inputs/day_07/input.txt")) {
        Err(why) => panic!("Error opening input file: {}", why),
        Ok(in_file) => in_file,
    };

    // define the root of the dir structure
    let root_dir = Dir {
        inum:       0,
        name:       "/".to_string(),
        size:       0,
        super_dir:  0,
        sub_dirs:   Vec::new(),
    };

    // keep track of all the listed directories and add in root directory
    let mut dir_list: Vec<Dir> = Vec::new();
    dir_list.push(root_dir);

    // keep track of our current directory
    let mut curr_dir: usize = 0;

    // read through the input file
    for line in io::BufReader::new(in_file).lines() {
        // make sure the lines are doing alright.
        if let Ok(line) = line {
            // split up the line into tokens
            let cmd_tokens: Vec<&str> = line.split(" ").collect();

            // Note(Ammar): valid tokens:
            // "$", "cd", "/"
            // "$", "ls"
            // "dir", "a"
            // "14848514", "b.txt"

            // is this a command?
            if cmd_tokens[0] == "$" {
                // is it the change dir command?
                if cmd_tokens[1] == "cd" {
                    // go to root dir?
                    if cmd_tokens[2] == "/" {
                        // set to root directory
                        curr_dir = 0;
                    } 
                    // go to parent dir? 
                    else if cmd_tokens[2] == ".." {
                        curr_dir = dir_list[curr_dir].super_dir;
                    } 
                    // go to sub dir?
                    else {
                        // check to see if we have created the sub directory already
                        for inum in &dir_list[curr_dir].sub_dirs {
                            if cmd_tokens[2] == dir_list[*inum].name {
                                curr_dir = *inum;
                                // set directory and leave
                                continue;
                            }
                        }

                        // we dont so make a new one so create and move to new sub dir
                        let new_dir = Dir {
                            inum:       dir_list.len(),
                            name:       cmd_tokens[2].to_string(),
                            size:       0, 
                            super_dir:  curr_dir,   
                            sub_dirs:   Vec::new(),
                        };

                        // add to the current directories sub directories
                        curr_dir = new_dir.inum;
                        dir_list.push(new_dir.clone());
                        dir_list[new_dir.super_dir].sub_dirs.push(curr_dir);
                    }
                }
                // dont check for ls as we can safely ignore it 
            }
            // must be a directory listing
            else {
                // if its a number then its a file size, ignore dir listings
                if let Ok(file_size) = cmd_tokens[0].parse::<u32>() {
                    // add to the directories size
                    dir_list[curr_dir].size += file_size;
                }
            }
        }
    }

    /* Part 1
    
    // size of all the deletable directories
    let mut deletable_size: u32 = 0;
    
    // calculate the size of every directory
    for dir in &dir_list {
        let dir_size = calc_size(dir.inum, &dir_list);
        // if its less that 100000 add it to the deletable size
        if dir_size <= 100000 {
            deletable_size += dir_size;
        }
    }

    println!("Deletable Size: {:?}", deletable_size);
       ... */

    /* Part 2 */

    // the space required for a system update -> 30000000 - (70000000 - size of root dir)
    let req_space =  calc_size(0, &dir_list) - 40000000;
    // size of smallest directory to delete, init to maximum size of system
    let mut min_size = 70000000; 

    // index through each directory
    for dir in &dir_list {
        // recursively calculate size
        let dir_size = calc_size(dir.inum, &dir_list);
        
        // if the directory size is more than the required size
        if dir_size >= req_space{
            // if this directory smaller than the last directory
            min_size = dir_size.min(min_size);
        }
    }

    println!("Size of smallest directory to delete: {:?}", min_size);
    /* ... */

    // print out the time
    println!("Time: {:#?}", now.elapsed());
}

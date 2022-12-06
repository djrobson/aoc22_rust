use std::fs;
use std::io::{self, Read};
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() -> io::Result<()> {
    // Open the file in read-only mode
    let mut file = fs::File::open("input06.txt")?;

    // Create a new vec to store the contents of the file
    let mut contents: Vec<u8> = Vec::new();

    // Read the entire file into the vec
    file.read_to_end(&mut contents)?;

    // preload the counter with the length of the window
    let mut count = 13;

    // iterate through the vec in groups of 14
    for seq in contents.windows(14) {
        count += 1;
        let h: HashSet<&u8> = HashSet::from_iter(seq);
        if h.len() == 14 { // did we find 14 unique entries?
                println!("found new seq {:?} at {}", seq, count);
                break;
        }
    }

    Ok(())
}
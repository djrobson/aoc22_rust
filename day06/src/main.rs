use std::fs;
use std::io::{self, Read};
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() -> io::Result<()> {
    // Open the file in read-only mode
    let mut file = fs::File::open("input06.txt")?;

    // Create a new string to store the contents of the file
    let mut contents: Vec<u8> = Vec::new();

    // Read the entire file into the string
    file.read_to_end(&mut contents)?;

    let mut count = 13;

    // Print each line
    for seq in contents.windows(14) {
        count += 1;
        let h: HashSet<&u8> = HashSet::from_iter(seq);
        if h.len() == 14 {
                println!("found new seq {:?} at {}", seq, count);
                break;

        }
    }

    Ok(())
}
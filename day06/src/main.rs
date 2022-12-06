use std::fs;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Open the file in read-only mode
    let mut file = fs::File::open("input06.txt")?;

    // Create a new string to store the contents of the file
    let mut contents: Vec<u8> = Vec::new();

    // Read the entire file into the string
    file.read_to_end(&mut contents)?;

    let mut count = 3;

    // Print each line
    for seq in contents.windows(4) {
           count += 1;
        if seq[0] != seq[1] && seq[0] != seq[2] && seq[0] != seq[3] &&
           seq[1] != seq[2] && seq[1] != seq[3] &&
           seq[2] != seq[3] {
                println!("found new seq {:?} at {}", seq, count);
                break;
           }
    }

    Ok(())
}
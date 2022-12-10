use trees;
use std::fs;
use std::io;

struct TFile {
    name: String,
    size: usize,
}
struct TNode{
    name: String,
    size: Option<usize>,
    files: Vec<TFile>,
}
enum PState {
    ListingFiles,
    ReadingCommands,
}

fn main() -> io::Result<()> {

    let file_contents = fs::read_to_string("input07.txt").expect("Unable to read file");
    let mut tree: trees::Tree<TNode> = trees::Tree::new(TNode { name: "/".to_string(), size: None, files: Vec::new()});
    let mut cursor = tree.root();
    let mut ps = PState::ReadingCommands;

    for line in file_contents.lines() {
        match ps {
            PState::ReadingCommands => {
                if line.starts_with("$ ") {
                    // check the command
                    if line.starts_with("$ cd ") {
                        // go down or up
                        if line == "$ cd .." {
                            cursor = cursor.parent().unwrap();
                        } else if line == "$ cd /" {
                            cursor = tree.root();
                        } else {
                            for child in cursor.into_iter() {
                                if child.data().name == line.trim_start_matches("$ cd ") {
                                    cursor = child;
                                }
                            }
                        }
                    } else if line.starts_with("$ ls"){
                        ps = PState::ListingFiles;
                    } else {
                        panic!(); 
                    }
                }
            },
            PState::ListingFiles => {
                let first = line.split_whitespace().next().unwrap();
                let second = line.split_whitespace().next().unwrap();
                match first {
                    "dir" => cursor.push_front(trees::Tree::new(TNode{name: second.to_string(), size: None, files: Vec::new()})),
                    _ => if let Ok(sz) = first.parse::<usize>() {
                            cursor.data().files.push(TFile {name: second.to_string(), size: sz})}
                        else {
                            panic!();
                        },
                }

            },
        }
    }


    Ok(())
}
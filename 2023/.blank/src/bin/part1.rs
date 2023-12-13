use anyhow::Result;
use regex::Regex;
use std::{fs::File, io::Read};

fn main() -> Result<()> {
    // read in the inputs of the file to the contents string
    let mut input = File::open(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "input1.txt"))?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let out = parse_things(contents);
    println!("Output: {}", out);
    Ok(())
}

fn parse_things(contents: String) -> usize {
    0
}

#[cfg(test)]
#[path = "../test/test1.rs"]
mod test1;

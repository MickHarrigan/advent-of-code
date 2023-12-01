use anyhow::Result;
use std::{fs::File, io::Read};

// Correct!
fn main() -> Result<()> {
    let mut input = File::open(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "input1.txt"))?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let out = calibrate(contents);
    println!("Found value to be: {}", out);

    Ok(())
}

fn calibrate(contents: String) -> usize {
    contents.lines().map(|s| find_digits(s)).sum()
}

fn find_digits(s: &str) -> usize {
    let mut sum = 0;

    // zero does not appear in the items in any capacity
    let mut recent = None;
    for ch in s.chars() {
        match ch {
            x if x.is_digit(10) => {
                if recent.is_none() {
                    sum += x.to_digit(10).unwrap() * 10;
                }
                recent = Some(x.to_digit(10).unwrap());
            }
            _ => {}
        }
    }
    // almost certain that this doesn't need to be unwrap_or, but I have it just in case
    sum += recent.unwrap_or(0);
    sum as usize
}

#[cfg(test)]
#[path = "../test/test1.rs"]
mod test1;

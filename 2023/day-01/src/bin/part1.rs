use anyhow::Result;
use std::{fs::File, io::Read};

// Correct!
// but this is also inefficient as all hell
fn main() -> Result<()> {
    let mut input = File::open("day-01/input1.txt")?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let out = calibrate(contents);
    println!("Found value to be: {}", out);

    Ok(())
}

fn calibrate(contents: String) -> usize {
    contents
        .split_ascii_whitespace()
        .map(|s| find_digits(s))
        .sum()
}

fn find_digits(s: &str) -> usize {
    let mut l = 0;
    let mut r = s.len() - 1;
    let mut a = 0;
    let mut b = 0;

    let mut end = false;
    while !end {
        match s.chars().nth(l) {
            Some(x) if x.is_digit(10) => {
                a = x.to_digit(10).unwrap();
                end = true;
            }
            Some(_) => l += 1,
            None => end = true,
        }
    }
    end = false;
    while !end {
        match s.chars().nth(r) {
            Some(x) if x.is_digit(10) => {
                b = x.to_digit(10).unwrap();
                end = true;
            }
            Some(_) => r -= 1,
            None => end = true,
        }
    }
    (a * 10 + b) as usize
}

#[cfg(test)]
#[path = "../test/test1.rs"]
mod test1;

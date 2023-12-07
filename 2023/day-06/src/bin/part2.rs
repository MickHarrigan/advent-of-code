use anyhow::Result;
use regex::Regex;
use std::{fs::File, io::Read};

fn main() -> Result<()> {
    // read in the inputs of the file to the contents string
    let mut input = File::open(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "input1.txt"))?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let out = parse_races(contents);
    println!("Output: {}", out);
    Ok(())
}

fn parse_races(contents: String) -> usize {
    let re = Regex::new(r"\w:(?<nums>(\s+\d+)+)").unwrap();
    let race: Vec<usize> = contents
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            caps["nums"]
                .trim()
                .split_whitespace()
                .fold(String::new(), |acc, x| acc + x)
                .parse::<usize>()
                .unwrap()
        })
        .collect();

    // literally the quadratic formula
    let pos = (race[0] + ((race[0].pow(2) - 4 * race[1]) as f32).sqrt() as usize) / 2;
    let neg = (race[0] - ((race[0].pow(2) - 4 * race[1]) as f32).sqrt() as usize) / 2;
    pos - neg
}

#[cfg(test)]
#[path = "../test/test2.rs"]
mod test2;

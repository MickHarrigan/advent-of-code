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
    let races: Vec<Vec<usize>> = contents
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            caps["nums"]
                .trim()
                .split_whitespace()
                .filter_map(|num| num.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .collect();
    let races: Vec<(usize, usize)> = races[0]
        .iter()
        .zip(races[1].iter())
        .map(|(&a, &b)| (a, b))
        .collect();
    races.iter().map(count_wins).product()
}

fn count_wins(td: &(usize, usize)) -> usize {
    let time = td.0;
    let dist = td.1;
    (1..(time - 1))
        .map(|charge| (time - charge) * charge)
        .filter(|d| d > &dist)
        .count()
}

#[cfg(test)]
#[path = "../test/test1.rs"]
mod test1;

use anyhow::Result;
use core::panic;
use regex::Regex;
use std::{collections::HashMap, fs::File, io::Read, str::FromStr};

const AMOUNTS: [(Color, u32); 3] = [(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)];

// options for the colors
#[derive(Hash, Eq, PartialEq, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(()),
        }
    }
}

// man this is a mess, I can check back on it later to clean it up
fn main() -> Result<()> {
    // read in the inputs of the file to the contents string
    let mut input = File::open(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "input1.txt"))?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let out = parse_games(contents);
    println!("Output: {}", out);
    Ok(())
}

fn parse_games(contents: String) -> u32 {
    let re = Regex::new(r"Game \d+:\s*(.*)$").unwrap();
    let items: Vec<(u32, HashMap<Color, u32>)> = contents
        .lines()
        .zip(1u32..)
        .map(|(line, ind)| {
            // each line should become a vec of tuples for colors and amounts
            let amounts = re
                .captures(line)
                .and_then(|cap| {
                    cap.get(1).and_then(|results| {
                        let mut counts: HashMap<Color, u32> =
                            HashMap::from([(Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)]);
                        for pull in results.as_str().split(';').into_iter() {
                            // pull = 12 red, 6 green
                            // split on the commas to get each color
                            for cube in pull.trim().split(',') {
                                let num = cube
                                    .split_whitespace()
                                    .nth(0)
                                    .unwrap()
                                    .parse::<u32>()
                                    .unwrap();
                                let color =
                                    Color::from_str(cube.split_whitespace().nth(1).unwrap())
                                        .unwrap();

                                if let Some(count) = counts.get_mut(&color) {
                                    *count = std::cmp::max(num, *count);
                                }
                            }
                        }
                        Some(counts)
                    })
                })
                .unwrap();
            (ind, amounts)
        })
        .collect();
    let out: Vec<&(u32, HashMap<Color, u32>)> = items
        .iter()
        .filter(|(ind, counts)| {
            AMOUNTS
                .iter()
                .all(|pair| counts.get(&pair.0).unwrap() <= &pair.1)
        })
        .collect();

    let sum = out.iter().fold(0, |acc, (ind, _)| acc + ind);
    for each in out.iter() {
        println!("{:?}", each);
    }

    sum
}

#[cfg(test)]
#[path = "../test/test1.rs"]
mod test1;

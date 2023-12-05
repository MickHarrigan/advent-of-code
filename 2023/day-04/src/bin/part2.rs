use anyhow::Result;
use regex::Regex;
use std::{collections::HashSet, fs::File, io::Read};

fn main() -> Result<()> {
    // read in the inputs of the file to the contents string
    let mut input = File::open(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "input1.txt"))?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let out = parse_cards(contents);
    println!("Output: {}", out);
    Ok(())
}

fn parse_cards(contents: String) -> usize {
    // with each line, parse it as a card and get the winning numbers and the other numbers
    let re = Regex::new(r"Card\s+\d+:\s+(?<winning>(\d+\s+)+)\|(?<numbers>(\s+\d+)+)").unwrap();

    let mut cards = contents
        .lines()
        .map(|line| {
            // this changes the lines to a card
            re.captures(line)
                .and_then(|cap| {
                    // Some(Card {
                    let winning: HashSet<_> = cap["winning"]
                        .trim()
                        .split_whitespace()
                        .filter_map(|each| each.parse::<usize>().ok())
                        .collect();
                    Some(
                        cap["numbers"]
                            .trim()
                            .split_whitespace()
                            .filter_map(|each| each.parse::<usize>().ok())
                            // there may be a better way to do this fold
                            .fold(0usize, |acc, x| {
                                if winning.get(&x).is_some() {
                                    acc + 1
                                } else {
                                    acc
                                }
                            }),
                    )
                })
                // this removes all the Some() from the flatmapping
                .unwrap()
        })
        .zip(std::iter::repeat(1usize))
        .collect::<Vec<_>>();

    // update the amounts that each card has based on the amount of matches it has
    for i in 0..cards.len() {
        let matches = cards[i].0;
        let count = cards[i].1;

        for _ in 0..count {
            for j in 1..=matches {
                cards[j + i].1 += 1;
            }
        }
    }

    cards.into_iter().fold(0, |acc, x| acc + x.1)
}

#[cfg(test)]
#[path = "../test/test2.rs"]
mod test2;

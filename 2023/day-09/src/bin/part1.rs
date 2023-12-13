use anyhow::Result;
use std::{fs::File, io::Read};

fn main() -> Result<()> {
    // read in the inputs of the file to the contents string
    let mut input = File::open(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "input1.txt"))?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let data = parse_data(contents);
    let out = solve(data);

    println!("Output: {}", out);
    Ok(())
}

fn parse_data(contents: String) -> Vec<Vec<isize>> {
    // get each line and split on each space
    contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|word| word.parse::<isize>().ok())
                .collect()
        })
        .collect()
}

fn solve(mut data: Vec<Vec<isize>>) -> isize {
    // given a vec of each list of data points
    // get windows of 2 on each list until all zeroes
    // put each last number into a stack
    data.iter_mut()
        .map(|datum| {
            let mut stack = Vec::new();
            while datum.iter().any(|value| value != &0) {
                stack.push(datum.last().cloned().unwrap());
                *datum = datum.windows(2).map(|win| win[1] - win[0]).collect();
            }
            stack.iter().sum::<isize>()
        })
        .sum()
}

#[cfg(test)]
#[path = "../test/test1.rs"]
mod test1;

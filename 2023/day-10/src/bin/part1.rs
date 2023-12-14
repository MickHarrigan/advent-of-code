use anyhow::Result;
use regex::Regex;
use std::{collections::HashSet, fs::File, io::Read};

fn main() -> Result<()> {
    // read in the inputs of the file to the contents string
    let mut input = File::open(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "input1.txt"))?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let data = parse_pipes(contents);
    let out = solve(data);
    println!("Output: {}", out);
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Pipes {
    Start,
    Empty,
    Vertical,
    Horizontal,
    NE,
    NW,
    SE,
    SW,
}

// this is to determine how the pipes can be connected directionally
#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Pipes {
    fn can_connect(&self, rhs: &Pipes, dir: Direction) -> bool {
        // have a list of items that they can connect with
        // this can most likely be condensed, but I will check it out later
        match self {
            Self::Empty => vec![],
            Self::Start => match dir {
                Direction::Up => vec![Self::Vertical, Self::SW, Self::SE, Self::Start],
                Direction::Down => vec![Self::Vertical, Self::NW, Self::NE, Self::Start],
                Direction::Right => vec![Self::Horizontal, Self::SW, Self::NW, Self::Start],
                Direction::Left => vec![Self::Horizontal, Self::NE, Self::SE, Self::Start],
            },
            Self::Vertical => match dir {
                Direction::Up => vec![Self::SE, Self::SW, Self::Vertical, Self::Start],
                Direction::Down => vec![Self::NE, Self::NW, Self::Vertical, Self::Start],
                _ => vec![],
            },
            Self::Horizontal => match dir {
                Direction::Left => vec![Self::SE, Self::NE, Self::Horizontal, Self::Start],
                Direction::Right => vec![Self::SW, Self::NW, Self::Horizontal, Self::Start],
                _ => vec![],
            },
            Self::NE => match dir {
                Direction::Up => vec![Self::SE, Self::SW, Self::Vertical, Self::Start],
                Direction::Right => vec![Self::SW, Self::NW, Self::Horizontal, Self::Start],
                _ => vec![],
            },
            Self::NW => match dir {
                Direction::Up => vec![Self::SE, Self::SW, Self::Vertical, Self::Start],
                Direction::Left => vec![Self::NE, Self::SE, Self::Horizontal, Self::Start],
                _ => vec![],
            },
            Self::SE => match dir {
                Direction::Right => vec![Self::NW, Self::SW, Self::Horizontal, Self::Start],
                Direction::Down => vec![Self::NE, Self::NW, Self::Vertical, Self::Start],
                _ => vec![],
            },
            Self::SW => match dir {
                Direction::Left => vec![Self::SE, Self::NE, Self::Horizontal, Self::Start],
                Direction::Down => vec![Self::NE, Self::NW, Self::Vertical, Self::Start],
                _ => vec![],
            },
        }
        .contains(rhs)
    }
}

fn parse_pipes(contents: String) -> (Vec<Vec<Pipes>>, (usize, usize)) {
    // origin is top left corner
    // i => vertical
    // j => horizontal
    let mut start = (0, 0);
    (
        contents
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.char_indices()
                    .map(|(j, ch)| {
                        // each char is either [pipe, nothing, start]
                        // didn't want to do the whole from_str thing here since . exists and isn't *really* a pipe
                        match ch {
                            '.' => Pipes::Empty,
                            'S' => {
                                start = (i, j);
                                Pipes::Start
                            }
                            '|' => Pipes::Vertical,
                            '-' => Pipes::Horizontal,
                            'L' => Pipes::NE,
                            'J' => Pipes::NW,
                            '7' => Pipes::SW,
                            'F' => Pipes::SE,
                            _ => unreachable!(),
                        }
                    })
                    .collect()
            })
            .collect(),
        start,
    )
}

fn solve(data: (Vec<Vec<Pipes>>, (usize, usize))) -> usize {
    // given the pipes and the start location,
    // loop all the way through until you reach the start again
    let (pipes, start) = data;

    let mut count = 1;
    let mut prev = start;
    let mut location = find_connected(&pipes, start)
        .iter()
        .next()
        .cloned()
        .unwrap();

    while location != start {
        // update location on each pipe taken
        for connection in find_connected(&pipes, location).into_iter() {
            if connection != prev {
                prev = location;
                location = connection;
                count += 1;
                // hate that i used break, but otherwise it would get stuck
                break;
            }
        }
    }
    count / 2
}

fn find_connected(pipes: &Vec<Vec<Pipes>>, location: (usize, usize)) -> Vec<(usize, usize)> {
    // takes a reference to the pipes and a location and returns all (2 for any pipe other than S which could have up to 4)
    // pipes that are connected to the current one
    let (i, j) = location;
    let pipe = &pipes[i][j];
    let mut connected = Vec::new();
    let (max_i, max_j) = (pipes.len(), pipes[0].len());

    // check above, below, left and right if they exist
    if i > 0 && pipe.can_connect(&pipes[i - 1][j], Direction::Up) {
        connected.push((i - 1, j));
    }
    if i < max_i - 1 && pipe.can_connect(&pipes[i + 1][j], Direction::Down) {
        connected.push((i + 1, j));
    }
    if j > 0 && pipe.can_connect(&pipes[i][j - 1], Direction::Left) {
        connected.push((i, j - 1));
    }
    if j < max_j - 1 && pipe.can_connect(&pipes[i][j + 1], Direction::Right) {
        connected.push((i, j + 1));
    }
    connected
}

#[cfg(test)]
#[path = "../test/test1.rs"]
mod test1;

use anyhow::Result;
use rayon::prelude::*;
use regex::Regex;
use std::{
    fs::File,
    io::Read,
    sync::{Arc, Mutex},
};

fn main() -> Result<()> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(6)
        .build_global()?;
    // read in the inputs of the file to the contents string
    let mut input = File::open(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "input1.txt"))?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let maps = parse_maps(contents);
    let out = solve(maps);
    println!("Output: {}", out);
    Ok(())
}

struct Maps {
    a2b: usize,
    ranges: Vec<Vec<((u64, u64), (u64, u64))>>,
    seeds: Vec<(u64, u64)>,
    amount: u64,
}

fn parse_maps(contents: String) -> Maps {
    let seeds_re = Regex::new(r"^seeds:\s+(?<seeds>.*)").unwrap();
    let maps_re =
        Regex::new(r"(?<source>(\w*))\-to\-(?<dest>(.+))\s+map:\n(?<map>((\d+\s?){3})+)").unwrap();

    // this is the list of seeds in a string
    let mut seeds: Vec<u64> = seeds_re.captures(contents.as_str()).unwrap()["seeds"]
        .to_string()
        .split_whitespace()
        .filter_map(|seed| seed.parse().ok())
        .collect();
    // convert the seeds to a vec of pairs
    // for seed in seeds.iter() {

    // }
    let seeds: Vec<(u64, u64)> = seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1] - 1))
        .collect();
    let amount = seeds.iter().fold(0, |acc, (_, len)| acc + len);
    // println!("{:?}", seeds);

    let mut a2b = 0;
    let mut map_values: Vec<Vec<Vec<u64>>> = Vec::new();
    for caps in maps_re.captures_iter(contents.as_str()) {
        a2b += 1;
        // read each map value as a vec of a vec of len 3

        map_values.push(
            caps.name("map")
                .unwrap()
                .as_str()
                .trim()
                .split('\n')
                .map(|list_of_three| {
                    list_of_three
                        .split_whitespace()
                        .filter_map(|num| num.parse::<u64>().ok())
                        .collect()
                })
                .collect(),
        );
    }

    // this is the whole of the map in a vec
    Maps {
        a2b,
        ranges: maps_to_filters(map_values),
        seeds,
        amount,
    }
}

fn maps_to_filters(mut map_values: Vec<Vec<Vec<u64>>>) -> Vec<Vec<((u64, u64), (u64, u64))>> {
    // first find the range of values that apply for the given map
    // this is done by finding the max and min of each map
    let mut filters: Vec<Vec<((u64, u64), (u64, u64))>> = Vec::new();
    for ranges in map_values.iter_mut() {
        // sort the ranges by the source
        ranges.sort_by(|a, b| {
            let [_, src_a, _] = a[..] else {
                panic!("Got a line with more than 3 items in it!")
            };
            let [_, src_b, _] = b[..] else {
                panic!("Got a line with more than 3 items in it!")
            };
            src_a.cmp(&src_b)
        });
        let mut inner = Vec::new();
        ranges.iter_mut().for_each(|range| {
            // convert each range (dest, src, len)
            // into a tuple that shows each range of src to dest

            let [dest, src, len] = range[..] else {
                panic!("Got a line with more than 3 items in it!")
            };
            inner.push(((src, src + len - 1), (dest, dest + len - 1)));
        });
        filters.push(inner);
    }

    filters
}

fn solve(maps: Maps) -> u64 {
    // now that I have the filters, get each seed into it and see what the next layer will give
    // then run this again with each filter and the output from the previous to get a list of locations
    // then find the min of that

    // for each pair in seeds
    // make a thread that runs the algorithm as before
    // then join them and run min on the outputs of that
    let outer = Arc::new(Mutex::new(Vec::new()));
    maps.seeds.par_iter().for_each(|(start, end)| {
        {
            // expand the pair into input as a vec of seeds to work on
            let mut input: Vec<u64> = (*start..=*end).collect();
            for i in 0..input.len() {
                for j in 0..maps.a2b {
                    input[i] = check_mapping(input[i], &maps.ranges[j]);
                }
            }
            let min = input.into_iter().min().unwrap();
            let mut out = outer.lock().unwrap();
            out.push(min);
            // outer.lock().unwrap().push(input.into_iter().min().unwrap());
        }
    });
    let min = {
        let lock = outer.lock().unwrap();
        let val = lock.iter().cloned().min().unwrap();
        val
    };
    // outer.lock().unwrap().iter().cloned().min().unwrap()
    min
}

fn check_mapping(input: u64, filter: &Vec<((u64, u64), (u64, u64))>) -> u64 {
    // converts the input type to the next level via the filter
    match filter
        .iter()
        .find(|(src, _)| input >= src.0 && input <= src.1)
    {
        Some((src, dest)) => {
            let offset = input - src.0;
            dest.0 + offset
        }
        None => input,
    }
}

#[cfg(test)]
#[path = "../test/test2.rs"]
mod test2;

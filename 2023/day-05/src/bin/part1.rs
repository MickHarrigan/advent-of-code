use anyhow::Result;
use regex::Regex;
use std::{collections::HashSet, fs::File, io::Read, str::FromStr};

fn main() -> Result<()> {
    // read in the inputs of the file to the contents string
    let mut input = File::open(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "input1.txt"))?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let maps = parse_maps(contents);
    let out = solve(maps);
    println!("Output: {}", out);
    Ok(())
}

#[derive(Debug)]
enum Almanac {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Almanac {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "seed" => Ok(Almanac::Seed),
            "soil" => Ok(Almanac::Soil),
            "fertilizer" => Ok(Almanac::Fertilizer),
            "water" => Ok(Almanac::Water),
            "light" => Ok(Almanac::Light),
            "temperature" => Ok(Almanac::Temperature),
            "humidity" => Ok(Almanac::Humidity),
            "location" => Ok(Almanac::Location),
            _ => unreachable!(),
        }
    }
}

struct Maps {
    a2b: Vec<(Almanac, Almanac)>,
    ranges: Vec<Vec<((u64, u64), (u64, u64))>>,
    seeds: Vec<u64>,
}

fn parse_maps(contents: String) -> Maps {
    let seeds_re = Regex::new(r"^seeds:\s+(?<seeds>.*)").unwrap();
    let maps_re =
        Regex::new(r"(?<source>(\w*))\-to\-(?<dest>(.+))\s+map:\n(?<map>((\d+\s?){3})+)").unwrap();

    // this is the list of seeds in a string
    let seeds: Vec<u64> = seeds_re.captures(contents.as_str()).unwrap()["seeds"]
        .to_string()
        .split_whitespace()
        .filter_map(|seed| seed.parse().ok())
        .collect();

    let mut almanac_maps_str = Vec::new();
    let mut map_values: Vec<Vec<Vec<u64>>> = Vec::new();
    for caps in maps_re.captures_iter(contents.as_str()) {
        almanac_maps_str.push((
            caps.name("source").unwrap().as_str(),
            caps.name("dest").unwrap().as_str(),
        ));
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
        a2b: almanac_maps_str
            .into_iter()
            .map(|(a, b)| (Almanac::from_str(a).unwrap(), Almanac::from_str(b).unwrap()))
            .collect(),
        ranges: maps_to_filters(map_values),
        seeds,
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

    // now that I have the filters, get each seed into it and see what the next layer will give
    // then run this again with each filter and the output from the previous to get a list of locations
    // then find the min of that

    filters
}

fn solve(mut maps: Maps) -> usize {
    // using the maps' filters and seeds will find the min value
    todo!();
}

#[cfg(test)]
#[path = "../test/test1.rs"]
mod test1;

use anyhow::Result;
use rayon::prelude::*;
use regex::Regex;
use std::{collections::HashMap, fs::File, io::Read};

fn main() -> Result<()> {
    // read in the inputs of the file to the contents string
    let mut input = File::open(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "input2.txt"))?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let out = parse_maps(contents);
    println!("Output: {}", out);
    Ok(())
}

#[derive(Debug)]
struct Node {
    // the left and right are other node ids that can be traversed to
    left: usize,
    right: usize,
}

fn parse_maps(contents: String) -> usize {
    // parse each line into a node that must be connected with all the other nodes
    // this is a graph problem and must be created as such
    // AAA is the start, ZZZ is the end
    // read in each line and node pairing, sort them by the node name, then count them to create their ids

    let re = Regex::new(r"(?<node>\w+)\s=\s\((?<left>\w+),\s(?<right>\w+)\)").unwrap();
    let patt = Regex::new(r"(?<pattern>\w+)").unwrap();
    let pattern: Vec<char> = contents
        .lines()
        .nth(0)
        .map(|line| patt.captures(line).unwrap()["pattern"].to_string())
        .unwrap()
        .chars()
        .collect();
    let mut nodes: Vec<(String, (String, String))> = contents
        .lines()
        .skip(2)
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps["node"].to_string(),
                (caps["left"].to_string(), caps["right"].to_string()),
            )
        })
        .collect();
    nodes.sort_by(|a, b| a.0.chars().last().cmp(&b.0.chars().last()));
    // nodes.iter().for_each(|node| println!("{:?}", node));

    // now that the nodes are sorted by last letter only,
    // take all those that end in A as the starts

    // hash each node such that there is a mapping from "AAA" -> u64
    let mut hash = HashMap::new();
    let mut starts = Vec::new();
    let mut ends = Vec::new();
    for (node, ind) in nodes.iter().zip(0usize..) {
        if node.0.chars().last().unwrap() == 'A' {
            starts.push(ind);
        }
        if node.0.chars().last().unwrap() == 'Z' {
            ends.push(ind);
        }
        hash.insert(node.0.clone(), ind);
    }

    // starts.iter().for_each(|start| println!("{:?}", start));
    // ends.iter().for_each(|end| println!("{:?}", end));
    let nodes: Vec<Node> = nodes
        .into_iter()
        .map(|node| Node {
            left: hash.get(&node.1 .0).unwrap().clone(),
            right: hash.get(&node.1 .1).unwrap().clone(),
        })
        .collect();

    let mut outs = Vec::new();
    for start in starts.iter_mut() {
        let mut count = 0;
        while !ends.contains(start) {
            // iterate over the nodes until nodes.len() - 1 (ZZZ) is reached
            // need the current pattern and the current node
            let decision = pattern[count % pattern.len()];
            let curr = &nodes[*start];
            // println!("{:?} -> {:?}, ", curr, decision);
            match decision {
                'L' => *start = curr.left,
                'R' => *start = curr.right,
                _ => unreachable!(),
            }
            count += 1;
        }
        outs.push(count);
    }
    lcm(outs)
}

fn lcm(nums: Vec<usize>) -> usize {
    let mut res = 1;
    nums.iter().for_each(|num| res = num * res / gcd(*num, res));
    res
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn at_ends(starts: &Vec<usize>, ends: &Vec<usize>) -> bool {
    // return true when at the ends
    // if all the starts are at ends
    starts.iter().all(|start| ends.contains(start))
}

#[cfg(test)]
#[path = "../test/test2.rs"]
mod test2;

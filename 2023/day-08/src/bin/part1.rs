use anyhow::Result;
use regex::Regex;
use std::{collections::HashMap, fs::File, io::Read};

fn main() -> Result<()> {
    // read in the inputs of the file to the contents string
    let mut input = File::open(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "input1.txt"))?;
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
    nodes.sort_by(|a, b| a.0.cmp(&b.0));

    // hash each node such that there is a mapping from "AAA" -> u64
    let mut hash = HashMap::new();
    for (node, ind) in nodes.iter().zip(0usize..) {
        hash.insert(node.0.clone(), ind);
    }
    let nodes: Vec<Node> = nodes
        .into_iter()
        .map(|node| Node {
            left: hash.get(&node.1 .0).unwrap().clone(),
            right: hash.get(&node.1 .1).unwrap().clone(),
        })
        .collect();

    let mut out = 0;
    let mut ind = 0;
    while ind != nodes.len() - 1 {
        // iterate over the nodes until nodes.len() - 1 (ZZZ) is reached
        // need the current pattern and the current node
        let decision = pattern[out % pattern.len()];
        let curr = &nodes[ind];
        match decision {
            'L' => ind = curr.left,
            'R' => ind = curr.right,
            _ => unreachable!(),
        }
        out += 1;
    }

    out
}

#[cfg(test)]
#[path = "../test/test1.rs"]
mod test1;

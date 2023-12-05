use anyhow::Result;
use std::{collections::HashMap, fs::File, io::Read};

// I feel like a big dumb idiot after this one.
// this is such a mess I cannot believe that I did this lol
fn main() -> Result<()> {
    // read in the inputs of the file to the contents string
    let mut input = File::open(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "input1.txt"))?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let out = parse_schematic(contents);
    println!("Output: {}", out);
    Ok(())
}

// the contents of the schematic are one of three things,
// a number,
// a symbol,
// or a period (nothing)
#[derive(Debug)]
enum Item {
    Number(usize, Region),
    Symbol(Location),
    Gear(Location),
}

// this is the top left and bottom right corners of a rectangle
#[derive(Default, Clone, Debug)]
struct Region {
    // is 2 locations and the connecting area
    min: Location,
    max: Location,
}

impl Region {
    pub fn contains(&self, loc: &Location) -> bool {
        (loc.x >= self.min.x && loc.x <= self.max.x) && (loc.y >= self.min.y && loc.y <= self.max.y)
    }
}

// this is a specific coordinate (or char (i,j) index)
#[derive(Default, Clone, Debug, Eq, PartialEq, Hash)]
struct Location {
    x: u32,
    y: u32,
}

fn parse_schematic(contents: String) -> usize {
    // parse each line into a vec of items
    let mut things: Vec<Vec<Item>> = contents
        .lines()
        .zip(0u32..)
        .map(|(line, line_num)| {
            // iterate across each char, match it as an Item
            let mut num = String::new();
            let mut region = Region::default();
            let mut list = Vec::new();
            for (ind, ch) in line.char_indices() {
                match ch {
                    '.' => {
                        if num.len() != 0 {
                            let val = num.parse::<usize>().unwrap();
                            region.max = Location {
                                x: ind as u32,
                                y: line_num.saturating_add(1),
                            };
                            list.push(Item::Number(val, region.clone()));
                            num.clear();
                            region = Region::default();
                        }
                    }
                    '0'..='9' => {
                        // get the region as well as the number
                        if num.len() == 0 {
                            region.min = Location {
                                x: ind.saturating_sub(1) as u32,
                                y: line_num.saturating_sub(1),
                            };
                        }
                        num.push(ch);
                    }
                    '#' | '%' | '&' | '+' | '-' | '/' | '=' | '@' | '$' => {
                        if num.len() != 0 {
                            let val = num.parse::<usize>().unwrap();
                            region.max = Location {
                                x: ind as u32,
                                y: line_num + 1,
                            };
                            list.push(Item::Number(val, region.clone()));
                            num.clear();
                            region = Region::default();
                        }
                    }
                    '*' => {
                        // this is the case for finding a potential gear
                        if num.len() != 0 {
                            let val = num.parse::<usize>().unwrap();
                            region.max = Location {
                                x: ind as u32,
                                y: line_num + 1,
                            };
                            list.push(Item::Number(val, region.clone()));
                            num.clear();
                            region = Region::default();
                        }
                        list.push(Item::Gear(Location {
                            x: ind as u32,
                            y: line_num,
                        }));
                    }
                    _ => unreachable!(),
                }
            }
            if num.len() != 0 {
                let val = num.parse::<usize>().unwrap();
                region.max = Location {
                    x: line.len() as u32,
                    y: line_num + 1,
                };
                list.push(Item::Number(val, region.clone()));
                num.clear();
                region = Region::default();
            }
            list
        })
        .collect();
    let mut gears: HashMap<&Location, Vec<&usize>> = HashMap::new();
    things.windows(2).for_each(|win| {
        for lhs in win[0].iter() {
            for rhs in win[1].iter() {
                // given lhs is num and rhs is a symbol
                if let Item::Number(num, reg) = lhs {
                    if let Item::Gear(loc) = rhs {
                        if reg.contains(loc) {
                            gears
                                .entry(loc)
                                .and_modify(|e| e.push(num))
                                .or_insert(vec![num]);
                        }
                    }
                // given rhs is num and lhs is a symbol
                } else if let Item::Gear(loc) = lhs {
                    if let Item::Number(num, reg) = rhs {
                        if reg.contains(loc) {
                            gears
                                .entry(loc)
                                .and_modify(|e| e.push(num))
                                .or_insert(vec![num]);
                        }
                    }
                }
            }
        }
    });

    // need the case for checking within its own line
    for list in things.iter() {
        if list.len() >= 2 {
            list.windows(2).for_each(|win| {
                // if there are symbols and numbers next to each other then check if they apply
                if let Item::Number(num, reg) = &win[0] {
                    if let Item::Gear(loc) = &win[1] {
                        if reg.contains(&loc) {
                            gears
                                .entry(loc)
                                .and_modify(|e| e.push(num))
                                .or_insert(vec![num]);
                        }
                    }
                // given rhs is num and lhs is a symbol
                } else if let Item::Gear(loc) = &win[0] {
                    if let Item::Number(num, reg) = &win[1] {
                        if reg.contains(&loc) {
                            gears
                                .entry(loc)
                                .and_modify(|e| e.push(num))
                                .or_insert(vec![num]);
                        }
                    }
                }
            });
        }
    }
    gears
        .into_iter()
        .filter(|x| x.1.len() == 2)
        .map(|x| x.1.into_iter().product::<usize>())
        .sum()
}

#[cfg(test)]
#[path = "../test/test2.rs"]
mod test2;

use anyhow::Result;
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Read,
};

fn main() -> Result<()> {
    let mut input = File::open(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "input2.txt"))?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let out = calibrate(contents);
    println!("Found value to be: {}", out);

    Ok(())
}

fn calibrate(contents: String) -> usize {
    contents.lines().map(|s| find_digits(s)).sum()
}

fn find_digits(s: &str) -> usize {
    // iterate from left and right to compare if there are digits or words that are the numbers that are needed
    let mut word = VecDeque::new();

    let hash: HashMap<String, u32> = HashMap::from([
        ("one".to_owned(), 1),
        ("two".to_owned(), 2),
        ("three".to_owned(), 3),
        ("four".to_owned(), 4),
        ("five".to_owned(), 5),
        ("six".to_owned(), 6),
        ("seven".to_owned(), 7),
        ("eight".to_owned(), 8),
        ("nine".to_owned(), 9),
    ]);

    let mut sum = 0;

    // zero does not appear in the items in any capacity
    let mut recent = None;
    for ch in s.chars() {
        match ch {
            x if x.is_digit(10) => {
                if recent.is_none() {
                    sum += x.to_digit(10).unwrap() * 10;
                }
                recent = Some(x.to_digit(10).unwrap());
            }
            // otherwise
            x => {
                update_word(&mut word, x);
                if let Some(val) = check_word(&word, &hash) {
                    if recent.is_none() {
                        sum += *val * 10;
                    }
                    recent = Some(*val);
                }
            }
        }
    }
    // almost certain that this doesn't need to be unwrap_or, but I have it just in case
    sum += recent.unwrap_or(0);
    sum as usize
}

fn update_word(word: &mut VecDeque<char>, letter: char) {
    // this should update the word and keep only the last 5 items in it
    if word.len() >= 5 {
        word.pop_front();
    }
    word.push_back(letter);
}

fn check_word<'a>(letters: &VecDeque<char>, hash: &'a HashMap<String, u32>) -> Option<&'a u32> {
    // check the last 3,4,5

    let len = letters.len();
    for l in [3, 4, 5].into_iter() {
        if len >= l {
            let word = letters.range((len - l)..).collect::<String>();
            if hash.get(&word).is_some() {
                return hash.get(&word);
            }
        }
    }
    None
}

#[cfg(test)]
#[path = "../test/test2.rs"]
mod test2;

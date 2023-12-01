use anyhow::Result;
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Read,
};

fn main() -> Result<()> {
    let mut input = File::open("day-01/input2.txt")?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let out = calibrate(contents);
    println!("Found value to be: {}", out);

    Ok(())
}

fn calibrate(contents: String) -> usize {
    contents
        .split_ascii_whitespace()
        .map(|s| find_digits(s))
        .sum()
}

fn find_digits(s: &str) -> usize {
    // iterate from left and right to compare if there are digits or words that are the numbers that are needed
    let mut word = VecDeque::new();

    let hash: HashMap<String, u8> = HashMap::from([
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

    let mut l = 0;
    let mut r = s.len() - 1;
    let mut a = 0;
    let mut b = 0;

    let mut end = false;
    while !end {
        match s.chars().nth(l) {
            Some(x) if x.is_digit(10) => {
                a = x.to_digit(10).unwrap();
                end = true;
            }
            Some(letter) => {
                update_word(&mut word, letter);
                if let Some(val) = check_word(&word, &hash, true) {
                    a = *val as u32;
                    end = true;
                }

                l += 1;
            }
            None => end = true,
        }
    }
    end = false;
    word.clear();
    while !end {
        match s.chars().nth(r) {
            Some(x) if x.is_digit(10) => {
                b = x.to_digit(10).unwrap();
                end = true;
            }
            Some(letter) => {
                update_word(&mut word, letter);
                if let Some(val) = check_word(&word, &hash, false) {
                    b = *val as u32;
                    end = true;
                }

                r -= 1;
            }
            None => end = true,
        }
    }
    println!("{}{}", a, b);
    (a * 10 + b) as usize
}

fn update_word(word: &mut VecDeque<char>, letter: char) {
    // this should update the word and keep only the last 5 items in it
    if word.len() >= 5 {
        word.pop_front();
    }
    word.push_back(letter);
}

fn check_word<'a>(
    letters: &VecDeque<char>,
    hash: &'a HashMap<String, u8>,
    forwards: bool,
) -> Option<&'a u8> {
    // check the last 3,4,5

    let len = letters.len();
    for l in [3, 4, 5].into_iter() {
        if len >= l {
            let word: String;
            if forwards {
                word = letters.range((len - l)..).collect::<String>();
            } else {
                // reverses the letters when coming from the back
                word = letters.range((len - l)..).rev().collect::<String>();
            }
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

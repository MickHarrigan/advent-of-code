use super::*;

#[test]
fn example1() {
    let contents = String::from(
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
    );
    assert_eq!(calibrate(contents), 281);
}
#[test]
fn example2() {
    // this was the time where I learned that I was not reading from the right correctly
    let contents = String::from("8oneqzvdcrh");
    assert_eq!(calibrate(contents), 81);
}

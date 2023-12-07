use super::*;

#[test]
fn example() {
    assert_eq!(
        parse_races(String::from(
            "Time:      7  15   30
Distance:  9  40  200"
        )),
        71503
    );
}

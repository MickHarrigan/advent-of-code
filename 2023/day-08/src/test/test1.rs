use super::*;

#[test]
fn example1() {
    let input = String::from(
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
    );

    assert_eq!(parse_maps(input), 2);
}

#[test]
fn example2() {
    let input = String::from(
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
    );

    assert_eq!(parse_maps(input), 6);
}

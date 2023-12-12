use super::*;

#[test]
fn example1() {
    let input = String::from(
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
    );

    assert_eq!(parse_maps(input), 6);
}

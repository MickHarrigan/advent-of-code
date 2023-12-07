use super::*;

#[test]
fn case1() {
    assert_eq!(
        467835,
        parse_schematic(String::from(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        ))
    );
}

#[test]
fn case2() {
    assert_eq!(40, parse_schematic(String::from("2*4*8")));
}

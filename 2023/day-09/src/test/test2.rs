use super::*;

#[test]
fn example() {
    let input = String::from(
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
    );
    assert_eq!(solve(parse_data(input)), 2);
}

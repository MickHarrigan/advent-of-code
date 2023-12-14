use super::*;

#[test]
fn example1a() {
    let input = String::from(
        ".....
.S-7.
.|.|.
.L-J.
.....",
    );
    assert_eq!(solve(parse_pipes(input)), 4);
}

#[test]
fn example1b() {
    let input = String::from(
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
    );
    assert_eq!(solve(parse_pipes(input)), 4);
}

#[test]
fn example2() {
    let input = String::from(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
    );
    assert_eq!(solve(parse_pipes(input)), 8);
}

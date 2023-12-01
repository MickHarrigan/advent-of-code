use super::*;

#[test]
fn example1() {
    let contents = String::from("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");
    assert_eq!(calibrate(contents), 142);
}

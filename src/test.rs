#[cfg(test)]
use crate::day_one;
use std::fs;

#[test]
/// Find the two entries that sum to 2020 and then multiply those two numbers together.
fn day_one_test() {
    let input = fs::read_to_string("./test_data/input/1.txt").unwrap();
    let result = day_one(input);

    let output = fs::read_to_string("./test_data/output/1.txt").unwrap();
    assert_eq!(output.parse::<i32>().unwrap(), result);
}

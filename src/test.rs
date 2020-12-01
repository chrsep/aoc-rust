#[cfg(test)]
use crate::day_one;
use std::fs;

#[test]
fn day_one_test() {
    match fs::read_to_string("./test_inputs/1.txt") {
        Ok(input) => day_one(input),
        Err(err) => {
            println!("{}",err);
            assert!(false)
        }
    }
}

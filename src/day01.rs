#![allow(clippy::integer_arithmetic)]

#[cfg(test)]
use pretty_assertions::assert_eq;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

/// To find the fuel required for a module,
/// take its mass, divide by three, round down, and subtract 2.
pub fn compute_fuel(mass: i32) -> i32 {
    #![allow(clippy::integer_division)]
    (mass / 3) - 2
}

pub fn answer() -> i32 {
    let filename = "inputs/day01.txt";
    let file = File::open("log.txt")
        .unwrap_or_else(|e| panic!("Something went wrong reading {}: {}", filename, e));
    let buf = BufReader::new(file);
    let contents: Vec<i32> = buf
        .lines()
        .map(|l| {
            l.expect("Could not parse line")
                .parse::<i32>()
                .expect("Could not parse integer")
        })
        .collect();

    contents
        .iter()
        .fold(0, |acc, &mass| acc + compute_fuel(mass))
}

#[test]
fn test_compute_fuel() {
    // For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
    assert_eq!(compute_fuel(12), 2);
    // For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
    assert_eq!(compute_fuel(14), 2);
    // For a mass of 1969, the fuel required is 654.
    assert_eq!(compute_fuel(1969), 654);
    // For a mass of 100756, the fuel required is 33583.
    assert_eq!(compute_fuel(100_756), 33_583);
}

//! <https://adventofcode.com/2019/day/1>

#![allow(clippy::integer_arithmetic)]

#[cfg(test)]
use pretty_assertions::assert_eq;

use std::{
    cmp,
    fs::File,
    io::{prelude::*, BufReader},
};

/// Public facing entry point that is called in main.rs to get the answers.
pub fn answers() -> (i32, i32) {
    let input = parse();
    (compute_total_fuel(&input), compute_all_bonus_fuel(&input))
}

/// Parses the day01 input
fn parse() -> Vec<i32> {
    let filename = "inputs/day01.txt";
    let file = File::open(filename)
        .unwrap_or_else(|e| panic!("Something went wrong reading {}: {}", filename, e));
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| {
            l.expect("Could not parse line")
                .parse::<i32>()
                .expect("Could not parse integer")
        })
        .collect()
}

/// Computes fuel for all masses
fn compute_total_fuel(masses: &[i32]) -> i32 {
    masses.iter().map(|&mass| compute_fuel(mass)).sum()
}

/// To find the fuel required for a module,
/// take its mass, divide by three, round down, and subtract 2.
/// Negative mass requires no fuel.
fn compute_fuel(mass: i32) -> i32 {
    #![allow(clippy::integer_division)]
    cmp::max(0, (mass / 3) - 2)
}

#[test]
fn test_compute_fuel() {
    assert_eq!(compute_fuel(0), 0);
    // For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
    assert_eq!(compute_fuel(12), 2);
    // For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
    assert_eq!(compute_fuel(14), 2);
    // For a mass of 1969, the fuel required is 654.
    assert_eq!(compute_fuel(1969), 654);
    // For a mass of 100756, the fuel required is 33583.
    assert_eq!(compute_fuel(100_756), 33_583);
    // Part 1 answer
    assert_eq!(compute_total_fuel(&parse()), 3_390_596);
}

/// Calculates the fuel requirements for each module separately, then adds them all up
fn compute_all_bonus_fuel(masses: &[i32]) -> i32 {
    masses
        .iter()
        .map(|&mass| compute_fuel_fixed_point(mass))
        .sum()
}

/// Fuel itself requires fuel just like a module - take its mass,
/// divide by three, round down, and subtract 2.
/// However, that fuel also requires fuel, and that fuel requires fuel,
/// and so on.
/// Any mass that would require negative fuel should instead be treated as if it
/// requires zero fuel; the remaining mass, if any, is instead handled by
/// wishing really hard, which has no mass
/// and is outside the scope of this calculation.
fn compute_fuel_fixed_point(mass: i32) -> i32 {
    let mut fuel_so_far = compute_fuel(mass);
    let mut fuel_for_fuel = compute_fuel(fuel_so_far);
    while fuel_for_fuel > 0 {
        fuel_so_far += fuel_for_fuel;
        fuel_for_fuel = compute_fuel(fuel_for_fuel);
    }
    fuel_so_far
}

#[test]
fn test_fuel_fixed_pnt() {
    // A module of mass 14 requires 2 fuel.
    assert_eq!(compute_fuel_fixed_point(14), 2);
    // A module of mass 1969 requires is 654 + 216 + 70 + 21 + 5 = 966.
    assert_eq!(compute_fuel_fixed_point(1969), 966);
    // The fuel required by a module of mass 100756 is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
    assert_eq!(compute_fuel_fixed_point(100_756), 50_346);
    // Part 2 answer
    assert_eq!(compute_all_bonus_fuel(&parse()), 5_083_024);
}

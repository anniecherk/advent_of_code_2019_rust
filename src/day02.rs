//! <https://adventofcode.com/2019/day/2>

#![allow(clippy::panic)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::integer_arithmetic)]

#[cfg(test)]
use pretty_assertions::assert_eq;

/// Writes data[pc+1] + data[pc+2] to data[pc+3], returns new program counter
/// pc is the address of the add opcode being executed
fn add(pc: usize, memory: &mut Vec<usize>) -> usize {
    // Opcode 1 adds together numbers read from two positions and stores the
    // result in a third position. The three integers immediately after the
    // opcode tell you these three positions - the first two indicate the
    // positions from which you should read the input values, and the third
    // indicates the position at which the output should be stored.
    let arg1_addr = memory[pc + 1];
    let arg2_addr = memory[pc + 2];
    let result_addr = memory[pc + 3];
    memory[result_addr] = memory[arg1_addr] + memory[arg2_addr];
    pc + 4
}

/// Writes data[pc+1] * data[pc+2] to data[pc+3], returns new program counter
/// pc is the address of the mult opcode being executed
fn mult(pc: usize, memory: &mut Vec<usize>) -> usize {
    // Opcode 2 works exactly like opcode 1, except it multiplies the two
    // inputs instead of adding them. Again, the three integers after the
    // opcode indicate where the inputs and outputs are, not their values.
    let arg1_addr = memory[pc + 1];
    let arg2_addr = memory[pc + 2];
    let result_addr = memory[pc + 3];
    memory[result_addr] = memory[arg1_addr] * memory[arg2_addr];
    pc + 4
}

/// Public facing entry point that is called in main.rs to get the answers.
pub fn answers() -> (usize, usize) {
    let day_02_contents = include_str!("../inputs/day02.txt");
    let mut memory = parse(day_02_contents);
    memory[1] = 12;
    memory[2] = 2;
    let answer1 = run_computer(&memory);
    let answer2 = part2(&memory);
    (answer1[0], answer2)
}

/// Intcode interpretter
/// Copies the contents of `input_memory` and then evaluates the program
/// starting the program counter at 0.
fn run_computer(input_memory: &[usize]) -> Vec<usize> {
    let mut pc = 0;
    let mut memory = input_memory.to_owned();
    loop {
        match memory[pc] {
            1 => pc = add(pc, &mut memory),
            2 => pc = mult(pc, &mut memory),
            99 => break,
            _ => panic!(format!("🔥🔥Unrecognized opcode {}🔥🔥", memory[pc])),
        }
    }
    memory
}

/// Determines what pair of inputs produces the output 19690720
fn part2(input_memory: &[usize]) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = input_memory.to_owned();
            memory[1] = noun;
            memory[2] = verb;
            let result = run_computer(&memory);
            let answer = result[0];
            // This magic number is the same for all puzzles.
            if 19_690_720 == answer {
                return 100 * noun + verb;
            }
        }
    }
    panic!("Did not find answer!");
}

/// Parses the day02 input
/// Day02 input is a comma seperated list of integers.
fn parse(data: &str) -> Vec<usize> {
    data.split(',')
        .map(|l| l.parse::<usize>().expect("Could not parse integer"))
        .collect()
}

#[test]
fn test_run_computer() {
    // 1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
    let input1 = vec![1, 0, 0, 0, 99];
    let answer1 = vec![2, 0, 0, 0, 99];
    let result1 = run_computer(&input1);
    assert_eq!(answer1, result1);
    // 2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6).
    let input2 = vec![2, 3, 0, 3, 99];
    let answer2 = vec![2, 3, 0, 6, 99];
    let result2 = run_computer(&input2);
    assert_eq!(answer2, result2);
    // 2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801).
    let input3 = vec![2, 4, 4, 5, 99, 0];
    let answer3 = vec![2, 4, 4, 5, 99, 9801];
    let result3 = run_computer(&input3);
    assert_eq!(answer3, result3);
    // 1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.
    let input4 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    let answer4 = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
    let result4 = run_computer(&input4);
    assert_eq!(answer4, result4);

    let mut lel_input = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 13, 19, 1, 9, 19, 23, 1, 6, 23, 27,
        2, 27, 9, 31, 2, 6, 31, 35, 1, 5, 35, 39, 1, 10, 39, 43, 1, 43, 13, 47, 1, 47, 9, 51, 1,
        51, 9, 55, 1, 55, 9, 59, 2, 9, 59, 63, 2, 9, 63, 67, 1, 5, 67, 71, 2, 13, 71, 75, 1, 6, 75,
        79, 1, 10, 79, 83, 2, 6, 83, 87, 1, 87, 5, 91, 1, 91, 9, 95, 1, 95, 10, 99, 2, 9, 99, 103,
        1, 5, 103, 107, 1, 5, 107, 111, 2, 111, 10, 115, 1, 6, 115, 119, 2, 10, 119, 123, 1, 6,
        123, 127, 1, 127, 5, 131, 2, 9, 131, 135, 1, 5, 135, 139, 1, 139, 10, 143, 1, 143, 2, 147,
        1, 147, 5, 0, 99, 2, 0, 14, 0,
    ];
    lel_input[1] = 12;
    lel_input[2] = 2;
    let lel_result = run_computer(&lel_input);
    assert_eq!(5_305_097, lel_result[0]);

    let mut aac_input = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 10, 19, 23, 1, 23, 9, 27,
        1, 5, 27, 31, 2, 31, 13, 35, 1, 35, 5, 39, 1, 39, 5, 43, 2, 13, 43, 47, 2, 47, 10, 51, 1,
        51, 6, 55, 2, 55, 9, 59, 1, 59, 5, 63, 1, 63, 13, 67, 2, 67, 6, 71, 1, 71, 5, 75, 1, 75, 5,
        79, 1, 79, 9, 83, 1, 10, 83, 87, 1, 87, 10, 91, 1, 91, 9, 95, 1, 10, 95, 99, 1, 10, 99,
        103, 2, 103, 10, 107, 1, 107, 9, 111, 2, 6, 111, 115, 1, 5, 115, 119, 2, 119, 13, 123, 1,
        6, 123, 127, 2, 9, 127, 131, 1, 131, 5, 135, 1, 135, 13, 139, 1, 139, 10, 143, 1, 2, 143,
        147, 1, 147, 10, 0, 99, 2, 0, 14, 0,
    ];
    aac_input[1] = 12;
    aac_input[2] = 2;
    let aac_result = run_computer(&aac_input);
    assert_eq!(4_930_687, aac_result[0]);
}

#![deny(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::else_if_without_else,
    clippy::print_stdout,
    clippy::missing_docs_in_private_items,
    clippy::non_ascii_literal,
    clippy::implicit_return,
    clippy::result_expect_used
)]

mod day02;

fn main() {
    println!("Lookout world, running day01!");
    let (part1, part2) = day02::answers();
    println!(
        "And the answer is 🥁🥁🥁 \n Part 1:{}, and Part 2:{}",
        part1, part2
    );
    println!("Done!");
}

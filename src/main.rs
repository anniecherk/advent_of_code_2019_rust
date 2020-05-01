#![deny(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::else_if_without_else,
    clippy::print_stdout,
    clippy::missing_docs_in_private_items,
    clippy::non_ascii_literal,
    clippy::implicit_return,
    clippy::result_expect_used
)]

mod day01;

fn main() {
    println!("Lookout world, running day01!");
    println!("And the answer is 🥁🥁🥁 {}", day01::answer());
    println!("Done!");
}

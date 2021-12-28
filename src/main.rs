#[allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod utils;

fn run() -> Result<(), utils::Error> {
    day7::problem1()?;
    day7::problem2()?;

    Ok(())
}

fn main() {
    let result = run();

    if let Err(e) = result {
        println!("An error occurred: {}", e);
    }
}

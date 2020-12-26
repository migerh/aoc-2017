#[allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod utils;

fn run() -> Result<(), utils::Error> {
    day5::problem1()?;
    day5::problem2()?;

    Ok(())
}

fn main() {
    let result = run();

    if let Err(e) = result {
        println!("An error occurred: {}", e);
    }
}

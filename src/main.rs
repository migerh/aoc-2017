#[allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod utils;

fn run() -> Result<(), utils::Error> {
    day3::problem1()?;
    day3::problem2()?;

    Ok(())
}

fn main() {
    let result = run();

    if let Err(e) = result {
        println!("An error occurred: {}", e);
    }
}

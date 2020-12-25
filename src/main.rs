#[allow(dead_code)]

mod day1;
mod day2;
mod utils;

fn run() -> Result<(), utils::Error> {
    day2::problem1()?;
    day2::problem2()?;

    Ok(())
}

fn main() {
    let result = run();

    if let Err(e) = result {
        println!("An error occurred: {}", e);
    }
}

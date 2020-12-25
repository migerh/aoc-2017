mod day1;
mod utils;

fn run() -> Result<(), utils::Error> {
    day1::problem1()
}

fn main() {
    let result = run();

    if let Err(e) = result {
        println!("An error occurred: {}", e);
    }
}

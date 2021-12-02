use std::error::Error;

mod day1;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    day1::run()?;
    Ok(())
}

use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");
    let mut values = vec![];
    for l in input.lines() {
        values.push(u32::from_str(l)?)
    }

    let increases = values.iter().tuple_windows().filter(|(f, l)| l > f).count();
    println!("Number of increases: {}", increases);

    let three_window = values
        .iter()
        .tuple_windows()
        .map(|(n1, n2, n3)| n1 + n2 + n3)
        .tuple_windows()
        .filter(|(f, l)| l > f)
        .count();
    println!("Number of increases comparing 3 windows: {}", three_window);
    Ok(())
}

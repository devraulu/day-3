use anyhow::Result;
use day_3::part1;
use std::{fs::File, io::BufReader};

fn main() -> Result<()> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let result = part1::process(reader)?;
    println!("amounts to: {}", result);

    Ok(())
}

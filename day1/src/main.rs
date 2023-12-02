use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: String) -> Result<String> {
    Ok(line
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
        .chars()
        .filter(|&c| c.is_digit(10))
        .collect::<String>())
}

fn main() -> Result<(), anyhow::Error> {
    let mut total = 0;
    for line in get_lines("input.txt")? {
        let digits = parse_line(line?)?;
        let amt = format!(
            "{}{}",
            digits.chars().next().unwrap(),
            digits.chars().rev().next().unwrap()
        )
        .parse::<i64>()?;
        total += amt;
    }
    println!("Total: {total}");
    Ok(())
}

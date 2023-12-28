use std::env;
use std::error::Error;
use std::fs;

fn compute_result(l: Vec<i64>) -> i64 {
    let last_value = l[l.len() - 1];

    // compute the differences between each value
    let mut diffs = Vec::new();
    // be efficient to see if we have all zeros
    let mut zeros = true;
    for i in 1..l.len() {
        let d = l[i] - l[i - 1];
        if d != 0 {
            zeros = false;
        }
        diffs.push(d);
    }

    // if we have all zeros, it's time to move up (just return last value because it'll just add to
    // 0)
    if zeros {
        last_value
    } else {
        last_value + compute_result(diffs)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let choice = match args.get(1) {
        None => panic!("Bad arguments"),
        Some(c) => c.as_str(),
    };

    let filename = match choice {
        "t" | "T" => "test.txt",
        "i" | "I" => "input.txt",
        _ => panic!("Invalid choice: t/T, i/T"),
    };

    let file = fs::read_to_string(filename)?;

    let mut result = 0;

    for line in file.lines() {
        // transform the input line to a Vec<i64>
        let l = line
            .split_whitespace()
            .map(|s| s.trim().parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()?;

        // compute what the result will be for this line
        result += compute_result(l);
    }

    println!("P1: {result}");

    Ok(())
}

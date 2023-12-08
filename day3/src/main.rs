use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Copy, Clone)]
struct Coordinate {
    row: i64,
    col: i64,
}

#[derive(Debug, Copy, Clone)]
struct Num {
    value: i32,
    start: Coordinate,
    end: Coordinate,
}

impl Num {
    fn new(num: String, row_num: usize, col_num: usize) -> Result<Self, Box<dyn Error>> {
        let r = row_num as i64;
        let c = col_num as i64;
        Ok(Num {
            value: num.parse()?,
            start: Coordinate {
                row: r,
                col: c - num.len() as i64,
            },
            end: Coordinate { row: r, col: c - 1 },
        })
    }
}

fn part_number(sym: Option<&char>) -> Option<(&char, bool)> {
    match sym {
        None => None,
        Some(s) => Some((s, !s.is_digit(10) && *s != '.')),
    }
}

/// Checks a given row, col in char_map
/// Returns true if symbol is there, false otherwise
fn check_symbol_at(row: i64, col: i64, char_map: &Vec<Vec<char>>) -> Option<(&char, bool)> {
    let r: usize = match row.try_into() {
        Err(_) => {
            return None;
        }
        Ok(rw) => rw,
    };
    let c: usize = match col.try_into() {
        Err(_) => {
            return None;
        }
        Ok(cl) => cl,
    };

    match char_map.get(r) {
        None => None,
        Some(row) => part_number(row.get(c)),
    }
}

fn gear_insert(gears: &mut HashMap<(i64, i64), [i32; 2]>, row: i64, col: i64, val: i32) {
    match gears.get_mut(&(row, col)) {
        Some(v) => {
            if v[1] != 0 {
                // this is a bad case. only two exact.. actually remove this from the map
                gears.remove(&(row, col));
                return;
            }
            // v[0] should have another value already
            v[1] = val;
        }
        None => {
            // create v[0] and insert
            let a = [val, 0];
            let _ = gears.insert((row, col), a);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    //let input = include_str!("../test1.txt");
    let input = include_str!("../input.txt");
    let mut numbers = Vec::new();

    for (row_num, row) in input.lines().enumerate() {
        let mut num = String::new();
        for (col_num, c) in row.chars().enumerate() {
            match c {
                '0'..='9' => {
                    num.push(c);
                }
                _ => {
                    if !num.is_empty() {
                        numbers.push(Num::new(num, row_num, col_num)?);
                        num = String::new();
                    }
                }
            }
        }
        if !num.is_empty() {
            numbers.push(Num::new(num, row_num, row.len())?);
        }
    }

    let mut char_map = Vec::new();
    for line in input.lines() {
        char_map.push(line.chars().collect::<Vec<_>>());
    }

    // DS for stars to compute gear ratios
    let mut gears: HashMap<(i64, i64), [i32; 2]> = HashMap::new();

    let mut part_sum = 0;

    for n in &numbers {
        // always continue to next number if we find a match
        // left of start
        let mut add_val = false;
        match check_symbol_at(n.start.row, n.start.col - 1, &char_map) {
            Some((sym, b)) if b => {
                add_val = true;
                if *sym == '*' {
                    gear_insert(&mut gears, n.start.row, n.start.col - 1, n.value);
                }
            }
            _ => {}
        }

        // right of end
        match check_symbol_at(n.end.row, n.end.col + 1, &char_map) {
            Some((sym, b)) if b => {
                add_val = true;
                if *sym == '*' {
                    gear_insert(&mut gears, n.end.row, n.end.col + 1, n.value);
                }
            }
            _ => {}
        }

        // diagonal left from start
        match check_symbol_at(n.start.row - 1, n.start.col - 1, &char_map) {
            Some((sym, b)) if b => {
                add_val = true;
                if *sym == '*' {
                    gear_insert(&mut gears, n.start.row - 1, n.start.col - 1, n.value);
                }
            }
            _ => {}
        }

        // diagonal right from end
        match check_symbol_at(n.end.row - 1, n.end.col + 1, &char_map) {
            Some((sym, b)) if b => {
                add_val = true;
                if *sym == '*' {
                    gear_insert(&mut gears, n.end.row - 1, n.end.col + 1, n.value);
                }
            }
            _ => {}
        }
        // loop from right above start of number to the end
        for c in n.start.col..=n.end.col {
            match check_symbol_at(n.start.row - 1, c, &char_map) {
                Some((sym, b)) if b => {
                    add_val = true;
                    if *sym == '*' {
                        gear_insert(&mut gears, n.start.row - 1, c, n.value);
                    }
                }
                _ => {}
            }
        }

        // diagonal left down from start
        match check_symbol_at(n.start.row + 1, n.start.col - 1, &char_map) {
            Some((sym, b)) if b => {
                part_sum += n.value;
                if *sym == '*' {
                    gear_insert(&mut gears, n.start.row + 1, n.start.col - 1, n.value);
                }
                continue;
            }
            _ => {}
        }

        // diag right down from end
        match check_symbol_at(n.end.row + 1, n.end.col + 1, &char_map) {
            Some((sym, b)) if b => {
                add_val = true;
                if *sym == '*' {
                    gear_insert(&mut gears, n.end.row + 1, n.end.col + 1, n.value);
                }
            }
            _ => {}
        }

        // below number
        for c in n.start.col..=n.end.col {
            match check_symbol_at(n.start.row + 1, c, &char_map) {
                Some((sym, b)) if b => {
                    add_val = true;
                    if *sym == '*' {
                        gear_insert(&mut gears, n.start.row + 1, c, n.value);
                    }
                }
                _ => {}
            }
        }

        if add_val {
            // only add once
            part_sum += n.value;
        }
    }

    println!("Sum: {part_sum}");

    // now we should loop through the values of gears and multiply them then sum them (if there is
    // only one, there should be a 0, so no impact on the sum
    let gear_sum: i32 = gears.values().map(|a| a[0] * a[1]).sum::<_>();
    println!("Gear sum: {gear_sum}");

    Ok(())
}

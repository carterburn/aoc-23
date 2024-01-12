use std::collections::HashSet;
use std::fmt;
use std::{env, fs};

#[derive(Debug)]
struct Graph {
    map: Vec<Vec<char>>,
    galaxies: Vec<(usize, usize)>,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for row in &self.map {
            for c in row.iter() {
                write!(f, "{c}")?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

impl Graph {
    fn parse(input: &str) -> Self {
        // create the double rows (if needed) first
        let mut map: Vec<Vec<char>> = Vec::new();
        for line in input.lines() {
            if !line.contains('#') {
                map.push(line.chars().collect());
            }
            map.push(line.chars().collect());
        }

        // loop through the columns marking which indices need another column added after them
        let mut col_adds = HashSet::<usize>::new();
        'cols: for col in 0..map[0].len() {
            for row in 0..map.len() {
                if map[row][col] == '#' {
                    continue 'cols;
                }
            }
            // if we finished the row loop, we have to add this column as one that needs to be
            // doubled
            col_adds.insert(col);
        }

        let mut expanded_map: Vec<Vec<char>> = Vec::new();
        // loop through the rows (already expanded) and make a new map based on repeated columns
        for row in map {
            let mut r = Vec::new();
            for (col, chr) in row.iter().enumerate() {
                if col_adds.get(&col).is_some() {
                    // add an additional column
                    r.push(*chr);
                }
                r.push(*chr);
            }
            expanded_map.push(r);
        }

        // now make the vec with the coordinates (x = col, y = row)
        let mut galaxies = Vec::new();
        for row in 0..expanded_map.len() {
            for col in 0..expanded_map[0].len() {
                if expanded_map[row][col] == '#' {
                    galaxies.push((col, row));
                }
            }
        }

        Self {
            map: expanded_map,
            galaxies,
        }
    }

    // compute the shortest distance between each galaxy
    pub fn all_pairs_shortest_distance(&self) -> u64 {
        let mut sum = 0;
        for (galaxy, (x1, y1)) in self.galaxies.iter().enumerate() {
            for (pair, (x2, y2)) in self.galaxies[galaxy + 1..].iter().enumerate() {
                let dist =
                    ((*x1 as i64 - *x2 as i64).abs() + (*y1 as i64 - *y2 as i64).abs()) as u64;
                sum += dist;
                println!(
                    "Shortest distance between {} and {} => {}",
                    galaxy + 1,
                    galaxy + 1 + pair + 1,
                    dist,
                );
            }
        }
        sum
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let choice = match args.get(1) {
        None => panic!("Bad arguments"),
        Some(c) => c.as_str(),
    };

    let filename = match choice {
        "1" => "test1.txt",
        "i" | "I" => "input.txt",
        _ => panic!("Invalid choice: 1, i/I"),
    };

    let file = fs::read_to_string(filename)?;

    let g = Graph::parse(&file);
    println!("{g}");

    let p1 = g.all_pairs_shortest_distance();
    println!("{p1}");

    Ok(())
}

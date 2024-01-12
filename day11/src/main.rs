use std::collections::HashSet;
use std::{env, fs};

const EXPANSION_AMT: usize = 1000000 - 1;

#[derive(Debug)]
struct Graph {
    row_adds: HashSet<usize>,
    col_adds: HashSet<usize>,
    galaxies: Vec<(usize, usize)>,
}

impl Graph {
    fn parse(input: &str) -> Self {
        let mut row_adds = HashSet::<usize>::new();
        for (y, line) in input.lines().enumerate() {
            if !line.contains('#') {
                row_adds.insert(y);
            }
        }

        // create the initial map first
        let mut map: Vec<Vec<char>> = Vec::new();
        for line in input.lines() {
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

        let mut galaxies = Vec::new();
        for (row, r) in map.iter().enumerate() {
            for (col, v) in r.iter().enumerate() {
                if *v == '#' {
                    galaxies.push((col, row));
                }
            }
        }

        Self {
            row_adds,
            col_adds,
            galaxies,
        }
    }

    /// Finds the number of columns from x1 to x2 with expansions enabled
    fn col_dist(&self, x1: usize, x2: usize) -> usize {
        let mut sum = 0;
        let rng = if x1 > x2 { x2..x1 } else { x1..x2 };
        for i in rng {
            // every column gets at least 1 addition
            sum += 1;
            // if it's in the col add, add the amount we need to
            if self.col_adds.contains(&i) {
                sum += EXPANSION_AMT;
            }
        }
        sum
    }

    /// Finds the number of rows from y1 to y2 with expansions enabled
    fn row_dist(&self, y1: usize, y2: usize) -> usize {
        let mut sum = 0;
        let rng = if y1 > y2 { y2..y1 } else { y1..y2 };
        for i in rng {
            sum += 1;
            if self.row_adds.contains(&i) {
                sum += EXPANSION_AMT;
            }
        }
        sum
    }

    // compute the shortest distance between each galaxy
    pub fn all_pairs_shortest_distance(&self) -> u64 {
        let mut sum = 0;
        for (galaxy, (x1, y1)) in self.galaxies.iter().enumerate() {
            for (pair, (x2, y2)) in self.galaxies[galaxy + 1..].iter().enumerate() {
                let dist = self.col_dist(*x1, *x2) + self.row_dist(*y1, *y2);
                sum += dist;
                println!(
                    "Shortest distance between {} and {} => {}",
                    galaxy + 1,
                    galaxy + 1 + pair + 1,
                    dist,
                );
            }
        }
        sum as u64
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
    println!("{g:?}");

    let p1 = g.all_pairs_shortest_distance();
    println!("{p1}");

    Ok(())
}

use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug)]
struct Node {
    sym: char,
    neighbors: ((i32, i32), (i32, i32)),
}

#[derive(Debug)]
struct Graph {
    start_coord: (usize, usize),
    rows: usize,
    cols: usize,
    map: Vec<Vec<Node>>,
    main_loop: Option<HashSet<(i32, i32)>>,
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in self.map.iter() {
            for val in row.iter() {
                write!(f, "{}", val.sym)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

impl Graph {
    pub fn parse(input: &str) -> Self {
        let (mut srow, mut scol) = (0, 0);
        let mut g = Vec::new();
        for (row, row_input) in input.lines().enumerate() {
            let mut r = Vec::with_capacity(row_input.len() - 1);
            for (col, v) in row_input.chars().enumerate() {
                if v == 'S' {
                    srow = row;
                    scol = col;
                }
                let neighs = Graph::compute_neighbors(v);
                r.push(Node {
                    sym: v,
                    neighbors: (
                        (row as i32 + neighs.0 .0, col as i32 + neighs.0 .1),
                        (row as i32 + neighs.1 .0, col as i32 + neighs.1 .1),
                    ),
                });
            }
            g.push(r);
        }
        let rows = g.len();
        let cols = if rows > 0 { g[0].len() } else { 0 };

        Self {
            start_coord: (srow, scol),
            rows,
            cols,
            map: g,
            main_loop: None,
        }
    }

    pub fn compute_neighbors(sym: char) -> ((i32, i32), (i32, i32)) {
        match sym {
            '|' => ((1, 0), (-1, 0)),
            '-' => ((0, 1), (0, -1)),
            'L' => ((-1, 0), (0, 1)),
            'J' => ((-1, 0), (0, -1)),
            '7' => ((1, 0), (0, -1)),
            'F' => ((1, 0), (0, 1)),
            _ => ((0, 0), (0, 0)),
        }
    }

    pub fn find_main_loop(&mut self) {
        // find start's orientation
        let mut neighs = Vec::new();
        let mut mv = Vec::new();
        let s = (self.start_coord.0 as i32, self.start_coord.1 as i32);
        // checkout each neighbor
        for i in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let cr = s.0 + i.0;
            let cc = s.1 + i.1;
            if cr < 0 || cr >= self.rows as i32 || cc < 0 || cc >= self.cols as i32 {
                // not a possible neighbor of start
                continue;
            }
            // get (cr, cc)'s neighbors
            let candidate = &self.map[cr as usize][cc as usize];

            if candidate.neighbors.0 == s {
                neighs.push((cr, cc));
                mv.push(i);
                continue;
            }
            if candidate.neighbors.1 == s {
                neighs.push((cr, cc));
                mv.push(i);
                continue;
            }
        }

        assert_eq!(neighs.len(), 2);
        // set start's neighbors
        self.map[s.0 as usize][s.1 as usize].neighbors = (neighs[0], neighs[1]);
        self.map[s.0 as usize][s.1 as usize].sym = 'S';

        // walk from start until we get back to it
        let mut path = HashSet::new();
        path.insert(s);
        let mut last = s;
        let mut cur = neighs[0];

        loop {
            // mark cur as visited
            if !path.insert(cur) {
                // already visited, we're done
                break;
            }

            // pick cur's neighbor that we aren't coming from
            let cur_neigh = self.map[cur.0 as usize][cur.1 as usize].neighbors;
            let nxt = if cur_neigh.0 == last {
                cur_neigh.1
            } else {
                cur_neigh.0
            };

            last = cur;
            cur = nxt;
        }

        self.main_loop = Some(path);
    }

    pub fn ray_trace(&self) -> usize {
        // ray trace diagonally so we can detect when we have collinear items (L and 7; right/down
        // or left/up)
        let mut inside = 0;

        let main_loop = match self.main_loop.as_ref() {
            None => {
                return 0;
            }
            Some(ml) => ml,
        };

        for row in 0..self.rows {
            for col in 0..self.cols {
                if main_loop.get(&(row as i32, col as i32)).is_some() {
                    continue;
                }

                let mut crosses = 0;
                let (mut r2, mut c2) = (row, col);
                while r2 < self.rows && c2 < self.cols {
                    let sym = self.map[r2][c2].sym;
                    if main_loop.get(&(r2 as i32, c2 as i32)).is_some() && sym != 'L' && sym != '7'
                    {
                        crosses += 1;
                    }
                    r2 += 1;
                    c2 += 1;
                }

                if crosses % 2 == 1 {
                    inside += 1;
                }
            }
        }
        inside
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
        "2" => "test2.txt",
        "3" => "test3.txt",
        "4" => "test4.txt",
        "5" => "test5.txt",
        "i" | "I" => "input.txt",
        _ => panic!("Invalid choice: 1, 2, 3, 4, 5, i/I"),
    };

    let file = fs::read_to_string(filename)?;

    let mut graph = Graph::parse(&file);

    graph.find_main_loop();

    println!("{:?}", graph.ray_trace());

    Ok(())
}

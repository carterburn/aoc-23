use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Coordinate {
    row: i32,
    col: i32,
}

impl Coordinate {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

#[derive(Clone, Hash, Debug)]
struct Node {
    start: bool,
    ground: bool,
    coord: Coordinate,
    neighbors: (Coordinate, Coordinate),
}

#[derive(Clone, Debug)]
struct Graph {
    verts: Vec<Node>,
}

impl Graph {
    pub fn parse(input: &str) -> anyhow::Result<Graph> {
        // no nom this time because getting coordinates is a bit more difficult (when they're not
        // part of the input)
        // this is technically too big but it'll work
        let mut verts = Vec::with_capacity(input.len());
        for (row, row_input) in input.lines().enumerate() {
            for (col, v) in row_input.chars().enumerate() {
                let row = row as i32;
                let col = col as i32;
                let mut start = false;
                let mut ground = false;
                let neighbors = match v {
                    // north and south (+1 row, -1 row)
                    '|' => (Coordinate::new(row + 1, col), Coordinate::new(row - 1, col)),
                    // east and west +1 col, -1 col
                    '-' => (Coordinate::new(row, col + 1), Coordinate::new(row, col - 1)),
                    // north and east
                    'L' => (Coordinate::new(row - 1, col), Coordinate::new(row, col + 1)),
                    // north and west
                    'J' => (Coordinate::new(row - 1, col), Coordinate::new(row, col - 1)),
                    // south and west
                    '7' => (Coordinate::new(row + 1, col), Coordinate::new(row, col - 1)),
                    // south and east
                    'F' => (Coordinate::new(row + 1, col), Coordinate::new(row, col + 1)),
                    // ground (no pipe / no neighbors)
                    '.' => {
                        ground = true;
                        (Coordinate::new(0, 0), Coordinate::new(0, 0))
                    }
                    'S' => {
                        start = true;
                        (Coordinate::new(0, 0), Coordinate::new(0, 0))
                    }
                    _ => {
                        return Err(anyhow::anyhow!("Invalid input character"));
                    }
                };
                verts.push(Node {
                    start,
                    ground,
                    coord: Coordinate::new(row, col),
                    neighbors,
                });
            }
        }
        Ok(Graph { verts })
    }

    pub fn find_start(&self) -> Option<&Node> {
        self.verts.iter().find(|n| n.start == true)
    }

    pub fn start_coord(&self) -> Option<&Coordinate> {
        self.find_start().map(|n| &n.coord)
    }

    pub fn find_node(&self, r: i32, c: i32) -> Option<&Node> {
        self.verts
            .iter()
            .find(|n| n.coord.row == r && n.coord.col == c)
    }

    pub fn get_start_neighbors(&self) -> (&Node, &Node) {
        let start = self.find_start().unwrap();
        let mut neighs = Vec::new();
        // check every possible neighbor (up, down, left, right) and see entrances
        for i in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let candidate = match self.find_node(start.coord.row + i.0, start.coord.col + i.1) {
                None => {
                    continue;
                }
                Some(c) => c,
            };
            // one of the neighbors has to support moving toward the start
            if candidate.neighbors.0 == start.coord {
                neighs.push(candidate);
                continue;
            }
            if candidate.neighbors.1 == start.coord {
                neighs.push(candidate);
                continue;
            }
        }

        // check to ensure we have two (impossible otherwise)
        assert_eq!(neighs.len(), 2);

        // once we have these we _could_ try to figure out what 'S' is but it doesn't really matter
        // just return the two coordinates to start at
        (neighs.get(0).unwrap(), neighs.get(1).unwrap())
    }

    /// Compute the max distance from the start point
    pub fn max_distance_from_start(&self) -> u32 {
        // hashmap will store coordinates until we find a duplicate
        let mut distances: HashMap<&Coordinate, u32> = HashMap::new();
        // find start's neighbors first
        let (mut n1, mut n2) = self.get_start_neighbors();
        // keep track of where we came from
        let (mut n1_last, mut n2_last) = (self.find_start().unwrap(), self.find_start().unwrap());
        let (mut n1_dist, mut n2_dist) = (1, 1);

        // setup the start with distance 0
        distances.insert(&self.find_start().unwrap().coord, 0);

        // setup the neighbors with distance 1
        distances.insert(&n1.coord, n1_dist);
        distances.insert(&n2.coord, n2_dist);

        // kick off a walk from each neighbor
        loop {
            // find the neighbor that isn't the last step
            let n1_next = if n1.neighbors.0 == n1_last.coord {
                // neighbors.1 is the next step
                let c = n1.neighbors.1;
                self.find_node(c.row, c.col)
            } else {
                // neighbors.0 is the next step
                let c = n1.neighbors.0;
                self.find_node(c.row, c.col)
            }
            .unwrap();

            let n2_next = if n2.neighbors.0 == n2_last.coord {
                // neighbors.1 is the next step
                let c = n2.neighbors.1;
                self.find_node(c.row, c.col)
            } else {
                //neighors.0 is the next step
                let c = n2.neighbors.0;
                self.find_node(c.row, c.col)
            }
            .unwrap();

            // found the next step for each walk, add the distance (if it already isn't)
            n1_dist += 1;
            n2_dist += 1;

            // attempt to add them, if we find it in the map, we've cycled so we'll return early
            match distances.insert(&n1_next.coord, n1_dist) {
                None => {}
                Some(_d) => {
                    return n1_dist;
                }
            }
            match distances.insert(&n2_next.coord, n2_dist) {
                None => {}
                Some(_d) => {
                    return n2_dist;
                }
            }

            // update the last step
            n1_last = n1;
            n1 = n1_next;
            n2_last = n2;
            n2 = n2_next;
        }
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
        "i" | "I" => "input.txt",
        _ => panic!("Invalid choice: 1, 2, i/I"),
    };

    let file = fs::read_to_string(filename)?;

    let graph = match Graph::parse(&file) {
        Ok(g) => g,
        Err(e) => anyhow::bail!("Unable to parse graph: {e:?}"),
    };

    println!("{graph:?}");

    // find the start
    let s = graph.start_coord();
    println!("Start: {s:?}");

    let max_dist = graph.max_distance_from_start();
    println!("Part 1: {max_dist}");

    Ok(())
}

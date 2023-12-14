use std::collections::VecDeque;
use std::error::Error;

/// Custom error type for Almananc errors
#[derive(Debug, Clone)]
#[allow(dead_code)]
enum AlmanacError {
    InvalidInput,
    InvalidRangeString,
}

impl Error for AlmanacError {}

impl std::fmt::Display for AlmanacError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidRangeString => write!(f, "Invalid range string"),
            Self::InvalidInput => write!(f, "Invalid input"),
        }
    }
}

/// Trait that describes an Almanac Map that can convert a source number to the destination
trait AlmanacConverter {
    fn convert(&self, source: u64) -> u64;
}

/// Type that describes the entire almanac
#[allow(dead_code)]
struct Almanac {
    init_seeds: Vec<u64>,
    seed_soil: AlMap,
    soil_fert: AlMap,
    fert_water: AlMap,
    water_light: AlMap,
    light_temp: AlMap,
    temp_humid: AlMap,
    humid_loc: AlMap,
}

impl Almanac {
    /// Parse raw challenge input into an Almanac
    pub fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        // parse the init seeds first
        let lines = input.lines().collect::<Vec<&str>>();
        let init_line = lines.get(0).ok_or(AlmanacError::InvalidInput)?;
        let init_seeds: Vec<u64> = init_line
            .split(' ')
            .filter_map(|x| x.parse::<u64>().ok())
            .collect();

        let mut remaining_lines = &lines[2..];
        let mut maps: VecDeque<AlMap> = VecDeque::new();

        while let Some(ind) = remaining_lines.iter().position(|x| x == &"") {
            let (map_data, rest) = remaining_lines.split_at(ind);

            maps.push_back(AlMap::parse(Vec::from(&map_data[1..]))?);

            remaining_lines = &rest[1..];
        }

        // still one more map to process
        maps.push_back(AlMap::parse(Vec::from(&remaining_lines[1..]))?);

        Ok(Self {
            init_seeds,
            seed_soil: maps.pop_front().ok_or(AlmanacError::InvalidInput)?,
            soil_fert: maps.pop_front().ok_or(AlmanacError::InvalidInput)?,
            fert_water: maps.pop_front().ok_or(AlmanacError::InvalidInput)?,
            water_light: maps.pop_front().ok_or(AlmanacError::InvalidInput)?,
            light_temp: maps.pop_front().ok_or(AlmanacError::InvalidInput)?,
            temp_humid: maps.pop_front().ok_or(AlmanacError::InvalidInput)?,
            humid_loc: maps.pop_front().ok_or(AlmanacError::InvalidInput)?,
        })
    }

    /// Returns the conversions for the initial seeds
    pub fn get_conversions(&self) -> Vec<u64> {
        let mut conversions = Vec::with_capacity(self.init_seeds.len());

        for seed in &self.init_seeds {
            conversions.push(self.get_conversion(seed.clone()));
        }

        conversions
    }

    pub fn get_conversion(&self, seed: u64) -> u64 {
        let soil = self.seed_soil.convert(seed);
        let fert = self.soil_fert.convert(soil);
        let water = self.fert_water.convert(fert);
        let light = self.water_light.convert(water);
        let temp = self.light_temp.convert(light);
        let humid = self.temp_humid.convert(temp);
        self.humid_loc.convert(humid)
    }

    /// Use init seeds as a range instead of the starting points
    pub fn init_seed_range(&self) -> u64 {
        let mut loc_min = u64::MAX;

        for chunk in self.init_seeds.chunks(2) {
            println!("Processing Init Seed Chunk: {chunk:?}");
            if chunk.len() != 2 {
                break;
            }

            // see if this is the new min
            for i in chunk[0]..chunk[0] + chunk[1] {
                let c = self.get_conversion(i);
                if c < loc_min {
                    loc_min = c;
                }
            }
        }
        loc_min
    }
}

/// Generic Almanac Map type that implements the convert trait
struct AlMap {
    /// The ranges for this specific map
    ranges: Vec<Range>,
}

impl AlMap {
    /// Given a list of range strings, parse into an AlMap
    pub fn parse(input: Vec<&str>) -> Result<Self, Box<dyn Error>> {
        let mut ranges = Vec::with_capacity(input.len());

        for r in input {
            ranges.push(Range::parse(r)?);
        }
        Ok(Self { ranges })
    }
}

impl AlmanacConverter for AlMap {
    /// Converts a source to the destination
    fn convert(&self, source: u64) -> u64 {
        // iterate over each range, checking to see if it is in the range
        for rng in &self.ranges {
            match rng.in_range(source) {
                Some(dest) => {
                    return dest;
                }
                None => {
                    continue;
                }
            }
        }

        // if here, never found one in range, so it's source
        return source;
    }
}

/// Type that describes a range with a source, destination, and length
#[derive(Debug, PartialEq, Eq)]
struct Range {
    source_start: u64,
    dest_start: u64,
    range_len: usize,
}

impl Range {
    pub fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let nums = input.split(' ').collect::<Vec<&str>>();

        Ok(Self {
            source_start: nums
                .get(1)
                .ok_or(AlmanacError::InvalidRangeString)?
                .parse()?,
            dest_start: nums
                .get(0)
                .ok_or(AlmanacError::InvalidRangeString)?
                .parse()?,
            range_len: nums
                .get(2)
                .ok_or(AlmanacError::InvalidRangeString)?
                .parse()?,
        })
    }

    /// Returns Some(destination) if source is in range, None if not
    /// in range
    pub fn in_range(&self, source: u64) -> Option<u64> {
        /*
        let source_range = self.source_start..self.source_start + self.range_len as u64;
        if source_range.contains(&source) {
            Some(self.dest_start + (source - self.source_start))
        } else {
            None
        }
        */
        if source >= self.source_start && source < self.source_start + self.range_len as u64 {
            Some(self.dest_start + (source - self.source_start))
        } else {
            None
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    //let input = include_str!("../test1.txt");
    let input = include_str!("../input.txt");

    let alm = Almanac::parse(input)?;
    let conversions = alm.get_conversions();

    println!(
        "Part 1: {}",
        conversions.iter().min().ok_or(AlmanacError::InvalidInput)?
    );

    // for part 2, we should convert the init seeds to a larger vec and then re-run
    // get_conversions()
    let p2convert = alm.init_seed_range();

    println!("Part 2: {}", p2convert);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_range() {
        let input = "52 50 48";
        let rng = Range::parse(input).unwrap();
        assert_eq!(rng.source_start, 50);
        assert_eq!(rng.dest_start, 52);
        assert_eq!(rng.range_len, 48);
    }

    #[test]
    fn test_in_range() {
        let input = "52 50 48";
        let rng = Range::parse(input).unwrap();
        assert_eq!(rng.in_range(79), Some(81));
        assert_eq!(rng.in_range(14), None);
        assert_eq!(rng.in_range(55), Some(57));
        assert_eq!(rng.in_range(13), None);
    }

    #[test]
    fn test_parse_almap() {
        let input = "50 98 2\n52 50 48".split('\n').collect::<Vec<&str>>();
        let map = AlMap::parse(input).unwrap();
        assert_eq!(map.ranges.len(), 2);
        assert_eq!(map.ranges.get(0).unwrap().source_start, 98);
    }

    #[test]
    fn test_converter() {
        let input = "50 98 2\n52 50 48".split('\n').collect::<Vec<&str>>();
        let map = AlMap::parse(input).unwrap();

        assert_eq!(map.convert(79), 81);
        assert_eq!(map.convert(14), 14);
        assert_eq!(map.convert(55), 57);
        assert_eq!(map.convert(13), 13);
    }

    #[test]
    fn test_parse_input() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let alm = Almanac::parse(input).unwrap();

        assert_eq!(alm.init_seeds, Vec::from([79, 14, 55, 13]));
        let seed_soil_inp = "50 98 2\n52 50 48".split('\n').collect::<Vec<&str>>();
        assert_eq!(
            alm.seed_soil.ranges.get(0).unwrap(),
            AlMap::parse(seed_soil_inp).unwrap().ranges.get(0).unwrap()
        );
    }
}

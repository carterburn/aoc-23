use std::cmp::{max, min};
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
    pub fn part2(&self) -> Option<(u64, u64)> {
        // start by making a set of ranges to start with
        let mut init_ranges = vec![];
        for chunk in self.init_seeds.chunks(2) {
            init_ranges.push((chunk[0], chunk[1]));
        }

        // for each of the ranges, output the min values from that translation
        init_ranges = Almanac::map_ranges(init_ranges, &self.seed_soil);
        init_ranges = Almanac::map_ranges(init_ranges, &self.soil_fert);
        init_ranges = Almanac::map_ranges(init_ranges, &self.fert_water);
        init_ranges = Almanac::map_ranges(init_ranges, &self.water_light);
        init_ranges = Almanac::map_ranges(init_ranges, &self.light_temp);
        init_ranges = Almanac::map_ranges(init_ranges, &self.temp_humid);
        init_ranges = Almanac::map_ranges(init_ranges, &self.humid_loc);

        init_ranges.iter().min_by_key(|x| x.0).copied()
    }

    pub fn map_ranges(mut init_ranges: Vec<(u64, u64)>, map: &AlMap) -> Vec<(u64, u64)> {
        // vec to store the eventual results for this mapping
        let mut final_ranges: Vec<(u64, u64)> = vec![];

        // process ranges until we run out in init_ranges
        while let Some(src_rng) = init_ranges.pop() {
            // for each of this map's translations, we need to determine if there is a range that
            // overlaps
            // need to keep track if we find an overlap for the given range (if not, then we just
            // add the identity mapping to final_ranges
            let mut overlap_found = false;
            for translation in &map.ranges {
                // overlaps for the src_rng (from init_ranges) and the rng (from the map
                // translation)
                let overlap_start = max(src_rng.0, translation.source_start);
                let overlap_end = min(
                    src_rng.0 + src_rng.1,
                    translation.source_start + translation.range_len as u64,
                );

                // if we have an overlap, then we can put that whole range in final_ranges
                if overlap_start < overlap_end {
                    // range that moves on is starting
                    final_ranges.push((
                        overlap_start - translation.source_start + translation.dest_start,
                        overlap_end - overlap_start,
                    ));
                    // check to see if there are leftovers from the source rang
                    if src_rng.0 < overlap_start {
                        init_ranges.push((src_rng.0, overlap_start - src_rng.0));
                    }
                    if overlap_end < src_rng.0 + src_rng.1 {
                        init_ranges.push((overlap_end, (src_rng.0 + src_rng.1) - overlap_end));
                    }
                    overlap_found = true;
                    break;
                }
            }
            if !overlap_found {
                // add identity mapping
                final_ranges.push(src_rng);
            }
        }
        final_ranges
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
    match alm.part2() {
        None => println!("Error finding location for part 2!"),
        Some(min_loc) => println!("Part 2: {:?}", min_loc.0),
    };

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

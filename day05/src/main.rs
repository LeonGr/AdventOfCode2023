use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, char, newline, space1, u64},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

fn read_input() -> String {
    let input = include_str!("../input");
    input.to_string()
}

type Seeds = Vec<u64>;

#[derive(Debug)]
enum Direction {
    Sub(u64),
    Add(u64),
}

#[derive(Debug)]
struct Displacement {
    range: Range<u64>,
    direction: Direction,
}

#[derive(Debug)]
struct Map {
    mappings: Vec<Displacement>,
}

fn seeds(input: &str) -> IResult<&str, Seeds> {
    let (extra, seeds) = preceded(tag("seeds: "), separated_list0(space1, u64))(input)?;

    Ok((extra, seeds))
}

fn displacement(input: &str) -> IResult<&str, Displacement> {
    let (extra, (destination_start, source_start, range_length)) =
        tuple((u64, preceded(space1, u64), preceded(space1, u64)))(input)?;

    let direction = match source_start.cmp(&destination_start) {
        std::cmp::Ordering::Less => Direction::Add(destination_start - source_start),
        std::cmp::Ordering::Greater => Direction::Sub(source_start - destination_start),
        std::cmp::Ordering::Equal => unreachable!("No ranges of length 0"),
    };

    let range = source_start..(source_start + range_length);

    Ok((extra, Displacement { range, direction }))
}

fn seed_map_name(input: &str) -> IResult<&str, String> {
    let (extra, results) = separated_list0(char('-'), alpha0)(input)?;
    let from = results.first().unwrap();
    let to = results.last().unwrap();

    Ok((extra, format!("{from}-to-{to}")))
}

fn seed_map(input: &str) -> IResult<&str, Map> {
    let (extra, (_, _, _, mappings)) = tuple((
        seed_map_name,
        tag(" map:"),
        newline,
        separated_list0(newline, displacement),
    ))(input)?;

    let map = Map { mappings };

    Ok((extra, map))
}

fn whole_input(input: &str) -> IResult<&str, (Seeds, Vec<Map>)> {
    let (extra, (seeds, _, maps)) =
        tuple((seeds, tag("\n\n"), separated_list1(tag("\n\n"), seed_map)))(input)?;

    Ok((extra, (seeds, maps)))
}

fn parse(input: &str) -> (Seeds, Vec<Map>) {
    match whole_input(input) {
        Ok((_, result)) => result,
        Err(err) => {
            println!("error: {err:#?}");
            panic!();
        }
    }
}

fn part1(seeds: &[u64], maps: &[Map]) -> u64 {
    let mut results = vec![];

    for seed in seeds {
        let mut step = *seed;

        for map in maps {
            for mapping in &map.mappings {
                let range = &mapping.range;

                if range.contains(&step) {
                    match &mapping.direction {
                        Direction::Sub(d) => step -= d,
                        Direction::Add(d) => step += d,
                    }

                    break;
                }
            }
        }

        results.push(step);
    }

    results.into_iter().min().unwrap()
}

type SeedRange = Range<u64>;

fn update_range(range: &SeedRange, direction: &Direction) -> SeedRange {
    let min = range.clone().min().unwrap();
    let max = range.clone().max().unwrap();

    match direction {
        Direction::Sub(d) => (min - d)..(max + 1 - d),
        Direction::Add(d) => (min + d)..(max + 1 + d),
    }
}

fn handle_map(seed_range: &SeedRange, map: &Map) -> Vec<SeedRange> {
    let mut outputs = vec![];

    let seed_range_min = seed_range.clone().min().unwrap();
    let seed_range_max = seed_range.clone().max().unwrap();

    for mapping in &map.mappings {
        let mapping_range = &mapping.range;

        let mapping_range_min = mapping_range.clone().min().unwrap();
        let mapping_range_max = mapping_range.clone().max().unwrap();

        // both
        if mapping_range.contains(&seed_range_min) && mapping_range.contains(&seed_range_max) {
            outputs.push(update_range(seed_range, &mapping.direction));
            break;
        }
        // minimum overlapped
        else if mapping_range.contains(&(seed_range.clone().min().unwrap())) {
            outputs.push(update_range(
                &(seed_range_min..mapping_range_max),
                &mapping.direction,
            ));

            handle_map(&((mapping_range_max + 1)..(seed_range_max + 1)), map)
                .into_iter()
                .for_each(|x| outputs.push(x));

            break;
        }
        // maximum overlapped
        else if mapping_range.contains(&(seed_range.clone().max().unwrap())) {
            let new_overlapping =
                update_range(&(mapping_range_min..seed_range_max), &mapping.direction);
            outputs.push(new_overlapping);

            let not_overlapping = seed_range_min..mapping_range_min;
            let new_not_overlapping = handle_map(&not_overlapping, map);

            new_not_overlapping
                .into_iter()
                .for_each(|x| outputs.push(x));

            break;
        }
    }

    if outputs.is_empty() {
        outputs.push(seed_range.clone());
    }

    outputs
}

fn handle_seed_ranges(seed_ranges: &[SeedRange], maps: &[Map]) -> Vec<SeedRange> {
    if maps.is_empty() {
        return seed_ranges.to_vec();
    }

    let map = &maps[0];

    let new_seed_ranges = seed_ranges
        .iter()
        .flat_map(|seed_range| handle_map(seed_range, map))
        .collect::<Vec<_>>()
        .clone();

    let remaining_maps = &maps[1..];

    handle_seed_ranges(&new_seed_ranges, remaining_maps)
}

fn part2(seeds: &[u64], maps: &[Map]) -> u64 {
    let seed_ranges: Vec<Range<u64>> = seeds
        .windows(2)
        .step_by(2)
        .map(|x| x[0]..(x[0] + x[1]))
        .collect();

    let final_locations = handle_seed_ranges(&seed_ranges, maps);

    final_locations
        .into_iter()
        .map(|range| range.min().unwrap())
        .min()
        .unwrap()
}

fn main() {
    let lines = read_input();
    let (seeds, maps) = parse(&lines);

    println!("part1: {}", part1(&seeds, &maps));
    println!("part2: {}", part2(&seeds, &maps));
}

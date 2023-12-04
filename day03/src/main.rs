use std::collections::HashMap;

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

enum Place {
    Digit(u8),
    Symbol(char),
}

type Coord = (i32, i32);

struct Schematic {
    width: i32,
    height: i32,
    symbols: HashMap<Coord, Place>,
}

impl Schematic {
    fn get_place(&self, coord: Coord) -> Option<&Place> {
        self.symbols.get(&coord)
    }

    fn has_adjacent_symbol(&self, (x, y): Coord) -> bool {
        for dy in -1..=1 {
            for dx in -1..=1 {
                if let Some(Place::Symbol(_)) = self.symbols.get(&(x + dx, y + dy)) {
                    return true;
                }
            }
        }

        false
    }

    fn try_get_adjacent_gear(&self, (x, y): Coord) -> Option<Coord> {
        for dy in -1..=1 {
            for dx in -1..=1 {
                if let Some(Place::Symbol('*')) = self.get_place((x + dx, y + dy)) {
                    return Some((x + dx, y + dy));
                }
            }
        }

        None
    }
}

fn parse(lines: &[String]) -> Schematic {
    let width = lines[0].len();
    let height = lines.len();

    let symbols = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, char)| {
                let coord = (x.try_into().unwrap(), y.try_into().unwrap());

                match char {
                    '.' => None,
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Some((
                        coord,
                        Place::Digit(char.to_digit(10).unwrap().try_into().unwrap()),
                    )),
                    _ => Some((coord, Place::Symbol(char))),
                }
            })
        })
        .collect();

    Schematic {
        width: width.try_into().unwrap(),
        height: height.try_into().unwrap(),
        symbols,
    }
}

fn buffer_to_number(buffer: &[u8]) -> u32 {
    buffer.iter().rev().enumerate().fold(0, |acc, (i, d)| {
        acc + u32::from(*d) * 10u32.pow(i.try_into().unwrap())
    })
}

fn solve(schematic: &Schematic) {
    let mut numbers: Vec<u32> = vec![];
    let mut adjacent_to_gear: HashMap<Coord, Vec<u32>> = HashMap::new();

    let mut buffer: Vec<u8> = vec![];
    let mut adjacent_to_symbol = false;
    let mut is_adjacent_to_gear = None;

    let mut clear_buffer = |buffer: &mut Vec<u8>,
                            adjacent_to_symbol: &mut bool,
                            is_adjacent_to_gear: &mut Option<Coord>| {
        if !buffer.is_empty() {
            let number = buffer_to_number(buffer);

            if *adjacent_to_symbol {
                numbers.push(number);
                *adjacent_to_symbol = false;
            }

            if let Some(coord) = is_adjacent_to_gear {
                adjacent_to_gear
                    .entry(*coord)
                    .and_modify(|x| x.push(number))
                    .or_insert_with(|| vec![number]);
                *is_adjacent_to_gear = None;
            }

            buffer.clear();
        }
    };

    for y in 0..schematic.height {
        for x in 0..schematic.width {
            match schematic.get_place((x, y)) {
                Some(Place::Digit(d)) => {
                    buffer.push(*d);

                    if adjacent_to_symbol || schematic.has_adjacent_symbol((x, y)) {
                        adjacent_to_symbol = true;
                    }

                    if let Some(coord) = schematic.try_get_adjacent_gear((x, y)) {
                        is_adjacent_to_gear = Some(coord);
                    }
                }
                _ => {
                    clear_buffer(
                        &mut buffer,
                        &mut adjacent_to_symbol,
                        &mut is_adjacent_to_gear,
                    );
                }
            }
        }

        clear_buffer(
            &mut buffer,
            &mut adjacent_to_symbol,
            &mut is_adjacent_to_gear,
        );
    }

    let part1: u32 = numbers.iter().sum();
    let part2 = adjacent_to_gear.iter().fold(0, |acc, (_, numbers)| {
        if numbers.len() == 2 {
            acc + numbers[0] * numbers[1]
        } else {
            acc
        }
    });

    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn main() {
    let lines = read_input();
    let schematic = parse(&lines);

    solve(&schematic);
}

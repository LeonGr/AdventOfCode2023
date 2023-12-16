use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    MirrorForward,
    MirrorBackward,
    SplitterVertical,
    SplitterHorizontal,
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tile = match s {
            "/" => Tile::MirrorForward,
            "\\" => Tile::MirrorBackward,
            "|" => Tile::SplitterVertical,
            "-" => Tile::SplitterHorizontal,
            _ => unreachable!("Character: '{s}'"),
        };

        Ok(tile)
    }
}

#[derive(Debug)]
struct Map {
    width: i32,
    height: i32,
    tiles: HashMap<Coord, Tile>,
}

fn parse(input: &[String]) -> Map {
    let tiles = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    let c = c.to_string();
                    let c = c.as_str();

                    if let "." = c {
                        None
                    } else {
                        let coord = (x.try_into().unwrap(), y.try_into().unwrap());
                        let tile = Tile::from_str(c).unwrap();

                        Some((coord, tile))
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    Map {
        tiles,
        width: input[0].len().try_into().unwrap(),
        height: input.len().try_into().unwrap(),
    }
}

type Coord = (i32, i32);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Beam {
    position: Coord,
    direction: Direction,
}

impl Beam {
    fn new(position: Coord, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    fn is_stopped(&self, map: &Map) -> bool {
        let (x, y) = self.position;

        x < 0 || y < 0 || x >= map.width || y >= map.height
    }

    fn step(&mut self, map: &Map) -> Option<Tile> {
        let (x, y) = self.position;

        self.position = match self.direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };

        map.tiles.get(&self.position).copied()
    }

    fn reflect(&mut self, tile: Tile) {
        self.direction = match tile {
            Tile::MirrorForward => match self.direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            Tile::MirrorBackward => match self.direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
            Tile::SplitterVertical | Tile::SplitterHorizontal => unreachable!(),
        };
    }
}

fn get_energized(map: &Map, beam: &Beam, energized: &mut HashSet<Beam>) {
    let mut beam = beam.clone();

    //println!("start: {beam:?}");

    while !beam.is_stopped(map) {
        // energized.insert(beam.clone());
        if energized.insert(beam.clone()) {
            //println!("inserting: {beam:?}");
        } else {
            //println!("already seen: {beam:?}");
            break;
        }

        // Check if the beam lands on a tile (not '.')
        if let Some(tile) = beam.step(map) {
            //println!("{beam:?}");
            // energized.insert(beam.clone());
            // if energized.insert(beam.clone()) {
            // //println!("after step, inserting: {beam:?}");
            // } else {
            // //println!("after step, already seen: {beam:?}");
            // break;
            // }

            match tile {
                Tile::MirrorForward | Tile::MirrorBackward => {
                    //println!("reflect");
                    beam.reflect(tile);
                }
                Tile::SplitterVertical => match beam.direction {
                    Direction::Up | Direction::Down => (),
                    Direction::Left | Direction::Right => {
                        //println!("split at {:?}", beam.position);
                        let first_new_beam = Beam::new(beam.position, Direction::Up);
                        let second_new_beam = Beam::new(beam.position, Direction::Down);

                        //println!("first: {first_new_beam:?}, second: {second_new_beam:?}");

                        get_energized(map, &first_new_beam, energized);
                        get_energized(map, &second_new_beam, energized);

                        // energized.extend(first_energized);
                        // energized.extend(second_energized);

                        break;
                    }
                },
                Tile::SplitterHorizontal => match beam.direction {
                    Direction::Left | Direction::Right => (),
                    Direction::Up | Direction::Down => {
                        //println!("split at {:?}", beam.position);
                        let first_new_beam = Beam::new(beam.position, Direction::Left);
                        let second_new_beam = Beam::new(beam.position, Direction::Right);

                        //println!("first: {first_new_beam:?}, second: {second_new_beam:?}");

                        get_energized(map, &first_new_beam, energized);
                        get_energized(map, &second_new_beam, energized);

                        // energized.extend(first_energized);
                        // energized.extend(second_energized);

                        break;
                    }
                },
            }
        } else {
            //println!("{beam:?}");
        }
    }

    //println!("end beam");
}

fn part1(map: &Map) -> usize {
    let start_beam = Beam::new((0, 0), Direction::Right);
    //println!("map: {map:?}");

    let mut energized = HashSet::new();
    get_energized(map, &start_beam, &mut energized);
    energized
        .iter()
        .map(|beam| beam.position)
        .collect::<HashSet<_>>()
        .len()
}

fn part2(map: &Map) -> usize {
    let mut start_beams = vec![];

    for x in 0..map.width {
        start_beams.push(Beam::new((x, 0), Direction::Down));
        start_beams.push(Beam::new((x, map.height - 1), Direction::Up));
    }

    for y in 0..map.height {
        start_beams.push(Beam::new((0, y), Direction::Right));
        start_beams.push(Beam::new((map.width - 1, y), Direction::Left));
    }

    start_beams
        .iter()
        .map(|start_beam| {
            let mut energized = HashSet::new();
            get_energized(map, start_beam, &mut energized);
            energized
                .iter()
                .map(|beam| beam.position)
                .collect::<HashSet<_>>()
                .len()
        })
        .max()
        .unwrap()
}

fn main() {
    let input = read_input();
    let parsed = parse(&input);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}

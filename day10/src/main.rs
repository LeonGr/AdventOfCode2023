use std::{collections::{HashSet, VecDeque}, str::FromStr};

fn read_input() -> String {
    let input = include_str!("../input");
    input.to_string()
}

#[derive(Clone, Copy)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    BendNorthEastPipe,
    BendNorthWestPipe,
    BendSouthWestPipe,
    BendSouthEastPipe,
    Ground,
    Start,
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tile = match s {
            "|" => Tile::VerticalPipe,
            "-" => Tile::HorizontalPipe,
            "L" => Tile::BendNorthEastPipe,
            "J" => Tile::BendNorthWestPipe,
            "7" => Tile::BendSouthWestPipe,
            "F" => Tile::BendSouthEastPipe,
            "." => Tile::Ground,
            "S" => Tile::Start,
            _ => unreachable!("Character: '{s}'"),
        };

        Ok(tile)
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| Tile::from_str(&char.to_string()).unwrap())
                    .collect()
            })
            .collect();

        Ok(Map { tiles })
    }
}

fn parse(input: &str) -> Map {
    Map::from_str(input).unwrap()
}

type Coord = (i32, i32);

impl Tile {
    fn displacements(self) -> Vec<(i32, i32)> {
        let north = (0, -1);
        let east = (1, 0);
        let south = (0, 1);
        let west = (-1, 0);

        match self {
            Tile::VerticalPipe => vec![north, south],
            Tile::HorizontalPipe => vec![east, west],
            Tile::BendNorthEastPipe => vec![north, east],
            Tile::BendNorthWestPipe => vec![north, west],
            Tile::BendSouthWestPipe => vec![south, west],
            Tile::BendSouthEastPipe => vec![south, east],
            Tile::Start => vec![north, east, south, west],
            Tile::Ground => panic!("Ground doesn't go anywhere"),
        }
    }
}

impl Map {
    fn start(&self) -> Coord {
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[0].len() {
                if let Tile::Start = self.tiles[y][x] {
                    return (x.try_into().unwrap(), y.try_into().unwrap());
                }
            }
        }

        unreachable!("There has to be a start");
    }

    fn get(&self, (x, y): Coord) -> Option<Tile> {
        if x < 0 || y < 0 {
            return None;
        }

        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();

        self.tiles.get(y).and_then(|row| row.get(x)).copied()
    }

    // Find neighbours of a coord which are connected to the pipe at the coord.
    fn connected(&self, coord @ (x, y): Coord) -> Vec<(Coord, Tile)> {
        let tile = self.get(coord);
        let tile = tile.expect("Non-existent tile is not connected to anything");

        let displacements = tile.displacements();

        if let Tile::Start = tile {
            displacements
                .iter()
                .filter_map(|(dx, dy)| {
                    let new_coord = (x + dx, y + dy);
                    let neighbour = self.get(new_coord);

                    // If the neighbour of a neighbour of start is start, it is connected to start.
                    match neighbour {
                        Some(Tile::Ground) | None => None,
                        Some(neighbour) => {
                            let is_connected_to_start = self
                                .connected(new_coord)
                                .iter()
                                .any(|(_, tile)| matches!(tile, Tile::Start));

                            if is_connected_to_start {
                                Some((new_coord, neighbour))
                            } else {
                                None
                            }
                        }
                    }
                })
                .collect()
        } else {
            displacements
                .iter()
                .map(|(dx, dy)| {
                    let new_coord = (x + dx, y + dy);

                    let neighbour = self.get(new_coord);

                    (
                        new_coord,
                        neighbour
                            .expect("All neighbours of a non-start pipe should be another pipe"),
                    )
                })
                .collect()
        }
    }

    fn next_in_loop(&self, coord: Coord, previous: Coord) -> (Coord, Tile) {
        let connected = self.connected(coord);
        let (first @ (first_coord, _), second) = (connected[0], connected[1]);

        if first_coord == previous {
            second
        } else {
            first
        }
    }

    fn big_main_loop(&self) -> HashSet<Coord> {
        let start_coord = self.start();
        let mut main_loop = HashSet::new();
        main_loop.insert(start_coord);

        let connected = self.connected(start_coord);
        let ((mut first_coord, _), (mut second_coord, _)) =
            (connected[0], connected[1]);

        let mut previous_first = start_coord;
        let mut previous_second = start_coord;

        while first_coord != second_coord {
            main_loop.insert(first_coord);
            main_loop.insert(second_coord);

            let new_first = self.next_in_loop(first_coord, previous_first);
            previous_first = first_coord;
            (first_coord, _) = new_first;
            let new_second = self.next_in_loop(second_coord, previous_second);
            previous_second = second_coord;
            (second_coord, _) = new_second;
        }

        main_loop.insert(first_coord);

        let mut big_main_loop = HashSet::new();

        for &(x, y) in &main_loop {
            match self.get((x, y)) {
                Some(tile) => {
                    // middle
                    big_main_loop.insert((x * 3 + 1, y * 3 + 1));

                    if let Tile::Start = tile {
                        let connected_to_start = self.connected((x, y));

                        for &((nx, ny), _) in &connected_to_start {
                            let (dx, dy) = (nx - x, ny - y);

                            big_main_loop.insert((x * 3 + 1 + dx, y * 3 + 1 + dy));
                        }
                    } else {
                        let displacements = tile.displacements();

                        for &(dx, dy) in &displacements {
                            big_main_loop.insert((x * 3 + 1 + dx, y * 3 + 1 + dy));
                        }
                    }
                }
                None => unreachable!("Tiles in main loop should exist."),
            }
            }

        big_main_loop
    }
}

fn part1(map: &Map) -> u32 {
    let start_coord = map.start();

    let connected = map.connected(start_coord);
    let ((mut first_coord, _), (mut second_coord, _)) =
        (connected[0], connected[1]);

    let mut previous_first = start_coord;
    let mut previous_second = start_coord;

    let mut distance = 1;
    while first_coord != second_coord {
        distance += 1;

        let new_first = map.next_in_loop(first_coord, previous_first);
        previous_first = first_coord;
        (first_coord, _) = new_first;
        let new_second = map.next_in_loop(second_coord, previous_second);
        previous_second = second_coord;
        (second_coord, _) = new_second;
    }

    distance
}

fn part2(map: &Map) -> usize {
    let big_main_loop = map.big_main_loop();

    let mut outside = HashSet::new();
    let mut checked = HashSet::new();

    let mut queue = VecDeque::from([(0, 0)]);

    let displacements = [ (0, 1), (0, -1), (1, 0), (-1, 0) ];
    let x_max = map.tiles[0].len();
    let x_max: i32 = (x_max * 3 - 1).try_into().unwrap();
    let y_max = map.tiles.len();
    let y_max: i32 = (y_max * 3 - 1).try_into().unwrap();

    // BFS for all outside tiles
    while !queue.is_empty() {
        let n @ (x, y) = queue.pop_front().unwrap();

        if checked.contains(&n) {
            continue;
        }

        checked.insert(n);

        if !big_main_loop.contains(&n) {
            outside.insert(n);

            for (dx, dy) in displacements {
                let (nx, ny) = (x + dx, y + dy);

                if nx < 0 || ny < 0 || nx > x_max || ny > y_max {
                    continue;
                }

                queue.push_back((nx, ny));
            }
        }
    }

    let inside: HashSet<_> =
        (0..=x_max).flat_map(|x| {
            (0..=y_max).filter_map(|y| {
                let coord = (x, y);
                if !outside.contains(&coord) && !big_main_loop.contains(&coord) {
                    Some(coord)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
        })
        .collect();

    inside.iter()
        .filter(|&(x, y)| {
            let displacements = [ (0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (-1, -1), (-1, 1), (1, -1) ];
            !displacements.iter().any(|&(dx, dy)| {
                let new = (x + dx, y + dy);
                big_main_loop.contains(&new)
            })
        })
        .count() / 9
}

fn main() {
    let input = read_input();
    let parsed = parse(&input);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}

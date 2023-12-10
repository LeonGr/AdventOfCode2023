use colored::Colorize;
use core::fmt;
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

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tile_character = match self {
            Tile::VerticalPipe => "│",
            Tile::HorizontalPipe => "─",
            Tile::BendNorthEastPipe => "└",
            Tile::BendNorthWestPipe => "┘",
            Tile::BendSouthWestPipe => "┐",
            Tile::BendSouthEastPipe => "┌",
            Tile::Ground => ".",
            Tile::Start => "S",
        };

        write!(f, "{tile_character}")
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
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

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        self.tiles.iter().for_each(|tile_line| {
            tile_line
                .iter()
                .for_each(|tile| output += &format!("{tile}"));

            output += "\n";
        });

        write!(f, "{output}")
    }
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
            Tile::Ground => panic!("Ground doesn't go anywhere"),
            Tile::Start => vec![north, east, south, west],
        }
    }

    fn to_big_string(self) -> (String, String, String) {
        match self {
            Tile::VerticalPipe => {
                (
                    String::from(".│."),
                    String::from(".│."),
                    String::from(".│."),
                )
            },
            Tile::HorizontalPipe => {
                (
                    String::from("..."),
                    String::from("───"),
                    String::from("..."),
                )
            }
            Tile::BendNorthEastPipe => {
                (
                    String::from(".│."),
                    String::from(".└─"),
                    String::from("..."),
                )
            }
            Tile::BendNorthWestPipe => {
                (
                    String::from(".│."),
                    String::from("─┘."),
                    String::from("..."),
                )
            }
            Tile::BendSouthWestPipe => {
                (
                    String::from("..."),
                    String::from("─┐."),
                    String::from(".│."),
                )
            }
            Tile::BendSouthEastPipe => {
                (
                    String::from("..."),
                    String::from(".┌─"),
                    String::from(".│."),
                )
            }
            Tile::Ground => {
                (
                    String::from("..."),
                    String::from("..."),
                    String::from("..."),
                )
            },
            Tile::Start => {
                (
                    String::from(".S."),
                    String::from("SSS"),
                    String::from(".S."),
                )
            }
        }
    }

    fn to_big(&self) -> (Vec<Tile>, Vec<Tile>, Vec<Tile>) {
        match self {
            Tile::VerticalPipe => {
                (
                    vec![Tile::Ground, Tile::VerticalPipe, Tile::Ground],
                    vec![Tile::Ground, Tile::VerticalPipe, Tile::Ground],
                    vec![Tile::Ground, Tile::VerticalPipe, Tile::Ground],
                )
            },
            Tile::HorizontalPipe => {
                (
                    vec![Tile::Ground, Tile::Ground, Tile::Ground],
                    vec![Tile::HorizontalPipe, Tile::HorizontalPipe, Tile::HorizontalPipe],
                    vec![Tile::Ground, Tile::Ground, Tile::Ground],
                )
            },
            Tile::BendNorthEastPipe => {
                (
                    vec![Tile::Ground, Tile::VerticalPipe, Tile::Ground],
                    vec![Tile::Ground, Tile::BendNorthEastPipe, Tile::HorizontalPipe],
                    vec![Tile::Ground, Tile::Ground, Tile::Ground],
                )
            },
            Tile::BendNorthWestPipe => {
                (
                    vec![Tile::Ground, Tile::VerticalPipe, Tile::Ground],
                    vec![Tile::HorizontalPipe, Tile::BendNorthWestPipe, Tile::Ground],
                    vec![Tile::Ground, Tile::Ground, Tile::Ground],
                )
            },
            Tile::BendSouthWestPipe => {
                (
                    vec![Tile::Ground, Tile::Ground, Tile::Ground],
                    vec![Tile::HorizontalPipe, Tile::BendSouthWestPipe, Tile::Ground],
                    vec![Tile::Ground, Tile::VerticalPipe, Tile::Ground],
                )
            },
            Tile::BendSouthEastPipe => {
                (
                    vec![Tile::Ground, Tile::Ground, Tile::Ground],
                    vec![Tile::Ground, Tile::BendSouthEastPipe, Tile::HorizontalPipe],
                    vec![Tile::Ground, Tile::VerticalPipe, Tile::Ground],
                )
            },
            Tile::Ground => {
                (
                    vec![Tile::Ground, Tile::Ground, Tile::Ground],
                    vec![Tile::Ground, Tile::Ground, Tile::Ground],
                    vec![Tile::Ground, Tile::Ground, Tile::Ground],
                )
            },
            Tile::Start => {
                (
                    vec![Tile::Ground, Tile::Ground, Tile::Ground],
                    vec![Tile::Ground, Tile::Start, Tile::Ground],
                    vec![Tile::Ground, Tile::Ground, Tile::Ground],
                )
            },
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

        // if let Tile::Ground = tile {
        // panic!("Ground has no connected neighbours")
        // }
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

    fn next(&self, coord: Coord, previous: Coord) -> (Coord, Tile) {
        let connected = self.connected(coord);
        let (first @ (first_coord, _), second) = (connected[0], connected[1]);

        if first_coord == previous {
            second
        } else {
            first
        }
    }

    fn print(&self) {
        let start_coord = self.start();
        // let mut main_loop = vec![start_coord];
        let mut main_loop = HashSet::new();
        main_loop.insert(start_coord);

        let connected = self.connected(start_coord);
        let (mut first @ (mut first_coord, _), mut second @ (mut second_coord, _)) =
            (connected[0], connected[1]);

        let mut previous_first = start_coord;
        let mut previous_second = start_coord;

        while first_coord != second_coord {
            // main_loop.push(first_coord);
            // main_loop.push(second_coord);
            main_loop.insert(first_coord);
            main_loop.insert(second_coord);

            first = self.next(first_coord, previous_first);
            previous_first = first_coord;
            (first_coord, _) = first;
            second = self.next(second_coord, previous_second);
            previous_second = second_coord;
            (second_coord, _) = second;
        }

        // main_loop.push(first_coord);
        main_loop.insert(first_coord);

        let mut output = String::new();
        self.tiles.iter().enumerate().for_each(|(y, tile_line)| {
            tile_line.iter().enumerate().for_each(|(x, tile)| {
                if main_loop.contains(&(x.try_into().unwrap(), y.try_into().unwrap())) {
                    output += &format!("{}", tile.to_string().blue());
                } else {
                    output += &format!("{tile}");
                }
            });

            output += "\n";
        });

        println!("{output}");
    }

    fn print_big(&self, outside: &HashSet<Coord>, inside: &HashSet<Coord>, big_main_loop: &HashSet<Coord>) {
        let start_coord = self.start();
        // let mut main_loop = vec![start_coord];
        // let mut main_loop = HashSet::new();
        // main_loop.insert(start_coord);

        // let connected = self.connected(start_coord);
        // let (mut first @ (mut first_coord, _), mut second @ (mut second_coord, _)) =
            // (connected[0], connected[1]);

        // let mut previous_first = start_coord;
        // let mut previous_second = start_coord;

        // while first_coord != second_coord {
            // // main_loop.push(first_coord);
            // // main_loop.push(second_coord);
            // main_loop.insert(first_coord);
            // main_loop.insert(second_coord);

            // first = self.next(first_coord, previous_first);
            // previous_first = first_coord;
            // (first_coord, _) = first;
            // second = self.next(second_coord, previous_second);
            // previous_second = second_coord;
            // (second_coord, _) = second;
        // }

        // // main_loop.push(first_coord);
        // main_loop.insert(first_coord);

        // let mut output = String::new();
        // let mut buffer0 = String::new();
        // let mut buffer1 = String::new();
        // let mut buffer2 = String::new();

        // self.tiles.iter().enumerate()
            // .for_each(|(y, tile_line)| {
                // tile_line.iter().enumerate()
                    // .for_each(|(x, tile)| {
                        // let (part0, part1, part2) = tile.to_big_string();

                        // if main_loop.contains(&(x.try_into().unwrap(), y.try_into().unwrap())) {
                            // buffer0 += &format!("{}", part0.blue());
                            // buffer1 += &format!("{}", part1.blue());
                            // buffer2 += &format!("{}", part2.blue());
                        // } else {
                            // buffer0 += &format!("{}", part0);
                            // buffer1 += &format!("{}", part1);
                            // buffer2 += &format!("{}", part2);
                        // }
                    // });

                // output += &buffer0;
                // output += "\n";
                // output += &buffer1;
                // output += "\n";
                // output += &buffer2;
                // output += "\n";

                // buffer0 = String::new();
                // buffer1 = String::new();
                // buffer2 = String::new();
            // });

        let mut big_tiles = vec![];
        let mut buffer0 = vec![];
        let mut buffer1 = vec![];
        let mut buffer2 = vec![];

        self.tiles.iter().enumerate()
            .for_each(|(y, tile_line)| {
                tile_line.iter().enumerate()
                    .for_each(|(x, tile)| {
                        let (part0, part1, part2) = tile.to_big();

                        part0.into_iter().for_each(|p| buffer0.push(p));
                        part1.into_iter().for_each(|p| buffer1.push(p));
                        part2.into_iter().for_each(|p| buffer2.push(p));
                    });

                big_tiles.push(buffer0.clone());
                big_tiles.push(buffer1.clone());
                big_tiles.push(buffer2.clone());

                buffer0.clear();
                buffer1.clear();
                buffer2.clear();
            });


        let mut output = String::new();

        big_tiles.iter().enumerate()
            .for_each(|(y, tile_line)| {
                tile_line.iter().enumerate()
                    .for_each(|(x, tile)| {
                        let coord = (x.try_into().unwrap(), y.try_into().unwrap());
                        if big_main_loop.contains(&coord) {
                            output += &format!("{}", tile.to_string().blue());
                        } else if outside.contains(&coord){
                            output += &format!("{}", tile.to_string().red());
                        } else if inside.contains(&coord){
                            output += &format!("{}", tile.to_string().yellow());
                        } else {
                            output += &format!("{tile}");
                        }
                    });

                output += "\n";
            });

        println!("{output}");
    }

    fn big_main_loop(&self) -> HashSet<Coord> {
        let start_coord = self.start();
        let mut main_loop = HashSet::new();
        main_loop.insert(start_coord);

        let connected = self.connected(start_coord);
        let (mut first @ (mut first_coord, _), mut second @ (mut second_coord, _)) =
            (connected[0], connected[1]);

        let mut previous_first = start_coord;
        let mut previous_second = start_coord;

        while first_coord != second_coord {
            main_loop.insert(first_coord);
            main_loop.insert(second_coord);

            first = self.next(first_coord, previous_first);
            previous_first = first_coord;
            (first_coord, _) = first;
            second = self.next(second_coord, previous_second);
            previous_second = second_coord;
            (second_coord, _) = second;
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

        let mut big_tiles = vec![];
        let mut buffer0 = vec![];
        let mut buffer1 = vec![];
        let mut buffer2 = vec![];

        self.tiles.iter().enumerate()
            .for_each(|(y, tile_line)| {
                tile_line.iter().enumerate()
                    .for_each(|(x, tile)| {
                        let (part0, part1, part2) = tile.to_big();

                        part0.into_iter().for_each(|p| buffer0.push(p));
                        part1.into_iter().for_each(|p| buffer1.push(p));
                        part2.into_iter().for_each(|p| buffer2.push(p));

                        if main_loop.contains(&(x.try_into().unwrap(), y.try_into().unwrap())) {

                        }
                    });

                big_tiles.push(buffer0.clone());
                big_tiles.push(buffer1.clone());
                big_tiles.push(buffer2.clone());

                buffer0.clear();
                buffer1.clear();
                buffer2.clear();
            });

        big_main_loop
    }
}

fn part1(map: &Map) -> u32 {
    let start_coord = map.start();
    println!("start: {start_coord:?}");

    println!("connected to start: {:?}", map.connected(start_coord));

    let connected = map.connected(start_coord);
    let (mut first @ (mut first_coord, _), mut second @ (mut second_coord, _)) =
        (connected[0], connected[1]);

    let mut previous_first = start_coord;
    let mut previous_second = start_coord;

    let mut distance = 1;
    while first_coord != second_coord {
        distance += 1;

        first = map.next(first_coord, previous_first);
        previous_first = first_coord;
        (first_coord, _) = first;
        second = map.next(second_coord, previous_second);
        previous_second = second_coord;
        (second_coord, _) = second;
    }

    distance
}

// fn is_part_of_main_loop(map: &Map, coord: Coord) -> bool {

    // todo!()
// }

fn part2(map: &Map) -> usize {
    // map.print_big();
    let big_main_loop = map.big_main_loop();
    println!("big_main_loop: {:?}", big_main_loop);

    // TODO: BFS from (0, 0) (or all wall pixels) to count outside part.
    let mut outside = HashSet::new();
    let mut checked = HashSet::new();

    let mut queue = VecDeque::from([(0, 0)]);

    let displacements = [ (0, 1), (0, -1), (1, 0), (-1, 0) ];
    let x_max = map.tiles[0].len();
    let x_max = (x_max * 3 - 1) as i32;
    let y_max = map.tiles.len();
    let y_max = (y_max * 3 - 1) as i32;
    println!("x_max: {}, y_max: {}", x_max, y_max);

    while !queue.is_empty() {
        let n @ (x, y) = queue.pop_front().unwrap();

        println!("checking: {n:?}");

        if checked.contains(&n) {
            continue;
        }

        checked.insert(n);

        if !big_main_loop.contains(&n) {
            outside.insert(n);

            for (dx, dy) in displacements {
                let (nx, ny) = (x + dx, y + dy);

                if nx < 0 || ny < 0 || nx > x_max || ny > y_max {
                    println!("outside of scope: {:?}", (nx, ny));
                    continue;
                }

                queue.push_back((nx, ny));
            }
        }
    }

    println!("outside: {outside:?}");

    let total_tiles_big_map = map.tiles.len() * map.tiles[0].len() * 9;
    println!("total_tiles_big_map: {total_tiles_big_map}");
    println!("big_main_loop: {}", big_main_loop.len());
    println!("outside: {}", outside.len());
    // total_tiles_big_map - big_main_loop.len() - outside.len();

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

    println!("inside: {inside:?}");

    map.print_big(&outside, &inside, &big_main_loop);

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

    println!("{parsed}");

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}

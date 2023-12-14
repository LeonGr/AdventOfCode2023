use std::fmt::Debug;

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}


#[derive(Clone, Copy, PartialEq)]
enum Tile {
    RoundRock,
    CubeRock,
    Empty,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RoundRock => write!(f, "O"),
            Self::CubeRock => write!(f, "#"),
            Self::Empty => write!(f, "."),
        }
    }
}

type Row = Vec<Tile>;

type Map = Vec<Row>;

fn parse(lines: &[String]) -> Map {
    lines.iter()
        .map(|line| {
            line.chars()
                .map(|c| {
                    match c {
                        '#' => Tile::CubeRock,
                        'O' => Tile::RoundRock,
                        '.' => Tile::Empty,
                        _ => unreachable!()
                    }
                })
                .collect()
        })
        .collect()
}


fn roll_single_up(map: &mut Map, row: usize, col: usize) {
    let mut new_row = row;
    map[row][col] = Tile::Empty;

    for y in (0..row).rev() {
        match &map[y][col] {
            Tile::RoundRock | Tile::CubeRock => {
                break;
            },
            Tile::Empty => {
                new_row -= 1;
            },
        }
    }

    map[new_row][col] = Tile::RoundRock;
}

fn roll_row_up(map: &mut Map, row: usize) {
    let row_tiles = &map[row].clone();

    for (col, rock) in row_tiles.iter().enumerate() {
        if matches!(rock, Tile::RoundRock) {
            roll_single_up(map, row, col);
        }
    }
}

fn roll_all_up(map: &mut Map) {
    for i in 0..map.len() {
        roll_row_up(map, i);
    }
}


fn map_to_string(map: &Map) -> String {
    let mut output = String::new();

    for row in map {
        for tile in row {
            output +=
                match tile {
                    Tile::RoundRock => "O",
                    Tile::CubeRock => "#",
                    Tile::Empty => ".",
                };
        }

        output += "\n";
    }

    output
}

fn calculate_load(map: &Map) -> usize {
    map.iter().rev().enumerate()
        .map(|(i, row)| {
            let row_weight = i + 1;
            let round_rock_count = row.iter().filter(|tile| matches!(tile, Tile::RoundRock)).count();

            row_weight * round_rock_count
        })
        .sum()
}

fn part1(map: &Map) -> usize {
    let mut map = map.clone(); 

    roll_all_up(&mut map);

    calculate_load(&map)
}

fn rotate(map: &mut Map) {
    let rows = map.len();

    // Transpose
    let mut copy = map.clone();

    for i in 0..map.len() {
        for j in 0..i {
            let temp = copy[i][j];
            copy[i][j] = copy[j][i];
            copy[j][i] = temp;
        }
    }

    *map = copy;

    // Reverse each row
    (0..rows).for_each(|i| {
        map[i].reverse();
    });
}

fn cycle(map: &mut Map) {
    roll_all_up(map);
    rotate(map);
    roll_all_up(map);
    rotate(map);
    roll_all_up(map);
    rotate(map);
    roll_all_up(map);
    rotate(map);
}

fn part2(map: &Map) -> usize {
    let mut map = map.clone(); 

    let mut after_cycles = vec![];

    let mut diff = 0;
    let mut start = 0;
    let mut c = 1;
    'outer: loop {
        cycle(&mut map);

        for (i, after_cycle) in after_cycles.iter().enumerate() {
            if *after_cycle == map {
                diff = c - (i+1);
                start = i + 1;
                break 'outer;
            }
        }

        after_cycles.push(map.clone());
        c += 1;
    }

    let need_cycle = (1_000_000_000 - start) % diff + start;

    calculate_load(&after_cycles[need_cycle - 1])
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}

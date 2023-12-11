use itertools::Itertools;
use std::collections::HashSet;

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

type Coord = (usize, usize);

struct Picture {
    galaxies: HashSet<Coord>,
    width: usize,
    height: usize,
}

fn parse(lines: &[String]) -> Picture {
    let width = lines[0].len();
    let height = lines.len();

    let galaxies =
        lines.iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, char)| {
                        match char {
                            '#' => Some((x, y)),
                            '.' => None,
                            _ => unreachable!()
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

    Picture { galaxies, width, height }
}


fn expanded(picture: &Picture, expansion: usize) -> Picture {
    let expansion = expansion - 1; // Subtract existing empty row/column

    let mut preceding_empty_cols_before = vec![];

    let mut empty_cols_before = 0;

    // Count empty columns before a certain column
    for x in 0..picture.width {
        let mut is_empty = true;

        for y in 0..picture.height {
            if picture.galaxies.contains(&(x, y)) {
                is_empty = false;
                break;
            }
        }

        if is_empty {
            empty_cols_before += expansion;
        }

        preceding_empty_cols_before.push(empty_cols_before);
    }

    let mut preceding_empty_rows_before = vec![];

    let mut empty_rows_before = 0;

    // Count empty rows before a certain row
    for y in 0..picture.height {
        let mut is_empty = true;

        for x in 0..picture.width {
            if picture.galaxies.contains(&(x, y)) {
                is_empty = false;
                break;
            }
        }

        if is_empty {
            empty_rows_before += expansion;
        }

        preceding_empty_rows_before.push(empty_rows_before);
    }

    let galaxies: HashSet<Coord> = picture.galaxies.iter()
        .map(|&(x, y)| {
            (x + preceding_empty_cols_before[x], y + preceding_empty_rows_before[y])
        })
        .collect();

    let (width, height) = galaxies.iter()
        .fold((0, 0), |(max_x, max_y), &(x, y)| {
            (max_x.max(x), max_y.max(y))
        });
    let width = width + 1;
    let height = height + 1;

    Picture { galaxies, width, height }
}

fn part1(picture: &Picture) -> usize {
    let picture: Picture = expanded(picture, 1);

    picture.galaxies.iter()
        .tuple_combinations::<(_, _)>()
        .map(|(&(x0, y0), &(x1, y1))| {
            x0.abs_diff(x1) + y0.abs_diff(y1)
        })
        .sum()
}

fn part2(picture: &Picture) -> usize {
    let picture: Picture = expanded(picture, 1_000_000);

    picture.galaxies.iter()
        .tuple_combinations::<(_, _)>()
        .map(|(&(x0, y0), &(x1, y1))| {
            x0.abs_diff(x1) + y0.abs_diff(y1)
        })
        .sum()
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}

use std::{collections::{HashSet, VecDeque}, str::FromStr};

fn read_input() -> String {
    let input = include_str!("../input");
    input.to_string()
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Ash,
    Rocks,
}

type Pattern = Vec<Vec<Tile>>;

fn parse(input: &str) -> Vec<Pattern> {
    input.split("\n\n")
        .map(|pattern| {
            pattern.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| {
                            match c {
                                '#' => Tile::Rocks,
                                '.' => Tile::Ash,
                                _ => unreachable!()
                            }
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn row_to_string(row: &Vec<Tile>) -> String {
    let mut output = String::new();
    for tile in row {
        match tile {
            Tile::Ash => output += ".",
            Tile::Rocks => output += "#",
        }
    }

    output
}


fn is_perfect_reflection_rows(pattern: &Pattern, row: usize) -> bool {
    let mut first_index = row - 1;
    let mut second_index = row;
    let mut first = pattern.get(first_index);
    let mut second = pattern.get(second_index);

    let width = pattern[0].len();

    while first.is_some() && second.is_some() {
        let first_tiles = first.unwrap();
        let second_tiles = second.unwrap();

        // println!("Comparing:");
        // println!("{:?}", row_to_string(first_tiles));
        // println!("{:?}", row_to_string(second_tiles));

        for x in 0..width {
            let first_tile = &first_tiles[x];
            let second_tile = &second_tiles[x];

            if *first_tile != *second_tile {
                return false;
            }
        }

        if first_index == 0 {
            break;
        }

        first_index -= 1;
        second_index += 1;
        first = pattern.get(first_index);
        second = pattern.get(second_index);
    }

    true
}

fn col_to_string(pattern: &Pattern, col: usize) -> String {
    let mut output = String::new();

    let height = pattern.len();
    for y in 0..height {
        let tile = &pattern[y][col];

        match tile {
            Tile::Ash => output += ".",
            Tile::Rocks => output += "#",
        }
    }

    output
}

fn is_perfect_reflection_cols(pattern: &Pattern, col: usize) -> bool {
    let height = pattern.len();
    let width = pattern[0].len();

    let mut first_index = col - 1;
    let mut second_index = col;

    while second_index < width {
        // println!("Comparing:");
        // println!("{:?}", col_to_string(pattern, first_index));
        // println!("{:?}", col_to_string(pattern, second_index));

        for y in 0..height {
            let first_tile = &pattern[y][first_index];
            let second_tile = &pattern[y][second_index];

            if *first_tile != *second_tile {
                // println!("{:?} != {:?}", first_tile, second_tile);
                return false;
            }
        }

        if first_index == 0 {
            break;
        }

        first_index -= 1;
        second_index += 1;
    }

    true
}

fn find_reflection_lines_before(pattern: &Pattern) -> (Option<usize>, Option<usize>) {
    // println!("Pattern: '{pattern:?}'");
    let width = pattern[0].len();
    let height = pattern.len();

    // println!("width {width}, height: {height}, total: {}", width * height);

    'outer: for y in 1..height {
        let first = &pattern[y - 1];
        let second = &pattern[y];

        for x in 0..width {
            let first_tile = &first[x];
            let second_tile = &second[x];

            if *first_tile != *second_tile {
                continue 'outer;
            }
        }

        // println!("Found reflection in row {}", y);
        if is_perfect_reflection_rows(pattern, y) {
            // println!("Found perfect reflection in row {}", y);
            return (None, Some(y));
        }
    }

    'outer: for x in 1..width {
        for y in 0..height {
            let first_tile = &pattern[y][x - 1];
            let second_tile = &pattern[y][x];

            if *first_tile != *second_tile {
                continue 'outer;
            }
        }

        // println!("Found reflection in col {}", x);
        if is_perfect_reflection_cols(pattern, x) {
            // println!("Found perfect reflection in col {}", x);
            return (Some(x), None);
        }
    }

    (None, None)
}

fn part1(patterns: &[Pattern]) -> usize {
    patterns.iter().enumerate()
        .map(|(i, pattern)| {
            match find_reflection_lines_before(pattern) {
                (Some(columns_before), None) => columns_before,
                (None, Some(rows_before)) => 100 * rows_before,
                other => unreachable!("Other: {other:?}")
            }
        })
        .sum()
}

fn find_new_reflection_lines_before(pattern: &Pattern, old: (Option<usize>, Option<usize>)) -> (Option<usize>, Option<usize>) {
    // println!("Pattern: '{pattern:?}'");
    let width = pattern[0].len();
    let height = pattern.len();

    // println!("width {width}, height: {height}, total: {}", width * height);

    'outer: for y in 1..height {
        let first = &pattern[y - 1];
        let second = &pattern[y];

        for x in 0..width {
            let first_tile = &first[x];
            let second_tile = &second[x];

            if *first_tile != *second_tile {
                continue 'outer;
            }
        }

        // println!("Found reflection in row {}", y);
        if is_perfect_reflection_rows(pattern, y) {
            // println!("Found perfect reflection in row {}", y);
            let result = (None, Some(y));
            if result != old {
                return result;
            }
        }
    }

    'outer: for x in 1..width {
        for y in 0..height {
            let first_tile = &pattern[y][x - 1];
            let second_tile = &pattern[y][x];

            if *first_tile != *second_tile {
                continue 'outer;
            }
        }

        // println!("Found reflection in col {}", x);
        if is_perfect_reflection_cols(pattern, x) {
            // println!("Found perfect reflection in col {}", x);
            let result = (Some(x), None);
            if result != old {
                return result;
            }
        }
    }

    (None, None)
}

fn find_altered_reflection_lines_before(pattern: &Pattern) -> (Option<usize>, Option<usize>) {
    println!("Pattern: '{pattern:?}'");
    let width = pattern[0].len();
    let height = pattern.len();

    println!("width {width}, height: {height}, total: {}", width * height);

    let original = find_reflection_lines_before(pattern);
    println!("original {:?}", original);

    let mut pattern = pattern.clone();

    for x in 0..width {
        for y in 0..height {
            let tile = pattern[y][x];
            let flipped = flip(&tile);

            pattern[y][x] = flipped;
            let new = find_new_reflection_lines_before(&pattern, original);
            match new {
                (None, Some(r)) if new != original => {
                    println!("Different: {new:?}");
                    return new;
                },
                (Some(c), None) if new != original => {
                    println!("Different: {new:?}");
                    return new;
                },
                (Some(c), Some(r)) => {
                    println!("both");
                }
                _ => (),
            }
            pattern[y][x] = tile;
        }
    }

    todo!()
}

fn flip(tile: &Tile) -> Tile {
    match tile {
        Tile::Ash => Tile::Rocks,
        Tile::Rocks => Tile::Ash,
    }
}

fn part2(patterns: &[Pattern]) -> usize {
    patterns.into_iter().enumerate()
        .map(|(i, pattern)| {
            println!("i: {i}");
            // let mut pattern = *pattern;
            match find_altered_reflection_lines_before(pattern) {
                (Some(columns_before), None) => columns_before,
                (None, Some(rows_before)) => 100 * rows_before,
                other => unreachable!("Other: {other:?}")
            }
        })
        .sum()
}

fn main() {
    let input = read_input();
    let parsed = parse(&input);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
